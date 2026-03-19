use crate::presentation::AppState;
use eframe::{
    egui::{
        self, Atom, AtomExt, Color32, Direction, FontData, FontDefinitions, FontFamily, FontId,
        Image, Layout, Margin, RichText, TextStyle, Theme, Vec2,
    },
    Renderer,
};
use image::{GenericImageView, ImageFormat, ImageReader};
use std::{io::Cursor, sync::Arc};
use tracing::{error, info};

const ICON_SIZE: f32 = 16.0;
const SPACING: f32 = 8.0;
const PREVIEW_MAX_HEIGHT: f32 = 300.0;
const SIDE_PANEL_WIDTH: f32 = 60.0;

#[derive(Debug, Clone, Copy, PartialEq)]
enum MenuSection {
    Home,
    Windows,
    Settings,
}

pub struct AntiInterviewApp {
    state: AppState,
    active_section: MenuSection,
    #[cfg(feature = "video")]
    video_player: Option<egui_video::Player>,
    gif_frames: Vec<egui::TextureHandle>,
    gif_frame_index: usize,
    gif_last_update: std::time::Instant,
    gif_frame_delay: std::time::Duration,
}

impl AntiInterviewApp {
    pub fn new(state: AppState, ctx: &egui::Context) -> Self {
        #[cfg(feature = "video")]
        let video_player = Self::init_video_player(ctx);
        
        #[cfg(feature = "video")]
        let use_gif = video_player.is_none();
        #[cfg(not(feature = "video"))]
        let use_gif = true;
        
        let gif_frames = if use_gif {
            Self::load_gif_frames(ctx)
        } else {
            Vec::new()
        };

        Self { 
            state,
            active_section: MenuSection::Home,
            #[cfg(feature = "video")]
            video_player,
            gif_frames,
            gif_frame_index: 0,
            gif_last_update: std::time::Instant::now(),
            gif_frame_delay: std::time::Duration::from_millis(100),
        }
    }

    #[cfg(feature = "video")]
    fn init_video_player(ctx: &egui::Context) -> Option<egui_video::Player> {
        let video_data = include_bytes!("../../../assets/banner/banner.mp4");
        
        match egui_video::Player::new_from_bytes(ctx, video_data) {
            Ok(mut player) => {
                player.set_looping(true);
                info!("Vídeo banner carregado com sucesso");
                Some(player)
            }
            Err(e) => {
                error!("Falha ao carregar vídeo: {:?}", e);
                None
            }
        }
    }

    fn load_gif_frames(ctx: &egui::Context) -> Vec<egui::TextureHandle> {
        use image::AnimationDecoder;
        
        let gif_data = include_bytes!("../../../assets/banner/banner.gif");
        
        match image::codecs::gif::GifDecoder::new(Cursor::new(gif_data)) {
            Ok(decoder) => {
                let frames = decoder.into_frames();
                let mut textures = Vec::new();
                
                for (i, frame_result) in frames.enumerate() {
                    match frame_result {
                        Ok(frame) => {
                            let img = frame.into_buffer();
                            let size = [img.width() as usize, img.height() as usize];
                            let color_image = egui::ColorImage::from_rgba_unmultiplied(
                                size,
                                img.as_raw(),
                            );
                            
                            textures.push(ctx.load_texture(
                                format!("banner_gif_frame_{}", i),
                                color_image,
                                egui::TextureOptions::LINEAR,
                            ));
                        }
                        Err(e) => {
                            error!("Falha ao decodificar frame {}: {:?}", i, e);
                            break;
                        }
                    }
                }
                
                if !textures.is_empty() {
                    info!("GIF banner carregado com {} frames", textures.len());
                }
                
                textures
            }
            Err(e) => {
                error!("Falha ao carregar GIF: {:?}", e);
                Vec::new()
            }
        }
    }

