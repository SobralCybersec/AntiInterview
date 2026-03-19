use crate::presentation::{video::*, AppState};
use eframe::{
    egui::{self, Atom, AtomExt, Color32, Direction, FontData, FontDefinitions, FontFamily, 
          FontId, Image, Layout, Margin, RichText, TextStyle, Theme, Vec2},
    Renderer,
};
use image::{GenericImageView, ImageFormat, ImageReader};
use std::{io::Cursor, sync::Arc};
use tracing::{error, info};

const ICON_SIZE: f32 = 16.0;
const SPACING: f32 = 8.0;
const PREVIEW_MAX_HEIGHT: f32 = 500.0;
const SIDE_PANEL_WIDTH: f32 = 60.0;

#[derive(Debug, Clone, Copy, PartialEq)]
enum MenuSection {
    Home,
    Windows,
    Settings,
    Credits,
}

pub struct AntiInterviewApp {
    state: AppState,
    active_section: MenuSection,
    video_renderer: VideoRenderer,
    home_icon: Option<egui::TextureHandle>,
    windows_icon: Option<egui::TextureHandle>,
    settings_icon: Option<egui::TextureHandle>,
    credits_icon: Option<egui::TextureHandle>,
    menu_expanded: bool,
    menu_animation: f32,
    test_opacity: u8,
    test_hide_cursor: bool,
}

impl AntiInterviewApp {
    pub fn new(state: AppState, ctx: &egui::Context) -> Self {
        let mut video_renderer = VideoRenderer::new();
        
        let gif_data = include_bytes!("../../../assets/banner/banner.gif");
        let (frames, video_info) = load_embedded_frames(ctx, gif_data);
        if !frames.is_empty() {
            let video_stream = VideoStream::new(frames, video_info.fps as u32);
            video_renderer.load_video(video_stream);
            info!("Banner loaded: {} frames at {:.2} FPS", video_info.frame_count, video_info.fps);
        }

        let theme = if state.config().ui.dark_theme { Theme::Dark } else { Theme::Light };
        ctx.set_theme(theme);

        let home_icon = Self::load_icon(ctx, "home");
        let windows_icon = Self::load_icon(ctx, "windows");
        let settings_icon = Self::load_icon(ctx, "settings");
        let credits_icon = Self::load_icon(ctx, "credits");

        Self { 
            state,
            active_section: MenuSection::Home,
            video_renderer,
            home_icon,
            windows_icon,
            settings_icon,
            credits_icon,
            menu_expanded: false,
            menu_animation: 0.0,
            test_opacity: 255,
            test_hide_cursor: false,
        }
    }

    fn load_icon(ctx: &egui::Context, name: &str) -> Option<egui::TextureHandle> {
        let path = format!("assets/icons/menu/{}.png", name);
        if let Ok(img) = image::open(&path) {
            let rgba = img.to_rgba8();
            let size = [rgba.width() as usize, rgba.height() as usize];
            let color_image = egui::ColorImage::from_rgba_unmultiplied(size, rgba.as_raw());
            Some(ctx.load_texture(name, color_image, egui::TextureOptions::LINEAR))
        } else {
            None
        }
    }

