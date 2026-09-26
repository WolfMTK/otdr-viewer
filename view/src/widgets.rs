use eframe::egui::{
    Color32, CornerRadius, CursorIcon, Rect, Response, Sense, Stroke, Ui, Vec2, Widget,
};

use crate::icons::{Icon, icon};
use crate::theme;

const BUTTON_RADIUS: CornerRadius = CornerRadius::same(8);

#[derive(Clone, Copy)]
pub struct IconStyle {
    pub normal: Color32,
    pub hover: Color32,
}

impl IconStyle {
    pub const TOOLBAR: Self = Self {
        normal: theme::INK,
        hover: theme::INK,
    };

    pub const fn solid(color: Color32) -> Self {
        Self {
            normal: color,
            hover: color,
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
}

impl IconButton {
    pub fn new(glyph: Icon) -> Self {
        Self {
            glyph,
            style: IconStyle::TOOLBAR,
            button_size: 32.0,
            icon_size: 18.0,
            flat: false,
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
}

impl Widget for IconButton {
    fn ui(self, ui: &mut Ui) -> Response {
        let (rect, response) =
            ui.allocate_exact_size(Vec2::splat(self.button_size), Sense::click());
        let hovered = response.contains_pointer();

        if ui.is_rect_visible(rect) {
            if hovered && !self.flat {
                ui.painter()
                    .rect_filled(rect, BUTTON_RADIUS, theme::HOVER_FILL);
            }
            let color = if hovered {
                self.style.hover
            } else {
                self.style.normal
            };
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