    fn render_side_menu(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.add_space(SPACING);

            if ui.add_sized(
                [50.0, 50.0],
                egui::Button::new(RichText::new("■").size(24.0))
            ).on_hover_text("Início").clicked() {
                self.active_section = MenuSection::Home;
            }

            ui.add_space(SPACING);

            if ui.add_sized(
                [50.0, 50.0],
                egui::Button::new(RichText::new("□").size(24.0))
            ).on_hover_text("Janelas").clicked() {
                self.active_section = MenuSection::Windows;
            }

            ui.add_space(SPACING);

            if ui.add_sized(
                [50.0, 50.0],
                egui::Button::new(RichText::new("⚙").size(24.0))
            ).on_hover_text("Configurações").clicked() {
                self.active_section = MenuSection::Settings;
            }
        });
    }

    fn render_home(&mut self, ui: &mut egui::Ui, ctx: &egui::Context, theme: Theme) {
        self.render_banner(ui, ctx);
        self.render_preview(ui, ctx, theme);
        
        ui.add_space(SPACING);
        ui.vertical_centered(|ui| {
            ui.heading("Bem-vindo ao Anti-Interview");
            ui.label("Oculte janelas durante compartilhamento de tela.");
            ui.add_space(SPACING);
            
            if ui.button(RichText::new("Gerenciar Janelas").size(16.0)).clicked() {
                self.active_section = MenuSection::Windows;
            }
        });
    }

    fn render_banner(&mut self, ui: &mut egui::Ui, _ctx: &egui::Context) {
        if let Some(banner_url) = &self.state.config().ui.banner_url {
            let width = ui.available_width();
            let height = 150.0;
            
            ui.vertical_centered(|ui| {
                ui.add(
                    egui::Image::from_uri(banner_url)
                        .fit_to_exact_size(Vec2::new(width, height))
                );
            });
            ui.separator();
            ui.add_space(SPACING);
            return;
        }
        
        #[cfg(feature = "video")]
        if let Some(player) = &mut self.video_player {
            let width = ui.available_width();
            let aspect_ratio = 16.0 / 9.0;
            let height = (width / aspect_ratio).min(150.0);
            let size = [width, height];
            
            ui.vertical_centered(|ui| {
                player.ui(ui, size);
            });
            ui.separator();
            ui.add_space(SPACING);
            return;
        }
        
        if !self.gif_frames.is_empty() {
            let now = std::time::Instant::now();
            if now.duration_since(self.gif_last_update) >= self.gif_frame_delay {
                self.gif_frame_index = (self.gif_frame_index + 1) % self.gif_frames.len();
                self.gif_last_update = now;
            }
            
            let texture = &self.gif_frames[self.gif_frame_index];
            let width = ui.available_width();
            let aspect_ratio = texture.size()[0] as f32 / texture.size()[1] as f32;
            let height = (width / aspect_ratio).min(150.0);
            
            ui.vertical_centered(|ui| {
                ui.add(
                    egui::Image::from_texture(texture)
                        .fit_to_exact_size(Vec2::new(width, height))
                );
            });
            
            ui.ctx().request_repaint();
        } else {
            ui.vertical_centered(|ui| {
                ui.add_space(10.0);
                ui.heading(RichText::new("Anti-Interview").size(22.0));
                ui.label(RichText::new("Oculte janelas durante compartilhamento de tela").size(11.0).color(Color32::GRAY));
                ui.add_space(10.0);
            });
        }
        ui.separator();
        ui.add_space(SPACING);
    }

    fn render_section_header(
        ui: &mut egui::Ui,
        theme: Theme,
        header: &str,
        description: &str,
    ) {
        let (header_color, desc_color) = match theme {
            Theme::Light => (
                Color32::from_rgb(34, 34, 34),
                Color32::from_rgb(119, 119, 119),
            ),
            Theme::Dark => (
                Color32::from_rgb(242, 242, 242),
                Color32::from_rgb(148, 148, 148),
            ),
        };

        ui.label(RichText::new(header).heading().color(header_color));
        ui.label(RichText::new(description).color(desc_color));
        ui.add_space(SPACING);
    }

    fn render_preview(&mut self, ui: &mut egui::Ui, ctx: &egui::Context, theme: Theme) {
        if !self.state.config().ui.show_preview {
            return;
        }

        Self::render_section_header(ui, theme, "Preview", "Como outros verão sua tela");

        self.state.update_capture_texture(ctx);

        if let Some(texture) = self.state.capture_texture() {
            ui.add(
                egui::Image::from_texture(texture)
                    .shrink_to_fit()
                    .max_height(PREVIEW_MAX_HEIGHT),
            );
        } else {
            ui.label("Carregando preview...");
        }

        let monitor_count = self.state.monitors().len();
        let active_monitor = self.state.active_monitor();

        if monitor_count > 1 {
            ui.add_space(SPACING);
            let mut new_monitor = None;
            ui.horizontal_wrapped(|ui| {
                for i in 0..monitor_count {
                    let is_active = i == active_monitor;
                    if ui
                        .selectable_label(is_active, format!("Tela {}", i + 1))
                        .clicked()
                        && !is_active
                    {
                        new_monitor = Some(i);
                    }
                }
            });
            if let Some(i) = new_monitor {
                self.state.set_active_monitor(i);
            }
        }

        ui.add_space(14.0);
    }

    fn render_window_list(&mut self, ui: &mut egui::Ui, ctx: &egui::Context, _theme: Theme) {
        let filter_text = self.state.filter_text().to_string();
        let hide_from_taskbar = self.state.config().ui.hide_from_taskbar;

        let mut filter = filter_text.clone();
        ui.horizontal(|ui| {
            ui.label("Filtro:");
            ui.text_edit_singleline(&mut filter);

            if ui.button("Selecionar Tudo").clicked() {
                info!("Selecionar tudo clicado");
            }

            if ui.button("Limpar").clicked() {
                info!("Limpar clicado");
            }
        });

        if filter != filter_text {
            self.state.set_filter_text(filter);
        }

        ui.add_space(SPACING);

        let windows = self.state.get_filtered_windows();

        for window in windows.iter() {
            ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Truncate);

            let icon_atom = self.state.get_icon(ctx, window.id())
                .map(|texture| Image::from_texture(texture)
                    .max_height(ICON_SIZE)
                    .atom_max_width(ICON_SIZE))
                .unwrap_or_else(|| Atom::grow().atom_size(Vec2::new(ICON_SIZE, 0.0)));

            let checkbox_label = (
                Atom::grow().atom_size(Vec2::ZERO),
                icon_atom,
                Atom::grow().atom_size(Vec2::ZERO),
                window.title(),
            );

            let mut is_hidden = window.is_hidden();
            if ui.checkbox(&mut is_hidden, checkbox_label).changed() {
                self.state.update_window_hidden(window.id(), is_hidden);

                let hide_taskbar = if hide_from_taskbar { Some(is_hidden) } else { None };

                self.state.toggle_window_visibility(
                    window.process_id().clone(),
                    window.id().clone(),
                    is_hidden,
                    hide_taskbar,
                );
            }

            ui.add_space(2.0);
        }
    }

    fn render_settings(&mut self, ui: &mut egui::Ui) {
        ui.heading("Configurações");
        ui.add_space(SPACING);

        ui.group(|ui| {
            ui.label(RichText::new("Interface").strong());
            ui.add_space(SPACING);

            let mut hide_from_taskbar = self.state.config().ui.hide_from_taskbar;
            if ui
                .checkbox(&mut hide_from_taskbar, "Ocultar de Alt+Tab e Barra de Tarefas")
                .changed()
            {
                self.state.config_mut().ui.hide_from_taskbar = hide_from_taskbar;
            }

            let mut show_preview = self.state.config().ui.show_preview;
            if ui
                .checkbox(&mut show_preview, "Mostrar preview da área de trabalho")
                .changed()
            {
                self.state.toggle_preview(show_preview);
            }
        });

        ui.add_space(SPACING);

        ui.group(|ui| {
            ui.label(RichText::new("Atalhos").strong());
            ui.add_space(SPACING);
            ui.label(format!("Screenshot: {}", self.state.config().hotkeys.screenshot));
            ui.label(format!("Ocultar Janela: {}", self.state.config().hotkeys.hide_window));
            ui.label(format!("Mostrar GUI: {}", self.state.config().hotkeys.show_gui));
        });

        ui.add_space(SPACING);

        if ui.button("Salvar Configuração").clicked() {
            match self.state.save_config() {
                Ok(_) => info!("Configuração salva com sucesso"),
                Err(e) => error!("Falha ao salvar configuração: {:?}", e),
            }
        }
    }
}