    fn render_side_menu(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        let target_animation = if self.menu_expanded { 1.0 } else { 0.0 };
        self.menu_animation += (target_animation - self.menu_animation) * 0.2;
        
        if (target_animation - self.menu_animation).abs() > 0.01 {
            ctx.request_repaint();
        }

        ui.vertical_centered(|ui| {
            ui.add_space(SPACING);

            if ui.add_sized([50.0, 30.0], egui::Button::new(if self.menu_expanded { "◀" } else { "▶" }))
                .on_hover_text(if self.menu_expanded { "Recolher" } else { "Expandir" })
                .clicked() 
            {
                self.menu_expanded = !self.menu_expanded;
            }

            ui.add_space(SPACING * 2.0);

            let home_button = if let Some(icon) = &self.home_icon {
                egui::Button::image(Image::from_texture(icon).fit_to_exact_size(Vec2::new(32.0, 32.0)))
            } else {
                egui::Button::image(egui::include_image!("../../../assets/icons/start.png"))
            };

            if self.menu_expanded {
                ui.horizontal(|ui| {
                    if ui.add_sized([50.0, 50.0], home_button).clicked() {
                        self.active_section = MenuSection::Home;
                    }
                    ui.add_space(SPACING);
                    ui.label(RichText::new("Início").size(14.0));
                });
            } else if ui.add_sized([50.0, 50.0], home_button)
                .on_hover_text("Início")
                .clicked() 
            {
                self.active_section = MenuSection::Home;
            }

            ui.add_space(SPACING);

            let windows_button = if let Some(icon) = &self.windows_icon {
                egui::Button::image(Image::from_texture(icon).fit_to_exact_size(Vec2::new(32.0, 32.0)))
            } else {
                egui::Button::image(egui::include_image!("../../../assets/icons/windows.png"))
            };

            if self.menu_expanded {
                ui.horizontal(|ui| {
                    if ui.add_sized([50.0, 50.0], windows_button).clicked() {
                        self.active_section = MenuSection::Windows;
                    }
                    ui.add_space(SPACING);
                    ui.label(RichText::new("Janelas").size(14.0));
                });
            } else if ui.add_sized([50.0, 50.0], windows_button)
                .on_hover_text("Janelas")
                .clicked() 
            {
                self.active_section = MenuSection::Windows;
            }

            ui.add_space(SPACING);

            let settings_button = if let Some(icon) = &self.settings_icon {
                egui::Button::image(Image::from_texture(icon).fit_to_exact_size(Vec2::new(32.0, 32.0)))
            } else {
                egui::Button::image(egui::include_image!("../../../assets/icons/settings.png"))
            };

            if self.menu_expanded {
                ui.horizontal(|ui| {
                    if ui.add_sized([50.0, 50.0], settings_button).clicked() {
                        self.active_section = MenuSection::Settings;
                    }
                    ui.add_space(SPACING);
                    ui.label(RichText::new("Configurações").size(14.0));
                });
            } else if ui.add_sized([50.0, 50.0], settings_button)
                .on_hover_text("Configurações")
                .clicked() 
            {
                self.active_section = MenuSection::Settings;
            }

            ui.add_space(SPACING);

            let credits_button = if let Some(icon) = &self.credits_icon {
                egui::Button::image(Image::from_texture(icon).fit_to_exact_size(Vec2::new(32.0, 32.0)))
            } else {
                egui::Button::image(egui::include_image!("../../../assets/icons/start.png"))
            };

            if self.menu_expanded {
                ui.horizontal(|ui| {
                    if ui.add_sized([50.0, 50.0], credits_button).clicked() {
                        self.active_section = MenuSection::Credits;
                    }
                    ui.add_space(SPACING);
                    ui.label(RichText::new("Créditos").size(14.0));
                });
            } else if ui.add_sized([50.0, 50.0], credits_button)
                .on_hover_text("Créditos")
                .clicked() 
            {
                self.active_section = MenuSection::Credits;
            }
        });
    }

    fn render_credits(&mut self, ui: &mut egui::Ui, _ctx: &egui::Context, _theme: Theme) {
        self.render_banner(ui, _ctx);
        
        ui.vertical_centered(|ui| {
            ui.heading(RichText::new("Créditos Anti-Interview").size(24.0));
            ui.add_space(SPACING);
            ui.label(RichText::new("Versão Atual: v1.0").size(14.0).color(Color32::GRAY));
            ui.add_space(SPACING * 3.0);
            
            ui.label(RichText::new("Autores").size(18.0).strong());
            ui.add_space(SPACING);
            ui.label(RichText::new("Matheus & Pyetrah").size(16.0));
            
            ui.add_space(SPACING * 3.0);
            
            ui.label(RichText::new("Tecnologias").size(18.0).strong());
            ui.add_space(SPACING);
            ui.label("Rust • Windows API • egui");
            
            ui.add_space(SPACING * 3.0);
            
            ui.label(RichText::new("© 2025 - Todos os direitos reservados").size(12.0).color(Color32::GRAY));
        });
    }

    fn render_home(&mut self, ui: &mut egui::Ui, ctx: &egui::Context, _theme: Theme) {
        self.render_banner(ui, ctx);
        
        ui.add_space(SPACING);
        ui.vertical_centered(|ui| {
            ui.heading("Bem-vindo ao Anti-Interview");
            ui.label("Oculte janelas durante compartilhamento de tela.");
            ui.add_space(SPACING);
            
            if ui.button(RichText::new("Gerenciar Janelas").size(16.0)).clicked() {
                self.active_section = MenuSection::Windows;
            }

            ui.add_space(SPACING);

            ui.heading("Como Utilizar?");
            ui.label("Veja o Guia no Projeto");
        });
    }

