pub mod chart;
mod icons;
mod theme;
pub mod title_bar;
mod widgets;
mod window;
mod window_resize;

use eframe::egui;
use view_model::AppViewModel;

pub fn setup(ctx: &egui::Context) {
    egui_extras::install_image_loaders(ctx);
    theme::apply(ctx);
}

pub fn render(ui: &mut egui::Ui, vm: &mut AppViewModel) {
    let ctx = ui.ctx().clone();

    vm.title_bar.set_maximized(window::is_maximized(&ctx));

    title_bar::show(ui, &mut vm.title_bar);
    egui::CentralPanel::default()
        .frame(egui::Frame::new().fill(theme::BG_CONTENT))
        .show(ui, |_| {});

    for command in vm.title_bar.take_commands() {
        window::execute(&ctx, command);
    }
    window_resize::handle_window_resize(&ctx);
}

#[cfg(test)]
mod tests {
    use egui_kittest::Harness;
    use rstest::rstest;
    use view_model::AppViewModel;

    #[rstest]
    fn render_does_not_panic_and_drains_commands() {
        let mut vm = AppViewModel::new();
        vm.title_bar.minimize();

        let mut harness = Harness::new_ui(|ui| {
            crate::setup(ui.ctx());
            crate::render(ui, &mut vm);
        });
        harness.run();
        drop(harness);

        assert!(vm.title_bar.take_commands().is_empty());
    }
}
