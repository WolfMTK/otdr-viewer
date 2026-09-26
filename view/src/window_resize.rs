use eframe::egui;
use eframe::egui::{CursorIcon, PointerButton, ResizeDirection, ViewportCommand};

const EDGE: f32 = 5.0;
const CORNER: f32 = 10.0;

fn resize_direction(pos: egui::Pos2, rect: egui::Rect) -> Option<ResizeDirection> {
    let [left, right, top, bottom] = [
        pos.x - rect.left(),
        rect.right() - pos.x,
        pos.y - rect.top(),
        rect.bottom() - pos.y,
    ];

    if left.min(right).min(top).min(bottom) > EDGE {
        return None;
    }

    let (w, e, n, s) = (left <= CORNER, right <= CORNER, top <= CORNER, bottom <= CORNER);

    use ResizeDirection::*;
    Some(match (n, s, w, e) {
        (true, _, true, _) => NorthWest,
        (true, _, _, true) => NorthEast,
        (_, true, true, _) => SouthWest,
        (_, true, _, true) => SouthEast,
        (true, ..) => North,
        (_, true, ..) => South,
        (.., true, _) => West,
        _ => East,
    })
}

fn cursor_for(dir: ResizeDirection) -> CursorIcon {
    use ResizeDirection::*;
    match dir {
        North | South => CursorIcon::ResizeVertical,
        East | West => CursorIcon::ResizeHorizontal,
        NorthWest | SouthEast => CursorIcon::ResizeNwSe,
        NorthEast | SouthWest => CursorIcon::ResizeNeSw,
    }
}

pub fn handle_window_resize(ctx: &egui::Context) {
    let (skip, pos, pressed) = ctx.input(|i| {
        let vp = i.viewport();
        (
            vp.maximized.unwrap_or(false) || vp.fullscreen.unwrap_or(false),
            i.pointer.hover_pos(),
            i.pointer.button_pressed(PointerButton::Primary),
        )
    });
    if skip {
        return;
    }
    let Some(dir) = pos.and_then(|p| resize_direction(p, ctx.content_rect())) else {
        return;
    };
    ctx.set_cursor_icon(cursor_for(dir));
    if pressed {
        ctx.send_viewport_cmd(ViewportCommand::BeginResize(dir));
    }
}

#[cfg(test)]
mod tests {
    use eframe::egui::ResizeDirection::{
        East, North, NorthEast, NorthWest, South, SouthEast, SouthWest, West,
    };
    use eframe::egui::{Rect, ResizeDirection, pos2};
    use rstest::rstest;

    use crate::window_resize::{CORNER, EDGE, resize_direction};

    const R: Rect = Rect {
        min: pos2(0.0, 0.0),
        max: pos2(100.0, 100.0),
    };

    #[rstest]
    #[case::center(50.0, 50.0, None)]
    #[case::just_outside_edge(EDGE + 1.0, 50.0, None)]
    #[case::west(0.0, 50.0, Some(West))]
    #[case::east(100.0, 50.0, Some(East))]
    #[case::north(50.0, 0.0, Some(North))]
    #[case::south(50.0, 100.0, Some(South))]
    #[case::west_boundary(EDGE, 50.0, Some(West))]
    #[case::east_boundary(100.0 - EDGE, 50.0, Some(East))]
    #[case::nw(0.0, 0.0, Some(NorthWest))]
    #[case::ne(100.0, 0.0, Some(NorthEast))]
    #[case::sw(0.0, 100.0, Some(SouthWest))]
    #[case::se(100.0, 100.0, Some(SouthEast))]
    #[case::nw_via_west_edge(2.0, CORNER, Some(NorthWest))]
    #[case::se_via_east_edge(98.0, 100.0 - CORNER, Some(SouthEast))]
    #[case::west_past_corner(2.0, CORNER + 1.0, Some(West))]
    #[case::nw_via_north_edge(CORNER, 2.0, Some(NorthWest))]
    #[case::se_via_south_edge(100.0 - CORNER, 98.0, Some(SouthEast))]
    #[case::north_past_corner(CORNER + 1.0, 2.0, Some(North))]
    #[case::corner_zone_off_edge(EDGE + 1.0, EDGE + 1.0, None)]
    #[case::corner_zone_center(CORNER, CORNER, None)]
    fn direction(#[case] x: f32, #[case] y: f32, #[case] expected: Option<ResizeDirection>) {
        assert_eq!(resize_direction(pos2(x, y), R), expected);
    }

    #[rstest]
    #[case::west(200.0, 400.0, Some(West))]
    #[case::se(400.0, 500.0, Some(SouthEast))]
    #[case::center(300.0, 400.0, None)]
    fn non_zero_origin(#[case] x: f32, #[case] y: f32, #[case] expected: Option<ResizeDirection>) {
        let r = Rect::from_min_max(pos2(200.0, 300.0), pos2(400.0, 500.0));
        assert_eq!(resize_direction(pos2(x, y), r), expected);
    }
}
