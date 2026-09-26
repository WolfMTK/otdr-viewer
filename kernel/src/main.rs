use eframe::egui;
use view_model::AppViewModel;

struct App {
    view_model: AppViewModel,
}

impl App {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        egui_extras::install_image_loaders(&cc.egui_ctx);
        view::setup(&cc.egui_ctx);
        Self {
            view_model: AppViewModel::new(),
        }
    }
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        view::render(ui, &mut self.view_model);
    }
}

fn main() -> eframe::Result {
    let viewport = egui::ViewportBuilder::default()
        .with_title("OTDR Viewer")
        .with_inner_size([1200.0, 760.0])
        .with_min_inner_size([800.0, 600.0])
        .with_decorations(false);

    eframe::run_native(
        "OTDR Viewer",
        eframe::NativeOptions {
            viewport,
            ..Default::default()
        },
        Box::new(|cc| Ok(Box::new(App::new(cc)))),
    )
}
