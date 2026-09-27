use std::path::PathBuf;

use eframe::egui::{self, Frame, Id, Modal};
use view_model::document::DocumentViewModel;

pub fn pick(ctx: &egui::Context, document: &mut DocumentViewModel, parent: Option<&eframe::Frame>) {
    if document.is_picking() {
        return;
    }
    let mut dialog = rfd::AsyncFileDialog::new().add_filter("Рефлектограммы", &["sor", "SOR"]);
    if cfg!(not(target_os = "linux"))
        && let Some(parent) = parent
    {
        dialog = dialog.set_parent(parent);
    }
    let dialog = dialog.pick_file();
    let ctx = ctx.clone();
    document.pick_and_open(
        move || pollster::block_on(dialog).map(|file| file.path().to_path_buf()),
        move || ctx.request_repaint(),
    );
}

pub fn block_while_picking(ctx: &egui::Context, document: &DocumentViewModel) {
    if document.is_picking() {
        Modal::new(Id::new("file-dialog-open"))
            .frame(Frame::NONE)
            .show(ctx, |_| {});
    }
}

pub fn open_dropped(ctx: &egui::Context, document: &mut DocumentViewModel) {
    if document.is_picking() {
        return;
    }
    let dropped = ctx.input(|i| i.raw.dropped_files.first().map(|f| f.path().to_path_buf()));
    if let Some(path) = dropped {
        open(ctx, document, path);
    }
}

pub fn files_hovered(ctx: &egui::Context) -> bool {
    ctx.input(|i| !i.raw.hovered_files.is_empty())
}

pub fn open(ctx: &egui::Context, document: &mut DocumentViewModel, path: PathBuf) {
    let ctx = ctx.clone();
    document.open(path, move || ctx.request_repaint());
}
