use crate::presentation::{video::*, AiBrowserState, AiTab, AppState};
use eframe::{
    egui::{
        self, Align, Align2, Color32, CornerRadius, FontData, FontDefinitions, FontFamily, FontId,
        Id, Image, Layout, Margin, RichText, Sense, Stroke, StrokeKind, TextStyle, Theme, Vec2,
    },
    Renderer,
};
use image::{GenericImageView, ImageFormat, ImageReader};
use std::{
    io::Cursor,
    sync::Arc,
    time::{Duration, Instant},
};

const SPACING: f32 = 10.0;
const PREVIEW_MAX_HEIGHT: f32 = 500.0;
const SIDE_PANEL_COLLAPSED: f32 = 84.0;
const SIDE_PANEL_EXPANDED: f32 = 252.0;
const CONTENT_MAX_WIDTH: f32 = 1080.0;
const AI_BROWSER_REFRESH_DELAY: Duration = Duration::from_millis(1500);
const AI_BROWSER_HIDE_DELAY: Duration = Duration::from_millis(500);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum MenuSection {
    Home,
    Windows,
    Settings,
    Credits,
    AiBrowser,
}

#[derive(Debug, Clone, Copy)]
enum AiBrowserAutoHide {
    AwaitingRefresh(Instant),
    AwaitingHide(Instant),
}

impl MenuSection {
    const ALL: [Self; 5] = [
        Self::Home,
        Self::Windows,
        Self::AiBrowser,
        Self::Settings,
        Self::Credits,
    ];

    fn label(self) -> &'static str {
        match self {
            Self::Home => "Visão geral",
            Self::Windows => "Janelas",
            Self::Settings => "Configurações",
            Self::Credits => "Sobre",
            Self::AiBrowser => "AI Browser",
        }
    }
}

#[derive(Clone, Copy)]
struct UiPalette {
    canvas: Color32,
    panel: Color32,
    surface: Color32,
    surface_hover: Color32,
    text: Color32,
    muted: Color32,
    accent: Color32,
    accent_soft: Color32,
    secondary: Color32,
    border: Color32,
    success: Color32,
    warning: Color32,
}

pub struct AntiInterviewApp {
    state: AppState,
    active_section: MenuSection,
    video_renderer: VideoRenderer,
    menu_expanded: bool,
    section_started_at: f64,
    test_opacity: u8,
    test_process_stealth: bool,
    ai_browser: AiBrowserState,
    ai_browser_auto_hide: Option<AiBrowserAutoHide>,
}

impl AntiInterviewApp {
    pub fn new(state: AppState, ctx: &egui::Context) -> Self {
        let mut video_renderer = VideoRenderer::new();

        let gif_data = include_bytes!("../../../assets/banner/banner.gif");
        let (frames, video_info) = load_embedded_frames(ctx, gif_data);
        if !frames.is_empty() {
            let video_stream = VideoStream::new(frames, video_info.fps as u32);
            video_renderer.load_video(video_stream);
        }

        let theme = if state.config().ui.dark_theme {
            Theme::Dark
        } else {
            Theme::Light
        };
        Self::apply_modern_style(ctx, theme);

        let mut ai_browser = AiBrowserState::default();

        ai_browser.set_on_browser_spawn(move || {
            tracing::info!("Callback: Hiding AI browser windows");
        });

        Self {
            state,
            active_section: MenuSection::Home,
            video_renderer,
            menu_expanded: true,
            section_started_at: ctx.input(|input| input.time),
            test_opacity: 255,
            test_process_stealth: false,
            ai_browser,
            ai_browser_auto_hide: None,
        }
    }

    fn palette(theme: Theme) -> UiPalette {
        match theme {
            Theme::Dark => UiPalette {
                canvas: Color32::from_rgb(3, 3, 4),
                panel: Color32::from_rgb(8, 8, 10),
                surface: Color32::from_rgb(15, 15, 18),
                surface_hover: Color32::from_rgb(24, 24, 28),
                text: Color32::from_rgb(246, 246, 248),
                muted: Color32::from_rgb(158, 158, 168),
                accent: Color32::from_rgb(232, 232, 236),
                accent_soft: Color32::from_rgb(42, 42, 48),
                secondary: Color32::from_rgb(165, 180, 252),
                border: Color32::from_rgb(45, 45, 52),
                success: Color32::from_rgb(74, 222, 128),
                warning: Color32::from_rgb(251, 191, 36),
            },
            Theme::Light => UiPalette {
                canvas: Color32::from_rgb(241, 246, 250),
                panel: Color32::from_rgb(231, 240, 245),
                surface: Color32::WHITE,
                surface_hover: Color32::from_rgb(238, 247, 248),
                text: Color32::from_rgb(15, 35, 50),
                muted: Color32::from_rgb(83, 105, 120),
                accent: Color32::from_rgb(13, 148, 136),
                accent_soft: Color32::from_rgb(204, 251, 241),
                secondary: Color32::from_rgb(2, 132, 199),
                border: Color32::from_rgb(202, 218, 227),
                success: Color32::from_rgb(22, 163, 74),
                warning: Color32::from_rgb(180, 83, 9),
            },
        }
    }

    fn blend(from: Color32, to: Color32, amount: f32) -> Color32 {
        Color32::from(egui::lerp(
            egui::Rgba::from(from)..=egui::Rgba::from(to),
            amount.clamp(0.0, 1.0),
        ))
    }

    fn schedule_ai_browser_auto_hide(&mut self, ctx: &egui::Context) {
        self.ai_browser_auto_hide = Some(AiBrowserAutoHide::AwaitingRefresh(
            Instant::now() + AI_BROWSER_REFRESH_DELAY,
        ));
        ctx.request_repaint_after(AI_BROWSER_REFRESH_DELAY);
    }

    fn update_ai_browser_auto_hide(&mut self, ctx: &egui::Context) {
        let Some(pending) = self.ai_browser_auto_hide else {
            return;
        };

        let now = Instant::now();
        let deadline = match pending {
            AiBrowserAutoHide::AwaitingRefresh(deadline)
            | AiBrowserAutoHide::AwaitingHide(deadline) => deadline,
        };

        if now < deadline {
            ctx.request_repaint_after(deadline.duration_since(now));
            return;
        }

        match pending {
            AiBrowserAutoHide::AwaitingRefresh(_) => {
                self.state.refresh_windows();
                self.ai_browser_auto_hide =
                    Some(AiBrowserAutoHide::AwaitingHide(now + AI_BROWSER_HIDE_DELAY));
                ctx.request_repaint_after(AI_BROWSER_HIDE_DELAY);
            }
            AiBrowserAutoHide::AwaitingHide(_) => {
                self.state.hide_browser_windows("AI Browser");
                self.ai_browser_auto_hide = None;
                tracing::info!("AI Browser windows auto-hidden");
            }
        }
    }

