use eframe::egui::{self, Color32, CornerRadius, FontId, Stroke, TextStyle};

pub const BG: Color32 = Color32::from_rgb(0xf3, 0xf3, 0xf1);
pub const BG_TOOLBAR: Color32 = Color32::from_rgb(0xfb, 0xfb, 0xfa);
pub const BG_CONTENT: Color32 = Color32::from_rgb(0xee, 0xf1, 0xf4);
pub const BG_SURFACE: Color32 = Color32::WHITE;
pub const ACCENT: Color32 = Color32::from_rgb(0x3b, 0x6e, 0xf5);
pub const ACCENT_SOFT: Color32 = Color32::from_rgb(0xe9, 0xf0, 0xfe);
pub const INK: Color32 = Color32::from_rgb(0x1f, 0x1f, 0x1f);
pub const TEXT: Color32 = Color32::from_rgb(0x33, 0x33, 0x31);
pub const DOT_RED: Color32 = Color32::from_rgb(0xff, 0x5f, 0x57);
pub const DOT_YELLOW: Color32 = Color32::from_rgb(0xfe, 0xbc, 0x2e);
pub const DOT_GREEN: Color32 = Color32::from_rgb(0x28, 0xc8, 0x40);
pub const BORDER: Color32 = Color32::from_black_alpha(20);
pub const BORDER_SOFT: Color32 = Color32::from_black_alpha(15);
pub const BORDER_STRONG: Color32 = Color32::from_black_alpha(31);
pub const HOVER_FILL: Color32 = Color32::from_black_alpha(15);
pub const WIDGET_HOVER: Color32 = Color32::from_rgb(0xf5, 0xf5, 0xf4);
pub const WIDGET_ACTIVE: Color32 = Color32::from_rgb(0xea, 0xea, 0xe8);
pub const ERROR: Color32 = Color32::from_rgb(0xc6, 0x28, 0x28);

pub fn apply(ctx: &egui::Context) {
    ctx.set_theme(egui::ThemePreference::Light);
    ctx.all_styles_mut(|style| {
        let visuals = &mut style.visuals;
        *visuals = egui::Visuals::light();
        visuals.panel_fill = BG;
        visuals.window_fill = BG_SURFACE;
        visuals.extreme_bg_color = BG_SURFACE;
        visuals.faint_bg_color = BG_TOOLBAR;
        visuals.window_corner_radius = CornerRadius::same(14);
        visuals.window_stroke = Stroke::new(1.0, BORDER);
        visuals.selection.bg_fill = ACCENT.gamma_multiply(0.35);
        visuals.selection.stroke = Stroke::new(1.0, ACCENT);
        visuals.hyperlink_color = ACCENT;
        visuals.override_text_color = Some(TEXT);

        let radius = CornerRadius::same(7);
        for w in [
            &mut visuals.widgets.noninteractive,
            &mut visuals.widgets.inactive,
            &mut visuals.widgets.hovered,
            &mut visuals.widgets.active,
            &mut visuals.widgets.open,
        ] {
            w.corner_radius = radius;
        }
        visuals.widgets.noninteractive.bg_stroke = Stroke::new(1.0, BORDER_SOFT);
        visuals.widgets.inactive.weak_bg_fill = BG_SURFACE;
        visuals.widgets.inactive.bg_fill = BG_SURFACE;
        visuals.widgets.inactive.bg_stroke = Stroke::new(1.0, BORDER_STRONG);
        visuals.widgets.hovered.weak_bg_fill = WIDGET_HOVER;
        visuals.widgets.hovered.bg_stroke = Stroke::new(1.0, BORDER_STRONG);
        visuals.widgets.active.weak_bg_fill = WIDGET_ACTIVE;
        visuals.widgets.active.bg_stroke = Stroke::new(1.0, BORDER_STRONG);

        style.spacing.item_spacing = egui::vec2(8.0, 4.0);
        style.spacing.button_padding = egui::vec2(12.0, 6.0);
        style.spacing.scroll.bar_width = 8.0;

        style.text_styles = [
            (TextStyle::Heading, FontId::proportional(18.0)),
            (TextStyle::Body, FontId::proportional(13.0)),
            (TextStyle::Button, FontId::proportional(13.0)),
            (TextStyle::Small, FontId::proportional(11.0)),
            (TextStyle::Monospace, FontId::monospace(12.0)),
        ]
        .into();
    });
}