impl eframe::App for AntiInterviewApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if let Some(event) = self.state.check_tray_events() {
            match event {
                crate::infrastructure::TrayEvent::Show => {
                    self.state.set_window_visible(true);
                    ctx.send_viewport_cmd(egui::ViewportCommand::Visible(true));
                }
                crate::infrastructure::TrayEvent::Quit => {
                    self.state.stop_capture();
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    return;
                }
            }
        }

        if ctx.input(|i| i.viewport().close_requested()) {
            if self.state.config().behavior.minimize_to_tray {
                self.state.set_window_visible(false);
                ctx.send_viewport_cmd(egui::ViewportCommand::Visible(false));
                ctx.send_viewport_cmd(egui::ViewportCommand::CancelClose);
                return;
            } else {
                self.state.stop_capture();
                return;
            }
        }

        for event in ctx.input(|i| i.events.clone()) {
            if let egui::Event::WindowFocused(focused) = event {
                if focused {
                    self.state.refresh_windows();
                    if self.state.config().ui.show_preview {
                        self.state.start_capture();
                    }
                } else {
                    self.state.stop_capture();
                }
            }
        }

        let theme = ctx.theme();
        let focused = ctx.input(|i| i.focused);

        egui::SidePanel::left("side_menu")
            .resizable(false)
            .exact_width(SIDE_PANEL_WIDTH)
            .show(ctx, |ui| {
                self.render_side_menu(ui);
            });

        egui::CentralPanel::default()
            .frame(egui::Frame::central_panel(&ctx.style()).inner_margin(Margin::same(14)))
            .show(ctx, |ui| {
                if !focused {
                    ui.with_layout(
                        Layout::centered_and_justified(Direction::LeftToRight),
                        |ui| {
                            ui.label(":)");
                        },
                    );
                    return;
                }

                egui::ScrollArea::vertical().show(ui, |ui| {
                    match self.active_section {
                        MenuSection::Home => self.render_home(ui, ctx, theme),
                        MenuSection::Windows => {
                            self.render_banner(ui, ctx);
                            Self::render_section_header(ui, theme, "Gerenciar Janelas", "Selecione as janelas para ocultar");
                            self.render_window_list(ui, ctx, theme);
                        }
                        MenuSection::Settings => {
                            self.render_banner(ui, ctx);
                            self.render_settings(ui);
                        }
                    }
                });
            });
    }
}

