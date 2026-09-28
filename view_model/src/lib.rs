use std::time::SystemTime;

use crate::chart_view::ChartSettings;
use crate::document::DocumentViewModel;
use crate::recent_files::RecentFilesViewModel;
use crate::title_bar::TitleBarViewModel;

pub mod chart_view;
pub mod document;
pub mod drop;
pub mod events;
pub mod format;
pub mod ideal;
pub mod markers;
pub mod params;
pub mod recent_files;
pub mod title_bar;
pub mod window;

#[derive(Debug, Default)]
pub struct AppViewModel {
    pub title_bar: TitleBarViewModel,
    pub document: DocumentViewModel,
    pub chart_settings: ChartSettings,
    pub recent_files: RecentFilesViewModel,
    pub sor_info_open: bool,
}

impl AppViewModel {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn update(&mut self) {
        if self.document.poll()
            && let Some(file) = self.document.opened()
        {
            let length_km = file.data().summary.fiber_length_km;
            self.recent_files
                .record(file.path(), length_km, SystemTime::now());
        }
    }
}
