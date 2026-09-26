use eframe::egui::{
    self, Align, Color32, CursorIcon, Layout, PointerButton, RichText, Sense, vec2,
};
use view_model::title_bar::TitleBarViewModel;

use crate::icons::{Icon, logo};
use crate::theme;
use crate::widgets::{self, IconButton, IconStyle};

pub const HEIGHT: f32 = 40.0;
const MARGIN_X: i8 = 10;
const MARGIN_Y: i8 = 5;
const ITEM_GAP: f32 = 5.0;
const LOGO_SIZE: f32 = 25.0;
const MENU_BUTTON_SIZE: f32 = 26.0;
const MENU_ICON_SIZE: f32 = 20.0;
const MENU_FONT_SIZE: f32 = 13.0;
const MENU_ITEM_GAP: f32 = 15.0;
const DOT_SIZE: f32 = 15.0;
const DOT_GAP: f32 = 10.0;

pub fn show(ui: &mut egui::Ui, vm: &mut TitleBarViewModel) {
    let panel = egui::Panel::top("title-bar")
        .exact_size(HEIGHT)
        .show_separator_line(false)
        .frame(
            egui::Frame::new()
                .fill(theme::BG)
                .inner_margin(egui::Margin::symmetric(MARGIN_X, MARGIN_Y)),
        )
        .show(ui, |ui| {
            ui.horizontal_centered(|ui| {
                ui.spacing_mut().item_spacing.x = ITEM_GAP;
                ui.add(logo(LOGO_SIZE));

                let menu_rect = ui.scope(|ui| menu(ui, vm)).response.rect;
                close_menu_on_outside_click(ui, vm, menu_rect);

                drag_region_and_controls(ui, vm);
            });
        });
    widgets::bottom_border(ui, panel.response.rect, theme::BORDER);
}

fn menu(ui: &mut egui::Ui, vm: &mut TitleBarViewModel) {
    if !vm.menu_open() {
        let button = IconButton::new(Icon::Menu)
            .style(IconStyle::solid(theme::TEXT))
            .size(MENU_BUTTON_SIZE, MENU_ICON_SIZE)
            .flat();
        if ui.add(button).clicked() {
            vm.open_menu();
        }
        return;
    }

    ui.add_space(ITEM_GAP);
    ui.spacing_mut().item_spacing.x = MENU_ITEM_GAP;
    for (index, label) in vm.menu_items().iter().enumerate() {
        let text = RichText::new(*label)
            .size(MENU_FONT_SIZE)
            .color(theme::TEXT);
        if ui.add(egui::Button::new(text).frame(false)).clicked() {
            vm.select_menu_item(index);
        }
    }
}

fn close_menu_on_outside_click(ui: &egui::Ui, vm: &mut TitleBarViewModel, menu_rect: egui::Rect) {
    if !vm.menu_open() {
        return;
    }
    let clicked_outside = ui.input(|i| {
        i.pointer.primary_clicked()
            && i.pointer
                .interact_pos()
                .is_some_and(|p| !menu_rect.contains(p))
    });
    if clicked_outside {
        vm.close_menu();
    }
}

fn drag_region_and_controls(ui: &mut egui::Ui, vm: &mut TitleBarViewModel) {
    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
        ui.spacing_mut().item_spacing.x = DOT_GAP;

        if dot(ui, theme::DOT_RED).clicked() {
            vm.close_window();
        }
        if dot(ui, theme::DOT_YELLOW).clicked() {
            vm.toggle_maximize();
        }
        if dot(ui, theme::DOT_GREEN).clicked() {
            vm.minimize();
        }

        let (_, drag) = ui.allocate_exact_size(ui.available_size(), Sense::click_and_drag());
        if drag.drag_started_by(PointerButton::Primary) {
            vm.start_drag();
        }
        if drag.double_clicked() {
            vm.toggle_maximize();
        }
    });
}

fn dot(ui: &mut egui::Ui, color: Color32) -> egui::Response {
    let (rect, response) = ui.allocate_exact_size(vec2(DOT_SIZE, DOT_SIZE), Sense::click());
    if ui.is_rect_visible(rect) {
        ui.painter()
            .circle_filled(rect.center(), DOT_SIZE / 2.0, color);
    }
    if response.contains_pointer() {
        ui.ctx().set_cursor_icon(CursorIcon::PointingHand);
    }
    response
}

#[cfg(test)]
mod tests {
    use view_model::title_bar::TitleBarViewModel;
    use view_model::window::WindowCommand;

    fn open() -> TitleBarViewModel {
        let mut vm = TitleBarViewModel::new();
        vm.open_menu();
        vm
    }

    #[test]
    fn menu_closed_by_default() {
        assert!(!TitleBarViewModel::new().menu_open());
    }

    #[test]
    fn selecting_item_closes_menu() {
        let mut vm = open();
        vm.select_menu_item(0);
        assert!(!vm.menu_open());
    }

    #[test]
    fn start_drag_closes_menu_and_emits_command() {
        let mut vm = open();
        vm.start_drag();
        assert!(!vm.menu_open());
        assert_eq!(vm.take_commands(), [WindowCommand::StartDrag]);
    }

    #[test]
    fn toggle_maximize_inverts_current_state() {
        let mut vm = TitleBarViewModel::new();
        vm.toggle_maximize();
        vm.set_maximized(true);
        vm.toggle_maximize();
        assert_eq!(
            vm.take_commands(),
            [
                WindowCommand::SetMaximized(true),
                WindowCommand::SetMaximized(false)
            ]
        );
    }

    #[test]
    fn commands_keep_order() {
        let mut vm = TitleBarViewModel::new();
        vm.minimize();
        vm.close_window();
        assert_eq!(vm.take_commands(), [WindowCommand::Minimize, WindowCommand::Close]);
    }

    #[test]
    fn take_commands_drains_queue() {
        let mut vm = TitleBarViewModel::new();
        vm.minimize();
        vm.take_commands();
        assert!(vm.take_commands().is_empty());
    }
}
