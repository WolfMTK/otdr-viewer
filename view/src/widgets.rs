use eframe::egui::{
    Color32, CornerRadius, CursorIcon, Rect, Response, Sense, Stroke, Ui, Vec2, Widget,
};

use crate::icons::{Icon, icon};
use crate::theme;

const BUTTON_RADIUS: CornerRadius = CornerRadius::same(8);
const DISABLED_OPACITY: f32 = 90.0 / 255.0;

#[derive(Clone, Copy)]
pub struct IconStyle {
    pub normal: Color32,
    pub hover: Color32,
    pub active: Color32,
}

impl IconStyle {
    pub const TOOLBAR: Self = Self {
        normal: theme::INK,
        hover: theme::INK,
        active: theme::ACCENT,
    };

    pub const fn solid(color: Color32) -> Self {
        Self {
            normal: color,
            hover: color,
            active: color,
        }
    }
}

#[must_use = "use `ui.add(...)`"]
pub struct IconButton {
    glyph: Icon,
    style: IconStyle,
    button_size: f32,
    icon_size: f32,
    flat: bool,
    active: bool,
    enabled: bool,
}

impl IconButton {
    pub fn new(glyph: Icon) -> Self {
        Self {
            glyph,
            style: IconStyle::TOOLBAR,
            button_size: 32.0,
            icon_size: 18.0,
            flat: false,
            active: false,
            enabled: true,
        }
    }

    pub fn style(mut self, style: IconStyle) -> Self {
        self.style = style;
        self
    }

    pub fn size(mut self, button: f32, icon: f32) -> Self {
        self.button_size = button;
        self.icon_size = icon;
        self
    }

    pub fn flat(mut self) -> Self {
        self.flat = true;
        self
    }

    pub fn active(mut self, active: bool) -> Self {
        self.active = active;
        self
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }
}

impl Widget for IconButton {
    fn ui(self, ui: &mut Ui) -> Response {
        let sense = if self.enabled {
            Sense::click()
        } else {
            Sense::hover()
        };
        let (rect, response) = ui.allocate_exact_size(Vec2::splat(self.button_size), sense);
        let hovered = self.enabled && response.contains_pointer();

        if ui.is_rect_visible(rect) {
            let background = if self.flat {
                None
            } else if self.active {
                Some(theme::ACCENT_SOFT)
            } else if hovered {
                Some(theme::HOVER_FILL)
            } else {
                None
            };
            if let Some(background) = background {
                ui.painter().rect_filled(rect, BUTTON_RADIUS, background);
            }

            let mut color = if self.active {
                self.style.active
            } else if hovered {
                self.style.hover
            } else {
                self.style.normal
            };
            if !self.enabled {
                color = color.gamma_multiply(DISABLED_OPACITY);
            }
            let glyph_rect = Rect::from_center_size(rect.center(), Vec2::splat(self.icon_size));
            icon(self.glyph, color, self.icon_size).paint_at(ui, glyph_rect);
        }

        if hovered {
            ui.ctx().set_cursor_icon(CursorIcon::PointingHand);
        }
        response
    }
}

pub fn bottom_border(ui: &Ui, rect: Rect, color: Color32) {
    ui.painter()
        .hline(rect.x_range(), rect.bottom() - 0.5, Stroke::new(1.0, color));
}

pub fn right_border(ui: &Ui, rect: Rect, color: Color32) {
    ui.painter()
        .vline(rect.right() - 0.5, rect.y_range(), Stroke::new(1.0, color));
}

pub fn left_border(ui: &Ui, rect: Rect, color: Color32) {
    ui.painter()
        .vline(rect.left() + 0.5, rect.y_range(), Stroke::new(1.0, color));
}

pub fn top_border(ui: &Ui, rect: Rect, color: Color32) {
    ui.painter()
        .hline(rect.x_range(), rect.top() + 0.5, Stroke::new(1.0, color));
}