    fn apply_modern_style(ctx: &egui::Context, theme: Theme) {
        ctx.set_theme(theme);
        let palette = Self::palette(theme);
        ctx.style_mut(|style| {
            style.animation_time = 0.18;
            style.spacing.item_spacing = Vec2::new(10.0, 10.0);
            style.spacing.button_padding = Vec2::new(14.0, 9.0);
            style.spacing.interact_size.y = 36.0;
            style.visuals.override_text_color = Some(palette.text);
            style.visuals.weak_text_color = Some(palette.muted);
            style.visuals.panel_fill = palette.canvas;
            style.visuals.window_fill = palette.surface;
            style.visuals.extreme_bg_color = palette.panel;
            style.visuals.faint_bg_color = palette.surface;
            style.visuals.hyperlink_color = palette.secondary;
            style.visuals.selection.bg_fill = palette.accent_soft;
            style.visuals.selection.stroke = Stroke::new(1.0_f32, palette.accent);

            style.visuals.widgets.inactive.bg_fill = palette.surface;
            style.visuals.widgets.inactive.weak_bg_fill = palette.surface;
            style.visuals.widgets.inactive.bg_stroke = Stroke::new(1.0_f32, palette.border);
            style.visuals.widgets.inactive.corner_radius = CornerRadius::same(10);
            style.visuals.widgets.hovered.bg_fill = palette.surface_hover;
            style.visuals.widgets.hovered.weak_bg_fill = palette.surface_hover;
            style.visuals.widgets.hovered.bg_stroke = Stroke::new(1.0_f32, palette.accent);
            style.visuals.widgets.hovered.corner_radius = CornerRadius::same(10);
            style.visuals.widgets.active.bg_fill = palette.accent_soft;
            style.visuals.widgets.active.weak_bg_fill = palette.accent_soft;
            style.visuals.widgets.active.bg_stroke = Stroke::new(1.0_f32, palette.accent);
            style.visuals.widgets.active.corner_radius = CornerRadius::same(10);
            style.visuals.widgets.open.bg_fill = palette.surface_hover;
            style.visuals.widgets.open.bg_stroke = Stroke::new(1.0_f32, palette.accent);
            style.visuals.widgets.open.corner_radius = CornerRadius::same(10);
        });
    }

    fn card_frame(palette: UiPalette) -> egui::Frame {
        egui::Frame::new()
            .fill(palette.surface)
            .stroke(Stroke::new(1.0_f32, palette.border))
            .corner_radius(CornerRadius::same(16))
            .inner_margin(Margin::same(18))
            .shadow(egui::epaint::Shadow {
                offset: [0, 8],
                blur: 24,
                spread: 0,
                color: Color32::from_black_alpha(36),
            })
    }

    fn navigate_to(&mut self, section: MenuSection, ctx: &egui::Context) {
        if self.active_section != section {
            self.active_section = section;
            self.section_started_at = ctx.input(|input| input.time);
            ctx.request_repaint();
        }
    }

    fn paint_nav_icon(
        painter: &egui::Painter,
        rect: egui::Rect,
        section: MenuSection,
        color: Color32,
    ) {
        let center = rect.center();
        let stroke = Stroke::new(1.7_f32, color);
        match section {
            MenuSection::Home => {
                let body = egui::Rect::from_center_size(
                    center + Vec2::new(0.0, 3.0),
                    Vec2::new(12.0, 10.0),
                );
                painter.rect_stroke(body, 2, stroke, StrokeKind::Inside);
                painter.line_segment(
                    [center + Vec2::new(-8.0, 0.0), center + Vec2::new(0.0, -7.0)],
                    stroke,
                );
                painter.line_segment(
                    [center + Vec2::new(0.0, -7.0), center + Vec2::new(8.0, 0.0)],
                    stroke,
                );
            }
            MenuSection::Windows => {
                let back = egui::Rect::from_center_size(
                    center + Vec2::new(3.0, -3.0),
                    Vec2::new(13.0, 10.0),
                );
                let front = egui::Rect::from_center_size(
                    center + Vec2::new(-3.0, 3.0),
                    Vec2::new(13.0, 10.0),
                );
                painter.rect_stroke(back, 2, stroke, StrokeKind::Inside);
                painter.rect_stroke(front, 2, stroke, StrokeKind::Inside);
            }
            MenuSection::Settings => {
                for (offset, knob) in [(-6.0, -3.0), (0.0, 4.0), (6.0, -1.0)] {
                    painter.line_segment(
                        [
                            center + Vec2::new(-8.0, offset),
                            center + Vec2::new(8.0, offset),
                        ],
                        stroke,
                    );
                    painter.circle_filled(center + Vec2::new(knob, offset), 2.4, color);
                }
            }
            MenuSection::Credits => {
                painter.circle_stroke(center, 8.0, stroke);
                painter.circle_filled(center + Vec2::new(0.0, -3.5), 1.2, color);
                painter.line_segment(
                    [center + Vec2::new(0.0, 0.0), center + Vec2::new(0.0, 5.0)],
                    Stroke::new(2.0_f32, color),
                );
            }
            MenuSection::AiBrowser => {
                painter.rect_stroke(
                    egui::Rect::from_center_size(center, Vec2::new(19.0, 15.0)),
                    5,
                    stroke,
                    StrokeKind::Inside,
                );
                painter.text(
                    center,
                    Align2::CENTER_CENTER,
                    "AI",
                    FontId::monospace(8.5),
                    color,
                );
            }
        }
    }

    fn render_nav_item(
        ui: &mut egui::Ui,
        ctx: &egui::Context,
        palette: UiPalette,
        section: MenuSection,
        selected: bool,
        expansion: f32,
    ) -> egui::Response {
        let (rect, response) = ui.allocate_exact_size(
            Vec2::new(ui.available_width().max(48.0), 48.0),
            Sense::click(),
        );
        response.widget_info(|| {
            egui::WidgetInfo::labeled(egui::WidgetType::Button, ui.is_enabled(), section.label())
        });
        let response = response.on_hover_text(section.label());
        let emphasis = ctx.animate_bool_responsive(
            Id::new(("nav_item", section)),
            selected || response.hovered(),
        );
        let fill = Self::blend(
            Color32::TRANSPARENT,
            if selected {
                palette.accent_soft
            } else {
                palette.surface_hover
            },
            emphasis,
        );
        ui.painter().rect_filled(rect, CornerRadius::same(12), fill);
        if selected {
            let rail = egui::Rect::from_min_max(
                rect.left_top() + Vec2::new(0.0, 8.0),
                rect.left_bottom() + Vec2::new(3.0, -8.0),
            );
            ui.painter()
                .rect_filled(rail, CornerRadius::same(2), palette.accent);
        }

        let icon_rect = egui::Rect::from_center_size(
            egui::pos2(rect.left() + 25.0, rect.center().y),
            Vec2::splat(24.0),
        );
        Self::paint_nav_icon(
            ui.painter(),
            icon_rect,
            section,
            if selected {
                palette.accent
            } else {
                palette.muted
            },
        );

        if expansion > 0.08 {
            ui.painter().text(
                egui::pos2(rect.left() + 50.0, rect.center().y),
                Align2::LEFT_CENTER,
                section.label(),
                FontId::proportional(13.5),
                palette.text.gamma_multiply(expansion),
            );
        }
        response
    }

