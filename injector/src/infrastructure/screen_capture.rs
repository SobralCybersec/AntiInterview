use anyhow::Result;
use crossbeam_channel::{Receiver, Sender};
use eframe::egui::ColorImage;
use windows_capture::{
    capture::{CaptureControl, Context, GraphicsCaptureApiHandler},
    frame::Frame,
    monitor::Monitor,
    settings::{
        ColorFormat, CursorCaptureSettings, DirtyRegionSettings, DrawBorderSettings,
        MinimumUpdateIntervalSettings, SecondaryWindowSettings, Settings,
    },
};

pub struct ScreenCaptureService {
    capture_send: Sender<ColorImage>,
}

impl GraphicsCaptureApiHandler for ScreenCaptureService {
    type Flags = Sender<ColorImage>;
    type Error = anyhow::Error;

    fn new(ctx: Context<Self::Flags>) -> Result<Self> {
        Ok(Self {
            capture_send: ctx.flags,
        })
    }

    fn on_frame_arrived(
        &mut self,
        frame: &mut Frame,
        _capture_control: windows_capture::graphics_capture_api::InternalCaptureControl,
    ) -> Result<()> {
        if self.capture_send.is_full() {
            return Ok(());
        }

        let width = frame.width();
        let height = frame.height();

        if let Ok(buffer) = frame.buffer() {
            let mut no_pad_buffer = Vec::new();
            let no_pad_buffer = buffer.as_nopadding_buffer(&mut no_pad_buffer);
            let img = ColorImage::from_rgba_unmultiplied(
                [width as usize, height as usize],
                no_pad_buffer,
            );
            let _ = self.capture_send.try_send(img);
        }

        Ok(())
    }
}

pub enum CaptureCommand {
    Start(Monitor),
    Stop,
}

pub struct CaptureManager {
    command_tx: Sender<CaptureCommand>,
    image_rx: Receiver<ColorImage>,
}

impl CaptureManager {
    pub fn new() -> Self {
        let (image_tx, image_rx) = crossbeam_channel::bounded(2);
        let (command_tx, command_rx) = crossbeam_channel::unbounded();

        std::thread::spawn(move || {
            let mut active_capture: Option<CaptureControl<_, _>> = None;

            for command in command_rx.iter() {
                if let Some(capture) = active_capture.take() {
                    let _ = capture.stop();
                }

                match command {
                    CaptureCommand::Start(monitor) => {
                        let settings = Settings::new(
                            monitor,
                            CursorCaptureSettings::Default,
                            DrawBorderSettings::Default,
                            SecondaryWindowSettings::Default,
                            MinimumUpdateIntervalSettings::Default,
                            DirtyRegionSettings::Default,
                            ColorFormat::Rgba8,
                            image_tx.clone(),
                        );

                        if let Ok(capture) = ScreenCaptureService::start_free_threaded(settings) {
                            active_capture = Some(capture);
                        }
                    }
                    CaptureCommand::Stop => {}
                }
            }
        });

        Self {
            command_tx,
            image_rx,
        }
    }

    pub fn start_capture(&self, monitor: Monitor) {
        let _ = self.command_tx.send(CaptureCommand::Start(monitor));
    }

    pub fn stop_capture(&self) {
        let _ = self.command_tx.send(CaptureCommand::Stop);
    }

    pub fn try_recv_image(&self) -> Option<ColorImage> {
        self.image_rx.try_recv().ok()
    }
}
