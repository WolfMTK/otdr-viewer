use crate::chart_view::ChartSettings;
use crate::document::DocumentViewModel;
use crate::title_bar::TitleBarViewModel;

pub mod chart_view;
pub mod document;
pub mod format;
pub mod title_bar;
pub mod window;

#[derive(Debug, Default)]
pub struct AppViewModel {
    pub title_bar: TitleBarViewModel,
    pub document: DocumentViewModel,
    pub chart_settings: ChartSettings,
}

impl AppViewModel {
    pub fn new() -> Self {
        Self::default()
    }
}