    fn render_side_menu(&mut self, ui: &mut egui::Ui, ctx: &egui::Context, expansion: f32) {
        let palette = Self::palette(ctx.theme());
        ui.spacing_mut().item_spacing.y = 8.0;
        ui.add_space(10.0);

        ui.horizontal(|ui| {
            let (logo_rect, _) = ui.allocate_exact_size(Vec2::splat(40.0), Sense::hover());
            ui.painter()
                .rect_filled(logo_rect, CornerRadius::same(12), palette.accent);
            ui.painter().text(
                logo_rect.center(),
                Align2::CENTER_CENTER,
                "A",
                FontId::proportional(20.0),
                Color32::from_rgb(8, 8, 10),
            );

            if expansion > 0.35 {
                ui.vertical(|ui| {
                    ui.label(
                        RichText::new("Anti-Interview")
                            .size(14.0)
                            .strong()
                            .color(palette.text.gamma_multiply(expansion)),
                    );
                    ui.label(
                        RichText::new("Privacy control")
                            .size(10.5)
                            .color(palette.muted.gamma_multiply(expansion)),
                    );
                });
            }
        });

        ui.add_space(8.0);
        let toggle_label = if expansion > 0.55 {
            if self.menu_expanded {
                "<  Recolher menu"
            } else {
                ">  Expandir menu"
            }
        } else if self.menu_expanded {
            "<"
        } else {
            ">"
        };
        if ui
            .add_sized(
                [ui.available_width(), 34.0],
                egui::Button::new(toggle_label).corner_radius(10),
            )
            .on_hover_text(if self.menu_expanded {
                "Recolher navegação"
            } else {
                "Expandir navegação"
            })
            .clicked()
        {
            self.menu_expanded = !self.menu_expanded;
        }

        ui.add_space(10.0);
        for section in MenuSection::ALL {
            if Self::render_nav_item(
                ui,
                ctx,
                palette,
                section,
                self.active_section == section,
                expansion,
            )
            .clicked()
            {
                self.navigate_to(section, ctx);
            }
        }

        ui.with_layout(Layout::bottom_up(Align::Center), |ui| {
            ui.add_space(12.0);
            if expansion > 0.55 {
                ui.horizontal(|ui| {
                    ui.colored_label(palette.success, "●");
                    ui.label(
                        RichText::new(format!("v{}  •  pronto", env!("CARGO_PKG_VERSION")))
                            .size(10.5)
                            .color(palette.muted.gamma_multiply(expansion)),
                    );
                });
                ui.add_space(8.0);
            }
        });
    }

    fn render_badge(ui: &mut egui::Ui, palette: UiPalette, label: &str, color: Color32) {
        egui::Frame::new()
            .fill(Self::blend(palette.surface, color, 0.12))
            .stroke(Stroke::new(1.0_f32, color.gamma_multiply(0.55)))
            .corner_radius(CornerRadius::same(20))
            .inner_margin(Margin::symmetric(10, 5))
            .show(ui, |ui| {
                ui.label(RichText::new(label).size(10.5).strong().color(color));
            });
    }

    fn render_metric_card(
        ui: &mut egui::Ui,
        palette: UiPalette,
        value: &str,
        label: &str,
        detail: &str,
    ) {
        Self::card_frame(palette).show(ui, |ui| {
            ui.set_min_height(92.0);
            ui.label(RichText::new(value).size(25.0).strong().color(palette.text));
            ui.label(RichText::new(label).size(12.5).strong().color(palette.text));
            ui.label(RichText::new(detail).size(10.5).color(palette.muted));
        });
    }

