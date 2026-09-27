use eframe::egui;
use view_model::AppViewModel;

const RECENT_FILES_KEY: &str = "recent_files";

struct App {
    view_model: AppViewModel,
}

impl App {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        view::setup(&cc.egui_ctx);
        let mut view_model = AppViewModel::new();
        if let Some(files) = cc
            .storage
            .and_then(|storage| eframe::get_value(storage, RECENT_FILES_KEY))
        {
            view_model.recent_files.restore(files);
        }
        Self { view_model }
    }
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        view::render(ui, &mut self.view_model, Some(&*frame));
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        let files = self.view_model.recent_files.files().to_vec();
        eframe::set_value(storage, RECENT_FILES_KEY, &files);
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
            event_loop_builder: prefer_x11(),
            ..Default::default()
        },
        Box::new(|cc| Ok(Box::new(App::new(cc)))),
    )
}

#[cfg(target_os = "linux")]
fn prefer_x11() -> Option<eframe::EventLoopBuilderHook> {
    std::env::var_os("DISPLAY")?;
    Some(Box::new(|builder| {
        use winit::platform::x11::EventLoopBuilderExtX11;
        builder.with_x11();
    }))
}

#[cfg(not(target_os = "linux"))]
fn prefer_x11() -> Option<eframe::EventLoopBuilderHook> {
    None
}
