use eframe::egui::{self, ViewportCommand};
use view_model::window::WindowCommand;

pub fn is_maximized(ctx: &egui::Context) -> bool {
    ctx.input(|i| {
        let vp = i.viewport();
        vp.maximized.unwrap_or(false) || vp.fullscreen.unwrap_or(false)
    })
}

pub fn execute(ctx: &egui::Context, command: WindowCommand) {
    match command {
        WindowCommand::Minimize => ctx.send_viewport_cmd(ViewportCommand::Minimized(true)),
        WindowCommand::SetMaximized(on) => ctx.send_viewport_cmd(ViewportCommand::Maximized(on)),
        WindowCommand::StartDrag => {
            ctx.send_viewport_cmd(ViewportCommand::StartDrag);
            // OS owns the drag now
            ctx.stop_dragging();
        }
        WindowCommand::Close => ctx.send_viewport_cmd(ViewportCommand::Close),
    }
}