    fn render_ai_browser(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        let palette = Self::palette(ctx.theme());
        Self::render_section_header(
            ui,
            ctx.theme(),
            "AI Browser",
            "Abra sua ferramenta de IA sem perder o fluxo de privacidade.",
        );

        egui::Frame::new()
            .fill(palette.panel)
            .stroke(Stroke::new(1.0_f32, palette.border))
            .corner_radius(CornerRadius::same(14))
            .inner_margin(Margin::same(6))
            .show(ui, |ui| {
                ui.horizontal_wrapped(|ui| {
                    for &tab in AiTab::all() {
                        let is_active = self.ai_browser.active_tab == tab;
                        let selected = ctx.animate_bool_responsive(
                            Id::new(("ai_provider", tab.label())),
                            is_active,
                        );
                        let response = ui.add(
                            egui::Button::new(RichText::new(tab.label()).size(12.5).strong())
                                .fill(Self::blend(
                                    Color32::TRANSPARENT,
                                    palette.accent_soft,
                                    selected,
                                ))
                                .stroke(Stroke::new(
                                    1.0_f32,
                                    Self::blend(palette.border, palette.accent, selected),
                                ))
                                .corner_radius(10),
                        );
                        if response.clicked() {
                            self.ai_browser.select_tab(tab);
                        }
                    }
                });
            });

        ui.add_space(14.0);
        let url = self.ai_browser.active_tab.url();
        Self::card_frame(palette).show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.label(
                        RichText::new(self.ai_browser.active_tab.label())
                            .size(20.0)
                            .strong()
                            .color(palette.text),
                    );
                    ui.label(
                        RichText::new("Sessão isolada em uma janela integrada")
                            .size(11.5)
                            .color(palette.muted),
                    );
                });
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    if self.ai_browser.is_browser_open() {
                        Self::render_badge(ui, palette, "Janela aberta", palette.success);
                    } else {
                        Self::render_badge(ui, palette, "Pronto", palette.secondary);
                    }
                });
            });

            ui.add_space(18.0);
            if self.ai_browser.is_browser_open() {
                egui::Frame::new()
                    .fill(Self::blend(palette.surface, palette.success, 0.08))
                    .corner_radius(CornerRadius::same(12))
                    .inner_margin(Margin::same(14))
                    .show(ui, |ui| {
                        ui.label(
                            RichText::new("O navegador integrado está em execução.")
                                .strong()
                                .color(palette.success),
                        );
                        ui.label(
                            RichText::new("Feche a janela atual antes de abrir outro provedor.")
                                .size(11.5)
                                .color(palette.muted),
                        );
                    });
            } else {
                ui.horizontal_wrapped(|ui| {
                    if ui
                        .add(
                            egui::Button::new(
                                RichText::new(format!(
                                    "Abrir {} integrado",
                                    self.ai_browser.active_tab.label()
                                ))
                                .strong()
                                .color(Color32::from_rgb(8, 8, 10)),
                            )
                            .fill(palette.accent)
                            .stroke(Stroke::NONE)
                            .corner_radius(10)
                            .min_size(Vec2::new(190.0, 40.0)),
                        )
                        .clicked()
                    {
                        tracing::info!(
                            "Opening integrated AI browser: {}",
                            self.ai_browser.active_tab.label()
                        );
                        self.ai_browser.open_browser_window();

                        if self.state.config().ui.auto_hide_ai_browser {
                            self.schedule_ai_browser_auto_hide(ctx);
                        }
                    }
                    if ui
                        .add(
                            egui::Button::new("Abrir no navegador padrão")
                                .corner_radius(10)
                                .min_size(Vec2::new(176.0, 40.0)),
                        )
                        .clicked()
                    {
                        tracing::info!("Opening external browser for: {}", url);
                        if let Err(error) = open::that(url) {
                            tracing::error!("Failed to open external browser: {:?}", error);
                        }
                    }
                });
            }
        });

        ui.add_space(14.0);
        Self::card_frame(palette).show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.label(
                        RichText::new("Screenshot silencioso")
                            .size(16.0)
                            .strong()
                            .color(palette.text),
                    );
                    ui.label(
                        RichText::new("Ctrl+Shift+P captura exatamente o que será compartilhado.")
                            .size(11.5)
                            .color(palette.muted),
                    );
                });
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    let (label, color) = if self.state.last_screenshot_path.is_some() {
                        ("Capturado", palette.success)
                    } else {
                        ("Aguardando", palette.muted)
                    };
                    Self::render_badge(ui, palette, label, color);
                });
            });

            if let Some(path) = self.state.last_screenshot_path.clone() {
                ui.add_space(14.0);
                let mut path_display = path.clone();
                ui.horizontal(|ui| {
                    ui.add(
                        egui::TextEdit::singleline(&mut path_display)
                            .desired_width(f32::INFINITY)
                            .interactive(false),
                    );
                    if ui.button("Copiar").clicked() {
                        ctx.copy_text(path.clone());
                    }
                    if ui.button("Limpar").clicked() {
                        self.ai_browser.clear_screenshot_path();
                        self.state.last_screenshot_path = None;
                    }
                });
            }
        });
    }

    fn render_credits(&mut self, ui: &mut egui::Ui, ctx: &egui::Context, _theme: Theme) {
        let palette = Self::palette(ctx.theme());
        Self::render_section_header(
            ui,
            ctx.theme(),
            "Sobre o Anti-Interview",
            "Tecnologia aberta para controle consciente da sua captura de tela.",
        );

        Self::card_frame(palette).show(ui, |ui| {
            ui.horizontal(|ui| {
                let (logo_rect, _) = ui.allocate_exact_size(Vec2::splat(56.0), Sense::hover());
                ui.painter()
                    .rect_filled(logo_rect, CornerRadius::same(16), palette.accent);
                ui.painter().text(
                    logo_rect.center(),
                    Align2::CENTER_CENTER,
                    "A",
                    FontId::proportional(28.0),
                    Color32::from_rgb(8, 8, 10),
                );
                ui.vertical(|ui| {
                    ui.label(
                        RichText::new("Anti-Interview")
                            .size(22.0)
                            .strong()
                            .color(palette.text),
                    );
                    ui.label(
                        RichText::new(format!("Versão {}", env!("CARGO_PKG_VERSION")))
                            .size(11.5)
                            .color(palette.muted),
                    );
                });
            });
            ui.add_space(20.0);
            ui.separator();
            ui.add_space(16.0);

            let render_details = |ui: &mut egui::Ui| {
                ui.label(RichText::new("Autores").strong().color(palette.accent));
                ui.label("Matheus S. & Pyetrah");
                ui.add_space(12.0);
                ui.label(RichText::new("Tecnologias").strong().color(palette.secondary));
                ui.label("Rust, C++, CMake, Windows API e egui");
                ui.add_space(12.0);
                ui.label(RichText::new("Licença").strong().color(palette.success));
                ui.label("MIT • 2021–2026 radiantly e contribuidores");
            };

            if ui.available_width() >= 640.0 {
                ui.columns(2, |columns| {
                    render_details(&mut columns[0]);
                    columns[1].label(
                        RichText::new("Privacidade é poder escolher o que entra em cena.")
                            .size(19.0)
                            .strong()
                            .color(palette.text),
                    );
                    columns[1].label(
                        RichText::new(
                            "O projeto combina controle de janelas, captura e automação local sem enviar seus dados para um serviço remoto.",
                        )
                        .size(12.0)
                        .color(palette.muted),
                    );
                });
            } else {
                render_details(ui);
            }
        });
    }

    fn render_home(&mut self, ui: &mut egui::Ui, ctx: &egui::Context, _theme: Theme) {
        let palette = Self::palette(ctx.theme());
        let windows = self.state.get_windows();
        let hidden_count = windows.iter().filter(|window| window.is_hidden()).count();
        let preview_enabled = self.state.config().ui.show_preview;
        let mut destination = None;

        Self::card_frame(palette)
            .fill(palette.panel)
            .inner_margin(Margin::same(22))
            .show(ui, |ui| {
                let render_copy = |ui: &mut egui::Ui, destination: &mut Option<MenuSection>| {
                    Self::render_badge(ui, palette, "Proteção de captura", palette.accent);
                    ui.add_space(18.0);
                    ui.label(
                        RichText::new("O que precisa ficar privado, fica.")
                            .size(31.0)
                            .strong()
                            .color(palette.text),
                    );
                    ui.add_space(8.0);
                    ui.label(
                        RichText::new(
                            "Escolha janelas, acompanhe a prévia e entre na chamada com confiança.",
                        )
                        .size(13.5)
                        .color(palette.muted),
                    );
                    ui.add_space(22.0);
                    ui.horizontal_wrapped(|ui| {
                        if ui
                            .add(
                                egui::Button::new(
                                    RichText::new("Gerenciar janelas")
                                        .strong()
                                        .color(Color32::from_rgb(8, 8, 10)),
                                )
                                .fill(palette.accent)
                                .stroke(Stroke::NONE)
                                .corner_radius(10)
                                .min_size(Vec2::new(166.0, 42.0)),
                            )
                            .clicked()
                        {
                            *destination = Some(MenuSection::Windows);
                        }
                        if ui
                            .add(
                                egui::Button::new("Abrir AI Browser")
                                    .corner_radius(10)
                                    .min_size(Vec2::new(146.0, 42.0)),
                            )
                            .clicked()
                        {
                            *destination = Some(MenuSection::AiBrowser);
                        }
                    });
                };

                if ui.available_width() >= 720.0 {
                    ui.columns(2, |columns| {
                        render_copy(&mut columns[0], &mut destination);
                        self.render_banner(&mut columns[1], ctx);
                    });
                } else {
                    render_copy(ui, &mut destination);
                    ui.add_space(20.0);
                    self.render_banner(ui, ctx);
                }
            });

        if let Some(section) = destination {
            self.navigate_to(section, ctx);
        }

        ui.add_space(16.0);
        let total = windows.len().to_string();
        let hidden = hidden_count.to_string();
        let preview = if preview_enabled { "Ativa" } else { "Pausada" };
        if ui.available_width() >= 540.0 {
            ui.columns(3, |columns| {
                Self::render_metric_card(
                    &mut columns[0],
                    palette,
                    &total,
                    "Janelas detectadas",
                    "Lista local atual",
                );
                Self::render_metric_card(
                    &mut columns[1],
                    palette,
                    &hidden,
                    "Janelas protegidas",
                    "Fora da captura",
                );
                Self::render_metric_card(
                    &mut columns[2],
                    palette,
                    preview,
                    "Prévia de captura",
                    "Validação em tempo real",
                );
            });
        } else {
            Self::render_metric_card(
                ui,
                palette,
                &total,
                "Janelas detectadas",
                "Lista local atual",
            );
            ui.add_space(10.0);
            Self::render_metric_card(
                ui,
                palette,
                &hidden,
                "Janelas protegidas",
                "Fora da captura",
            );
            ui.add_space(10.0);
            Self::render_metric_card(
                ui,
                palette,
                preview,
                "Prévia de captura",
                "Validação em tempo real",
            );
        }

        ui.add_space(16.0);
        Self::card_frame(palette).show(ui, |ui| {
            ui.label(
                RichText::new("Fluxo rápido")
                    .size(17.0)
                    .strong()
                    .color(palette.text),
            );
            ui.label(
                RichText::new("Três passos antes de compartilhar sua tela")
                    .size(11.5)
                    .color(palette.muted),
            );
            ui.add_space(14.0);
            ui.horizontal_wrapped(|ui| {
                for (step, title, description) in [
                    ("1", "Escolha", "Marque as janelas privadas"),
                    ("2", "Confira", "Valide a prévia de captura"),
                    ("3", "Compartilhe", "Inicie sua chamada"),
                ] {
                    egui::Frame::new()
                        .fill(palette.canvas)
                        .corner_radius(CornerRadius::same(12))
                        .inner_margin(Margin::same(12))
                        .show(ui, |ui| {
                            ui.set_min_width(164.0);
                            ui.horizontal(|ui| {
                                Self::render_badge(ui, palette, step, palette.secondary);
                                ui.vertical(|ui| {
                                    ui.label(RichText::new(title).strong());
                                    ui.label(
                                        RichText::new(description).size(10.5).color(palette.muted),
                                    );
                                });
                            });
                        });
                }
            });
        });
    }

    fn render_banner(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        let palette = Self::palette(ctx.theme());
        let width = ui.available_width().max(220.0);
        let height = (width / (16.0 / 9.0)).clamp(150.0, 280.0);

        egui::Frame::new()
            .fill(palette.canvas)
            .stroke(Stroke::new(1.0_f32, palette.border))
            .corner_radius(CornerRadius::same(16))
            .inner_margin(Margin::same(6))
            .show(ui, |ui| {
                if let Some(banner_url) = &self.state.config().ui.banner_url {
                    ui.add(
                        egui::Image::from_uri(banner_url)
                            .fit_to_exact_size(Vec2::new(width - 12.0, height)),
                    );
                    return;
                }

                if self.video_renderer.has_video() {
                    let image_rect = ui
                        .scope(|ui| self.video_renderer.render(ui, width - 12.0, height))
                        .response
                        .rect;
                    let progress = ((ctx.input(|input| input.time) * 0.22).fract()) as f32;
                    let scan_y = egui::lerp(image_rect.top()..=image_rect.bottom(), progress);
                    ui.painter().line_segment(
                        [
                            egui::pos2(image_rect.left(), scan_y),
                            egui::pos2(image_rect.right(), scan_y),
                        ],
                        Stroke::new(1.0_f32, palette.accent.gamma_multiply(0.35)),
                    );
                } else {
                    let (rect, _) =
                        ui.allocate_exact_size(Vec2::new(width - 12.0, height), Sense::hover());
                    ui.painter()
                        .rect_filled(rect, CornerRadius::same(12), palette.panel);
                    for scale in [0.30_f32, 0.52, 0.74] {
                        ui.painter().circle_stroke(
                            rect.center(),
                            rect.height() * scale,
                            Stroke::new(1.0_f32, palette.accent.gamma_multiply(0.30)),
                        );
                    }
                    ui.painter().text(
                        rect.center(),
                        Align2::CENTER_CENTER,
                        "PRIVADO",
                        FontId::monospace(12.0),
                        palette.accent,
                    );
                }
            });
    }

    fn render_section_header(ui: &mut egui::Ui, theme: Theme, header: &str, description: &str) {
        let palette = Self::palette(theme);
        ui.label(
            RichText::new(header)
                .size(27.0)
                .strong()
                .color(palette.text),
        );
        ui.label(RichText::new(description).size(12.5).color(palette.muted));
        ui.add_space(18.0);
    }

    fn render_preview(&mut self, ui: &mut egui::Ui, ctx: &egui::Context, theme: Theme) {
        if !self.state.config().ui.show_preview {
            return;
        }
        let palette = Self::palette(theme);
        self.state.update_capture_texture(ctx);

        Self::card_frame(palette).show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.label(
                        RichText::new("Prévia protegida")
                            .size(16.0)
                            .strong()
                            .color(palette.text),
                    );
                    ui.label(
                        RichText::new("Visualize o quadro antes de compartilhar")
                            .size(11.0)
                            .color(palette.muted),
                    );
                });
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    Self::render_badge(ui, palette, "Ao vivo", palette.success);
                });
            });
            ui.add_space(14.0);

            ui.vertical_centered(|ui| {
                if let Some(texture) = self.state.capture_texture() {
                    let available_width = ui.available_width();
                    let texture_size = texture.size_vec2();
                    let aspect_ratio = texture_size.x / texture_size.y;
                    let display_height = (available_width / aspect_ratio).min(PREVIEW_MAX_HEIGHT);
                    egui::Frame::new()
                        .fill(palette.canvas)
                        .corner_radius(CornerRadius::same(12))
                        .inner_margin(Margin::same(4))
                        .show(ui, |ui| {
                            ui.add(egui::Image::from_texture(texture).fit_to_exact_size(
                                Vec2::new(available_width - 8.0, display_height),
                            ));
                        });
                } else {
                    ui.add_space(24.0);
                    ui.spinner();
                    ui.label(RichText::new("Preparando prévia...").color(palette.muted));
                    ui.add_space(24.0);
                }
            });

            let monitor_count = self.state.monitors().len();
            let active_monitor = self.state.active_monitor();
            if monitor_count > 1 {
                ui.add_space(12.0);
                let mut new_monitor = None;
                ui.horizontal_wrapped(|ui| {
                    for index in 0..monitor_count {
                        let is_active = index == active_monitor;
                        if ui
                            .selectable_label(is_active, format!("Tela {}", index + 1))
                            .clicked()
                            && !is_active
                        {
                            new_monitor = Some(index);
                        }
                    }
                });
                if let Some(index) = new_monitor {
                    self.state.set_active_monitor(index);
                }
            }
        });
        ui.add_space(16.0);
    }

    fn render_window_list(&mut self, ui: &mut egui::Ui, ctx: &egui::Context, theme: Theme) {
        let palette = Self::palette(theme);
        let filter_text = self.state.filter_text().to_string();
        let hide_from_taskbar = self.state.config().ui.hide_from_taskbar;
        let hide_from_task_manager = self.state.config().ui.hide_from_task_manager;
        let mut filter = filter_text.clone();

        egui::Frame::new()
            .fill(palette.panel)
            .stroke(Stroke::new(1.0_f32, palette.border))
            .corner_radius(CornerRadius::same(14))
            .inner_margin(Margin::same(12))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.add(
                        egui::TextEdit::singleline(&mut filter)
                            .hint_text("Buscar por janela ou processo")
                            .desired_width(f32::INFINITY),
                    );
                    if ui
                        .add(
                            egui::Button::new("Atualizar")
                                .corner_radius(10)
                                .min_size(Vec2::new(96.0, 38.0)),
                        )
                        .clicked()
                    {
                        self.state.refresh_windows();
                    }
                });
            });
        if filter != filter_text {
            self.state.set_filter_text(filter);
        }

        ui.add_space(12.0);
        let windows = self.state.get_filtered_windows();
        if windows.is_empty() {
            Self::card_frame(palette).show(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(14.0);
                    ui.label(
                        RichText::new("Nenhuma janela encontrada")
                            .size(16.0)
                            .strong()
                            .color(palette.text),
                    );
                    ui.label(
                        RichText::new("Limpe o filtro ou atualize a lista para tentar novamente.")
                            .size(11.5)
                            .color(palette.muted),
                    );
                    ui.add_space(14.0);
                });
            });
            return;
        }

        for window in windows {
            let window_id = window.id().value();
            let icon = self.state.get_icon(ctx, window.id()).cloned();
            let mut is_hidden = window.is_hidden();
            let hidden_animation =
                ctx.animate_bool_responsive(Id::new(("window_hidden", window_id)), is_hidden);
            let base_fill = Self::blend(
                palette.surface,
                palette.accent_soft,
                hidden_animation * 0.70,
            );
            let mut changed = false;
            let mut frame = egui::Frame::new()
                .fill(base_fill)
                .stroke(Stroke::new(
                    1.0_f32,
                    Self::blend(palette.border, palette.accent, hidden_animation),
                ))
                .corner_radius(CornerRadius::same(14))
                .inner_margin(Margin::symmetric(14, 12))
                .begin(ui);

            frame.content_ui.horizontal(|ui| {
                if let Some(texture) = icon.as_ref() {
                    ui.add(
                        Image::from_texture(texture)
                            .fit_to_exact_size(Vec2::splat(28.0))
                            .corner_radius(6),
                    );
                } else {
                    let (icon_rect, _) = ui.allocate_exact_size(Vec2::splat(28.0), Sense::hover());
                    ui.painter().rect_stroke(
                        icon_rect,
                        6,
                        Stroke::new(1.0_f32, palette.border),
                        StrokeKind::Inside,
                    );
                    ui.painter().line_segment(
                        [
                            icon_rect.left_top() + Vec2::new(5.0, 8.0),
                            icon_rect.right_top() + Vec2::new(-5.0, 8.0),
                        ],
                        Stroke::new(1.0_f32, palette.muted),
                    );
                }

                ui.vertical(|ui| {
                    ui.label(
                        RichText::new(window.title())
                            .size(12.5)
                            .strong()
                            .color(palette.text),
                    );
                    ui.label(
                        RichText::new(window.process_name())
                            .size(10.5)
                            .color(palette.muted),
                    );
                });

                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    changed = ui.checkbox(&mut is_hidden, "Ocultar").changed();
                    if is_hidden {
                        Self::render_badge(ui, palette, "Protegida", palette.accent);
                    }
                });
            });

            let response = frame.allocate_space(ui);
            let hover = ctx
                .animate_bool_responsive(Id::new(("window_hover", window_id)), response.hovered());
            frame.frame.fill = Self::blend(base_fill, palette.surface_hover, hover * 0.65);
            frame.paint(ui);
            ui.add_space(8.0);

            if changed {
                self.state.update_window_hidden(window.id(), is_hidden);
                let hide_taskbar = hide_from_taskbar.then_some(is_hidden);
                self.state.toggle_window_visibility(
                    window.process_id().clone(),
                    window.id().clone(),
                    is_hidden,
                    hide_taskbar,
                );
                if is_hidden && hide_from_task_manager {
                    self.state.set_process_stealth(window.process_id().clone());
                }
            }
        }
    }

    fn render_settings(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        let palette = Self::palette(ctx.theme());
        Self::render_section_header(
            ui,
            ctx.theme(),
            "Configurações",
            "Ajuste interface, privacidade, automações e atalhos.",
        );

        Self::card_frame(palette).show(ui, |ui| {
            ui.label(RichText::new("Interface").strong());
            ui.add_space(SPACING);

            let mut dark_theme = self.state.config().ui.dark_theme;
            if ui.checkbox(&mut dark_theme, "Tema Escuro").changed() {
                self.state.config_mut().ui.dark_theme = dark_theme;
                let theme = if dark_theme {
                    Theme::Dark
                } else {
                    Theme::Light
                };
                Self::apply_modern_style(ctx, theme);
            }

            let mut hide_from_taskbar = self.state.config().ui.hide_from_taskbar;
            if ui
                .checkbox(
                    &mut hide_from_taskbar,
                    "Ocultar de Alt+Tab e Barra de Tarefas",
                )
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

        Self::card_frame(palette).show(ui, |ui| {
            ui.label(RichText::new("Comportamento de Janelas").strong());
            ui.add_space(SPACING);

            ui.label("Aplicar ao ocultar janelas:");
            ui.add_space(SPACING / 2.0);

            let mut ignore_mouse = self.state.config().ui.ignore_mouse;
            if ui
                .checkbox(&mut ignore_mouse, "Ignorar Mouse (Click-through)")
                .changed()
            {
                self.state.config_mut().ui.ignore_mouse = ignore_mouse;
            }

            ui.add_space(SPACING / 2.0);

            let mut hide_cursor = self.state.config().ui.hide_cursor;
            if ui.checkbox(&mut hide_cursor, "Ocultar Cursor").changed() {
                self.state.config_mut().ui.hide_cursor = hide_cursor;
                if let Err(e) = self.state.set_cursor_capture_hidden(hide_cursor) {
                    tracing::error!("Failed to set cursor capture visibility: {:?}", e);
                }
            }

            ui.add_space(SPACING / 2.0);

            let mut hide_from_task_manager = self.state.config().ui.hide_from_task_manager;
            if ui
                .checkbox(&mut hide_from_task_manager, "Ocultar do Task Manager")
                .changed()
            {
                self.state.config_mut().ui.hide_from_task_manager = hide_from_task_manager;
            }
        });

        ui.add_space(SPACING);

        Self::card_frame(palette).show(ui, |ui| {
            ui.label(RichText::new("Injetar Hooks no Task Manager").strong());
            ui.add_space(SPACING);
            ui.label(
                RichText::new("Atenção: requer privilégios de administrador")
                    .size(10.0)
                    .color(palette.warning),
            );
            ui.add_space(SPACING / 2.0);

            let mut hook_notepad = self.state.config().ui.hook_notepad;
            if ui
                .checkbox(&mut hook_notepad, "Ocultar Notepad.exe")
                .changed()
            {
                self.state.config_mut().ui.hook_notepad = hook_notepad;
                if hook_notepad {
                    let _ = self.state.inject_hook_dll("hook_notepad.dll");
                }
            }

            let mut hook_firefox = self.state.config().ui.hook_firefox;
            if ui
                .checkbox(&mut hook_firefox, "Ocultar firefox.exe (Firefox)")
                .changed()
            {
                self.state.config_mut().ui.hook_firefox = hook_firefox;
                if hook_firefox {
                    let _ = self.state.inject_hook_dll("hook_firefox.dll");
                }
            }

            let mut hook_edge = self.state.config().ui.hook_edge;
            if ui
                .checkbox(&mut hook_edge, "Ocultar msedge.exe (Microsoft Edge)")
                .changed()
            {
                self.state.config_mut().ui.hook_edge = hook_edge;
                if hook_edge {
                    let _ = self.state.inject_hook_dll("hook_edge.dll");
                }
            }

            let mut hook_chrome = self.state.config().ui.hook_chrome;
            if ui
                .checkbox(&mut hook_chrome, "Ocultar chrome.exe (Google Chrome)")
                .changed()
            {
                self.state.config_mut().ui.hook_chrome = hook_chrome;
                if hook_chrome {
                    let _ = self.state.inject_hook_dll("hook_chrome.dll");
                }
            }

            let mut hook_vscode = self.state.config().ui.hook_vscode;
            if ui
                .checkbox(&mut hook_vscode, "Ocultar Code.exe (Visual Studio Code)")
                .changed()
            {
                self.state.config_mut().ui.hook_vscode = hook_vscode;
                if hook_vscode {
                    let _ = self.state.inject_hook_dll("hook_vscode.dll");
                }
            }

            let mut hook_visualstudio = self.state.config().ui.hook_visualstudio;
            if ui
                .checkbox(&mut hook_visualstudio, "Ocultar devenv.exe (Visual Studio)")
                .changed()
            {
                self.state.config_mut().ui.hook_visualstudio = hook_visualstudio;
                if hook_visualstudio {
                    let _ = self.state.inject_hook_dll("hook_visualstudio.dll");
                }
            }

            let mut hook_antiinterview = self.state.config().ui.hook_antiinterview;
            if ui
                .checkbox(
                    &mut hook_antiinterview,
                    "Ocultar anti-interview.exe (Software)",
                )
                .changed()
            {
                self.state.config_mut().ui.hook_antiinterview = hook_antiinterview;
                if hook_antiinterview {
                    let _ = self.state.inject_hook_dll("hook_antiinterview.dll");
                }
            }

            ui.add_space(SPACING / 2.0);
            ui.label(
                RichText::new("Nota: Hooks são injetados no Taskmgr.exe ao marcar")
                    .size(10.0)
                    .color(palette.muted),
            );
        });

        ui.add_space(SPACING);

        Self::card_frame(palette).show(ui, |ui| {
            ui.label(RichText::new("AI Browser").strong());
            ui.add_space(SPACING);

            let mut auto_hide_ai_browser = self.state.config().ui.auto_hide_ai_browser;
            if ui
                .checkbox(
                    &mut auto_hide_ai_browser,
                    "Auto-ocultar janelas do AI Browser",
                )
                .changed()
            {
                self.state.config_mut().ui.auto_hide_ai_browser = auto_hide_ai_browser;
            }

            ui.add_space(SPACING / 2.0);

            let mut auto_paste_screenshot = self.state.config().ui.auto_paste_screenshot;
            if ui
                .checkbox(
                    &mut auto_paste_screenshot,
                    "Auto-copiar screenshot para clipboard",
                )
                .changed()
            {
                self.state.config_mut().ui.auto_paste_screenshot = auto_paste_screenshot;
            }

            ui.add_space(SPACING / 2.0);
            ui.label(
                RichText::new(
                    "Nota: Screenshot será copiado automaticamente ao pressionar Ctrl+Shift+P",
                )
                .size(10.0)
                .color(palette.muted),
            );
        });

        ui.add_space(SPACING);

        Self::card_frame(palette).show(ui, |ui| {
            ui.label(RichText::new("Testes de Payload").strong());
            ui.add_space(SPACING);

            ui.horizontal(|ui| {
                ui.button("Minimizar").clicked();
                ui.button("Maximizar").clicked();
                if ui.button("Restaurar").clicked() {}
            });

            ui.add_space(SPACING / 2.0);

            ui.horizontal(|ui| {
                ui.button("Sempre no Topo").clicked();
                if ui.button("Piscar na Barra").clicked() {}
            });

            ui.add_space(SPACING / 2.0);

            ui.horizontal(|ui| {
                ui.label("Opacidade:");
                ui.add(egui::Slider::new(&mut self.test_opacity, 0..=255));
            });

            ui.add_space(SPACING / 2.0);

            if ui
                .checkbox(
                    &mut self.test_process_stealth,
                    "Ocultar Processo do Task Manager",
                )
                .changed()
            {
                if self.test_process_stealth {
                    use crate::domain::ProcessId;
                    let current_pid = std::process::id();
                    self.state.set_process_stealth(ProcessId::new(current_pid));
                } else {
                    use crate::domain::ProcessId;
                    self.state.set_process_stealth(ProcessId::new(0));
                }
            }
        });

        ui.add_space(SPACING);

        Self::card_frame(palette).show(ui, |ui| {
            ui.label(RichText::new("Atalhos de Teclado").strong());
            ui.add_space(SPACING);

            ui.horizontal(|ui| {
                ui.label(RichText::new("Ação").strong().size(11.0));
                ui.add_space(40.0);
                ui.label(RichText::new("Atalho").strong().size(11.0));
            });

            ui.separator();
            ui.add_space(SPACING / 2.0);

            ui.horizontal(|ui| {
                ui.label("Screenshot:");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(
                        RichText::new(&self.state.config().hotkeys.screenshot)
                            .monospace()
                            .color(palette.secondary),
                    );
                });
            });

            ui.horizontal(|ui| {
                ui.label("Screenshot Silencioso:");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(
                        RichText::new(&self.state.config().hotkeys.silent_screenshot)
                            .monospace()
                            .color(palette.secondary),
                    );
                });
            });

            ui.horizontal(|ui| {
                ui.label("Ocultar Janela:");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(
                        RichText::new(&self.state.config().hotkeys.hide_window)
                            .monospace()
                            .color(palette.secondary),
                    );
                });
            });

            ui.horizontal(|ui| {
                ui.label("Mostrar/Ocultar GUI:");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(
                        RichText::new(&self.state.config().hotkeys.show_gui)
                            .monospace()
                            .color(palette.secondary),
                    );
                });
            });

            ui.horizontal(|ui| {
                ui.label("Atualizar Janelas:");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(
                        RichText::new(&self.state.config().hotkeys.refresh_windows)
                            .monospace()
                            .color(palette.secondary),
                    );
                });
            });

            ui.horizontal(|ui| {
                ui.label("Alternar Preview:");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(
                        RichText::new(&self.state.config().hotkeys.toggle_preview)
                            .monospace()
                            .color(palette.secondary),
                    );
                });
            });

            ui.horizontal(|ui| {
                ui.label("Abrir AI Browser:");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(
                        RichText::new(&self.state.config().hotkeys.open_ai_browser)
                            .monospace()
                            .color(palette.secondary),
                    );
                });
            });
        });

        ui.add_space(SPACING);

        if ui
            .add(
                egui::Button::new(
                    RichText::new("Salvar configuração")
                        .strong()
                        .color(Color32::from_rgb(8, 8, 10)),
                )
                .fill(palette.accent)
                .stroke(Stroke::NONE)
                .corner_radius(10)
                .min_size(Vec2::new(168.0, 42.0)),
            )
            .clicked()
        {
            let _ = self.state.save_config();
        }
    }
}