    fn render_banner(&mut self, ui: &mut egui::Ui, _ctx: &egui::Context) {
        if let Some(banner_url) = &self.state.config().ui.banner_url {
            let width = ui.available_width();
            let aspect_ratio = 1280.0 / 720.0;
            let height = width / aspect_ratio;
            
            ui.add(
                egui::Image::from_uri(banner_url)
                    .fit_to_exact_size(Vec2::new(width, height))
            );
            ui.separator();
            ui.add_space(SPACING);
            return;
        }
        
        if self.video_renderer.has_video() {
            let width = ui.available_width();
            let aspect_ratio = 1280.0 / 720.0;
            let height = width / aspect_ratio;
            
            self.video_renderer.render(ui, width, height);
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

        Self::render_section_header(ui, theme, "Demonstração:", "Como outros verão sua tela");

        self.state.update_capture_texture(ctx);

        ui.vertical_centered(|ui| {
            if let Some(texture) = self.state.capture_texture() {
                let available_width = ui.available_width();
                let texture_size = texture.size_vec2();
                let aspect_ratio = texture_size.x / texture_size.y;
                let display_height = (available_width / aspect_ratio).min(PREVIEW_MAX_HEIGHT);
                
                ui.add(
                    egui::Image::from_texture(texture)
                        .fit_to_exact_size(Vec2::new(available_width, display_height)),
                );
            } else {
                ui.label("Carregando preview...");
            }
        });

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

    fn render_settings(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        ui.heading("Configurações");
        ui.add_space(SPACING);

        ui.group(|ui| {
            ui.label(RichText::new("Interface").strong());
            ui.add_space(SPACING);

            let mut dark_theme = self.state.config().ui.dark_theme;
            if ui.checkbox(&mut dark_theme, "Tema Escuro").changed() {
                self.state.config_mut().ui.dark_theme = dark_theme;
                let theme = if dark_theme { Theme::Dark } else { Theme::Light };
                ctx.set_theme(theme);
            }

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
            ui.label(RichText::new("Comportamento de Janelas").strong());
            ui.add_space(SPACING);
            
            ui.label("Aplicar ao ocultar janelas:");
            ui.add_space(SPACING / 2.0);
            
            let mut ignore_mouse = self.state.config().ui.ignore_mouse;
            if ui.checkbox(&mut ignore_mouse, "Ignorar Mouse (Click-through)").changed() {
                self.state.config_mut().ui.ignore_mouse = ignore_mouse;
            }
        });

        ui.add_space(SPACING);

        ui.group(|ui| {
            ui.label(RichText::new("Testes de Payload").strong());
            ui.add_space(SPACING);
            ui.label(RichText::new("Funções de teste (aplicam na janela ativa)").size(11.0).color(Color32::GRAY));
            ui.add_space(SPACING);
            
            ui.horizontal(|ui| {
                if ui.button("Minimizar").clicked() {
                    info!("Teste: Minimizar janela");
                }
                if ui.button("Maximizar").clicked() {
                    info!("Teste: Maximizar janela");
                }
                if ui.button("Restaurar").clicked() {
                    info!("Teste: Restaurar janela");
                }
            });
            
            ui.add_space(SPACING / 2.0);
            
            ui.horizontal(|ui| {
                if ui.button("Sempre no Topo").clicked() {
                    info!("Teste: Sempre no topo");
                }
                if ui.button("Piscar na Barra").clicked() {
                    info!("Teste: Piscar na barra");
                }
            });
            
            ui.add_space(SPACING / 2.0);
            
            ui.horizontal(|ui| {
                ui.label("Opacidade:");
                if ui.add(egui::Slider::new(&mut self.test_opacity, 0..=255)).changed() {
                    info!("Teste: Opacidade {}", self.test_opacity);
                }
            });
            
            ui.add_space(SPACING / 2.0);
            
            if ui.checkbox(&mut self.test_hide_cursor, "Ocultar Cursor (experimental)").changed() {
                info!("Teste: Ocultar cursor: {}", self.test_hide_cursor);
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
            .exact_width(if self.menu_expanded { 200.0 } else { SIDE_PANEL_WIDTH })
            .show(ctx, |ui| {
                self.render_side_menu(ui, ctx);
            });

        egui::CentralPanel::default()
            .frame(egui::Frame::central_panel(&ctx.style()).inner_margin(Margin::same(14)))
            .show(ctx, |ui| {
                if !focused {
                    ui.with_layout(
                        Layout::centered_and_justified(Direction::LeftToRight),
                        |ui| {
                            ui.label("Não tem nada aqui :x");
                        },
                    );
                    return;
                }

                egui::ScrollArea::vertical().show(ui, |ui| {
                    match self.active_section {
                        MenuSection::Home => self.render_home(ui, ctx, theme),
                        MenuSection::Windows => {
                            self.render_banner(ui, ctx);
                            self.render_preview(ui, ctx, theme);
                            Self::render_section_header(ui, theme, "Gerenciar Janelas", "Selecione as janelas para ocultar");
                            self.render_window_list(ui, ctx, theme);
                        }
                        MenuSection::Settings => {
                            self.render_banner(ui, ctx);
                            self.render_settings(ui, ctx);
                        }
                        MenuSection::Credits => {
                            self.render_credits(ui, ctx, theme);
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
