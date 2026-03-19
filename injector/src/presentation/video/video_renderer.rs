use eframe::egui::{self, Vec2};
use super::VideoStream;

pub struct VideoRenderer {
    video_stream: Option<VideoStream>,
}

impl VideoRenderer {
    pub fn new() -> Self {
        Self {
            video_stream: None,
        }
    }

    pub fn load_video(&mut self, video_stream: VideoStream) {
        self.video_stream = Some(video_stream);
    }

    pub fn render(&mut self, ui: &mut egui::Ui, width: f32, height: f32) {
        if let Some(stream) = &mut self.video_stream {
            if let Some(texture) = stream.current_frame() {
                ui.add(
                    egui::Image::from_texture(texture)
                        .fit_to_exact_size(Vec2::new(width, height))
                );
                ui.ctx().request_repaint();
            }
        }
    }

    pub fn has_video(&self) -> bool {
        self.video_stream.as_ref().map_or(false, |s| !s.is_empty())
    }
}