impl eframe::App for AntiInterviewApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.update_ai_browser_auto_hide(ctx);

        if self.state.config().ui.hide_cursor {
            ctx.set_cursor_icon(egui::CursorIcon::None);

            if let Some(pos) = ctx.input(|i| i.pointer.hover_pos()) {
                let painter = ctx.layer_painter(egui::LayerId::new(
                    egui::Order::Tooltip,
                    egui::Id::new("fake_cursor"),
                ));

                // Arrow shape using a triangle
                let size = 12.0;
                let tip = pos;
                let p1 = tip + egui::Vec2::new(0.0, size);
                let p2 = tip + egui::Vec2::new(size * 0.4, size * 0.7);

                painter.add(egui::Shape::convex_polygon(
                    vec![tip, p1, p2],
                    egui::Color32::WHITE,
                    egui::Stroke::new(2.0_f32, egui::Color32::BLACK),
                ));
            }
        } else {
            ctx.set_cursor_icon(egui::CursorIcon::Default);
        }

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
            tracing::warn!(
                "close_requested fired (minimize_to_tray={})",
                self.state.config().behavior.minimize_to_tray
            );
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

        // Ctrl+Shift+P: silent screenshot
        if ctx.input(|i| i.modifiers.ctrl && i.modifiers.shift && i.key_pressed(egui::Key::P)) {
            match self.state.take_silent_screenshot() {
                Ok(path) => {
                    self.ai_browser.set_screenshot_path(path.clone());
                    self.navigate_to(MenuSection::AiBrowser, ctx);
                }
                Err(e) => tracing::error!("Silent screenshot failed: {:?}", e),
            }
        }

        // Ctrl+Shift+R: refresh windows
        if ctx.input(|i| i.modifiers.ctrl && i.modifiers.shift && i.key_pressed(egui::Key::R)) {
            self.state.refresh_windows();
            tracing::info!("Windows list refreshed via hotkey");
        }

        // Ctrl+Shift+V: toggle preview
        if ctx.input(|i| i.modifiers.ctrl && i.modifiers.shift && i.key_pressed(egui::Key::V)) {
            let new_state = !self.state.config().ui.show_preview;
            self.state.toggle_preview(new_state);
            tracing::info!("Preview toggled: {}", new_state);
        }

        // Ctrl+Shift+A: open AI browser
        if ctx.input(|i| i.modifiers.ctrl && i.modifiers.shift && i.key_pressed(egui::Key::A)) {
            self.navigate_to(MenuSection::AiBrowser, ctx);
            tracing::info!("AI Browser opened via hotkey");
        }

        let theme = ctx.theme();
        let palette = Self::palette(theme);
        let focused = ctx.input(|i| i.focused);
        let menu_animation =
            ctx.animate_bool_responsive(Id::new("side_menu_expanded"), self.menu_expanded);
        let menu_width = egui::lerp(SIDE_PANEL_COLLAPSED..=SIDE_PANEL_EXPANDED, menu_animation);

        egui::SidePanel::left("side_menu")
            .resizable(false)
            .exact_width(menu_width)
            .frame(
                egui::Frame::side_top_panel(&ctx.style())
                    .fill(palette.panel)
                    .stroke(Stroke::new(1.0_f32, palette.border))
                    .inner_margin(Margin::same(10)),
            )
            .show(ctx, |ui| {
                self.render_side_menu(ui, ctx, menu_animation);
            });

        egui::CentralPanel::default()
            .frame(
                egui::Frame::central_panel(&ctx.style())
                    .fill(palette.canvas)
                    .inner_margin(Margin::same(22)),
            )
            .show(ctx, |ui| {
                if !focused {
                    ui.vertical_centered(|ui| {
                        ui.add_space((ui.available_height() * 0.28).max(80.0));
                        Self::card_frame(palette)
                            .inner_margin(Margin::symmetric(28, 24))
                            .show(ui, |ui| {
                                ui.set_max_width(420.0);
                                ui.label(
                                    RichText::new("Conteúdo protegido")
                                        .size(21.0)
                                        .strong()
                                        .color(palette.text),
                                );
                                ui.label(
                                    RichText::new(
                                        "Retorne ao Anti-Interview para visualizar seus controles.",
                                    )
                                    .size(12.0)
                                    .color(palette.muted),
                                );
                            });
                    });
                    return;
                }

                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.set_max_width(CONTENT_MAX_WIDTH.min(ui.available_width()));

                    let elapsed = ctx.input(|input| input.time) - self.section_started_at;
                    let progress = (elapsed as f32 / 0.28).clamp(0.0, 1.0);
                    let eased = egui::emath::easing::cubic_out(progress);
                    if progress < 1.0 {
                        ctx.request_repaint_after(std::time::Duration::from_millis(16));
                    }
                    ui.add_space((1.0 - eased) * 12.0);
                    ui.set_opacity(0.42 + eased * 0.58);

                    match self.active_section {
                        MenuSection::Home => self.render_home(ui, ctx, theme),
                        MenuSection::Windows => {
                            Self::render_section_header(
                                ui,
                                theme,
                                "Gerenciar janelas",
                                "Defina o que fica fora da captura e valide o resultado.",
                            );
                            self.render_preview(ui, ctx, theme);
                            self.render_window_list(ui, ctx, theme);
                        }
                        MenuSection::Settings => self.render_settings(ui, ctx),
                        MenuSection::Credits => self.render_credits(ui, ctx, theme),
                        MenuSection::AiBrowser => self.render_ai_browser(ui, ctx),
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
            .with_min_inner_size([300.0, 400.0])
            .with_decorations(true),
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
                style.visuals.widgets.hovered.expansion = 0.0;
                style.visuals.widgets.active.expansion = 0.0;

                let mut text_styles = style.text_styles.clone();
                text_styles.insert(
                    TextStyle::Body,
                    FontId {
                        size: 13.0,
                        family: egui::FontFamily::Name("Inter_18pt-Regular".into()),
                    },
                );

                text_styles.insert(
                    TextStyle::Heading,
                    FontId {
                        size: 24.0,
                        family: egui::FontFamily::Name("Inter_18pt-Bold".into()),
                    },
                );

                text_styles.insert(
                    TextStyle::Button,
                    FontId {
                        size: 12.5,
                        family: egui::FontFamily::Name("Inter_18pt-Regular".into()),
                    },
                );

                text_styles.insert(
                    TextStyle::Small,
                    FontId {
                        size: 10.5,
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
