mod chart;
mod events_table;
mod icons;
mod open_file;
mod params_panel;
mod recent_files;
mod sidebar;
mod sor_info;
mod start_screen;
mod theme;
pub mod title_bar;
mod toolbar;
mod widgets;
mod window;
mod window_resize;

use eframe::egui;
use view_model::AppViewModel;

pub fn setup(ctx: &egui::Context) {
    egui_extras::install_image_loaders(ctx);
    theme::apply(ctx);
}

pub fn render(ui: &mut egui::Ui, vm: &mut AppViewModel, parent_window: Option<&eframe::Frame>) {
    let ctx = ui.ctx().clone();

    vm.update();
    vm.title_bar.set_maximized(window::is_maximized(&ctx));

    title_bar::show(ui, &mut vm.title_bar);
    let mut open_requested = vm.title_bar.take_open_file_request();
    open_file::open_dropped(&ctx, &mut vm.document);

    sidebar::show(ui, &mut vm.recent_files.panel_open, &mut vm.sor_info_open);
    if vm.recent_files.panel_open
        && let Some(path) = recent_files::show(ui, &mut vm.recent_files)
        && !vm.document.is_picking()
    {
        open_file::open(&ctx, &mut vm.document, path);
    }

    if let Some(file) = vm.document.opened_mut() {
        toolbar::show(ui, file.chart_mut(), &mut vm.chart_settings);
        params_panel::show(ui, file.params());
        events_table::show(ui, file.events());
    }

    let files_hovered = open_file::files_hovered(&ctx);
    let settings = vm.chart_settings;
    egui::CentralPanel::default()
        .frame(egui::Frame::new().fill(theme::BG_CONTENT))
        .show(ui, |ui| match vm.document.opened_mut() {
            Some(file) => chart::show(ui, file.chart_parts_mut(), &settings),
            None => {
                open_requested |= start_screen::show(
                    ui,
                    vm.document.state(),
                    files_hovered,
                    &mut vm.sor_info_open,
                );
            }
        });

    if open_requested {
        open_file::pick(&ctx, &mut vm.document, parent_window);
    }
    open_file::block_while_picking(&ctx, &vm.document);
    sor_info::show(&ctx, &mut vm.sor_info_open);

    for command in vm.title_bar.take_commands() {
        window::execute(&ctx, command);
    }
    if !vm.document.is_picking() {
        window_resize::handle_window_resize(&ctx);
    }
}

#[cfg(test)]
mod tests {
    use egui_kittest::Harness;
    use view_model::AppViewModel;

    #[test]
    fn render_does_not_panic_and_drains_commands() {
        let mut vm = AppViewModel::new();
        vm.title_bar.minimize();

        let mut harness = Harness::new_ui(|ui| {
            crate::setup(ui.ctx());
            crate::render(ui, &mut vm, None);
        });
        harness.run();
        drop(harness);

        assert!(vm.title_bar.take_commands().is_empty());
    }
}