pub fn start(state: AppState) {
    let config = state.config().clone();

    let mut options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size(config.ui.window_size)
            .with_resizable(true)
            .with_min_inner_size([300.0, 400.0]),
        renderer: Renderer::Glow,
        vsync: true,
        ..Default::default()
    };

    if let Ok(image) = ImageReader::with_format(
        Cursor::new(include_bytes!("../../../assets/icons/invicon.ico")),
        ImageFormat::Ico,
    )
    .decode()
    {
        let (width, height) = image.dimensions();
        options.viewport = options.viewport.with_icon(Arc::new(eframe::egui::IconData {
            rgba: image.into_rgba8().into_raw(),
            width,
            height,
        }));
    }

    eframe::run_native(
        "Anti-Interview",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);

            let mut fonts = FontDefinitions::default();

            fonts.font_data.insert(
                "Inter_18pt-Regular".to_owned(),
                Arc::new(FontData::from_static(include_bytes!(
                    "../../../assets/fonts/Inter_18pt-Regular.ttf"
                ))),
            );

            fonts.families.insert(
                FontFamily::Name("Inter_18pt-Regular".into()),
                vec!["Inter_18pt-Regular".to_owned()],
            );

            fonts.font_data.insert(
                "Inter_18pt-Bold".to_owned(),
                Arc::new(FontData::from_static(include_bytes!(
                    "../../../assets/fonts/Inter_18pt-Bold.ttf"
                ))),
            );

            fonts.families.insert(
                FontFamily::Name("Inter_18pt-Bold".into()),
                vec!["Inter_18pt-Bold".to_owned()],
            );

            cc.egui_ctx.set_fonts(fonts);

            cc.egui_ctx.all_styles_mut(|style| {
                style.visuals.widgets.inactive.corner_radius = Default::default();
                style.visuals.widgets.hovered.corner_radius = Default::default();
                style.visuals.widgets.active.corner_radius = Default::default();
                style.visuals.widgets.hovered.bg_stroke = Default::default();
                style.visuals.widgets.active.bg_stroke = Default::default();
                style.visuals.widgets.hovered.expansion = 0.0;
                style.visuals.widgets.active.expansion = 0.0;

                let mut text_styles = style.text_styles.clone();
                text_styles.insert(
                    TextStyle::Body,
                    FontId {
                        size: 12.0,
                        family: egui::FontFamily::Name("Inter_18pt-Regular".into()),
                    },
                );

                text_styles.insert(
                    TextStyle::Heading,
                    FontId {
                        size: 16.0,
                        family: egui::FontFamily::Name("Inter_18pt-Bold".into()),
                    },
                );

                text_styles.insert(
                    TextStyle::Button,
                    FontId {
                        size: 12.0,
                        family: egui::FontFamily::Name("Inter_18pt-Regular".into()),
                    },
                );

                style.text_styles = text_styles;
            });

            Ok(Box::new(AntiInterviewApp::new(state, &cc.egui_ctx)))
        }),
    )
    .expect("Failed to create window");
}
