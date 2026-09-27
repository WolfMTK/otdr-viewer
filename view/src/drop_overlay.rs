use eframe::egui::{
    self, Align2, Area, Color32, FontId, Id, Order, Rect, Shape, Stroke, pos2, vec2,
};
use view_model::drop::DropHint;

use crate::icons::{Icon, icon};
use crate::theme;

const FADE_SECONDS: f32 = 0.15;
const INSET: f32 = 14.0;
const BACKDROP_OPACITY: f32 = 0.88;
const TINT_OPACITY: f32 = 0.06;
const DASH: f32 = 8.0;
const DASH_GAP: f32 = 6.0;
const BORDER_WIDTH: f32 = 2.0;
const CIRCLE_RADIUS: f32 = 44.0;
const ICON_SIZE: f32 = 40.0;
const TITLE_SIZE: f32 = 18.0;
const SUBTITLE_SIZE: f32 = 13.0;
const TITLE_GAP: f32 = 24.0;
const SUBTITLE_GAP: f32 = 8.0;

struct Look {
    accent: Color32,
    soft: Color32,
    glyph: Icon,
    title: &'static str,
    subtitle: &'static str,
}

impl Look {
    fn of(hint: DropHint) -> Self {
        match hint {
            DropHint::Supported => Self {
                accent: theme::ACCENT,
                soft: theme::ACCENT_SOFT,
                glyph: Icon::Upload,
                title: "Отпустите файл, чтобы открыть",
                subtitle: "Рефлектограмма .sor · Telcordia SR-4731",
            },
            DropHint::Unsupported => Self {
                accent: theme::ERROR,
                soft: theme::ERROR_SOFT,
                glyph: Icon::File,
                title: "Этот файл не открыть",
                subtitle: "Поддерживаются только файлы .sor",
            },
        }
    }
}

pub fn show(ctx: &egui::Context, area: Rect, hint: Option<DropHint>) {
    let id = Id::new("drop-overlay");
    let visibility = ctx.animate_bool_with_time(id, hint.is_some(), FADE_SECONDS);
    if let Some(hint) = hint {
        ctx.data_mut(|d| d.insert_temp(id, hint));
    }
    if visibility <= 0.0 {
        return;
    }
    let Some(hint) = hint.or_else(|| ctx.data(|d| d.get_temp::<DropHint>(id))) else {
        return;
    };
    let look = Look::of(hint);

    Area::new(id)
        .order(Order::Foreground)
        .fixed_pos(area.min)
        .interactable(false)
        .show(ctx, |ui| {
            ui.set_min_size(area.size());
            let painter = ui.painter().with_clip_rect(area);
            let fade = |color: Color32| color.gamma_multiply(visibility);

            let backdrop = theme::BG_CONTENT.gamma_multiply(BACKDROP_OPACITY);
            painter.rect_filled(area, 0.0, fade(backdrop));
            painter.rect_filled(area, 0.0, fade(look.accent.gamma_multiply(TINT_OPACITY)));

            let zone = area.shrink(INSET);
            let corners = [
                zone.left_top(),
                zone.right_top(),
                zone.right_bottom(),
                zone.left_bottom(),
                zone.left_top(),
            ];
            painter.extend(Shape::dashed_line(
                &corners,
                Stroke::new(BORDER_WIDTH, fade(look.accent)),
                DASH,
                DASH_GAP,
            ));

            let grow = 0.9 + 0.1 * visibility;
            let center = zone.center() - vec2(0.0, TITLE_GAP);
            painter.circle_filled(center, CIRCLE_RADIUS * grow, fade(look.soft));
            let glyph = Rect::from_center_size(center, vec2(ICON_SIZE, ICON_SIZE) * grow);
            icon(look.glyph, fade(look.accent), ICON_SIZE).paint_at(ui, glyph);

            let title_top = center.y + CIRCLE_RADIUS + TITLE_GAP;
            painter.text(
                pos2(center.x, title_top),
                Align2::CENTER_TOP,
                look.title,
                FontId::proportional(TITLE_SIZE),
                fade(theme::TEXT_HEADING),
            );
            painter.text(
                pos2(center.x, title_top + TITLE_SIZE + SUBTITLE_GAP),
                Align2::CENTER_TOP,
                look.subtitle,
                FontId::proportional(SUBTITLE_SIZE),
                fade(theme::TEXT_MUTED),
            );
        });
}
