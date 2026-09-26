use crate::title_bar::TitleBarViewModel;

pub mod chart_view;
pub mod format;
pub mod title_bar;
pub mod window;

#[derive(Debug, Default)]
pub struct AppViewModel {
    pub title_bar: TitleBarViewModel,
}

impl AppViewModel {
    pub fn new() -> Self {
        Self::default()
    }
}
