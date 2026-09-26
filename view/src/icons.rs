use eframe::egui::{self, Color32, Image, ImageSource, Vec2};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Icon {
    Menu,
    FitView,
    ZoomIn,
    ZoomOut,
    Grid,
}

impl Icon {
    fn source(self) -> ImageSource<'static> {
        match self {
            Icon::Menu => egui::include_image!("../assets/icons/menu.svg"),
            Icon::FitView => egui::include_image!("../assets/icons/fit-view.svg"),
            Icon::ZoomIn => egui::include_image!("../assets/icons/zoom-in.svg"),
            Icon::ZoomOut => egui::include_image!("../assets/icons/zoom-out.svg"),
            Icon::Grid => egui::include_image!("../assets/icons/grid.svg"),
        }
    }
}

pub fn icon(icon: Icon, color: Color32, size: f32) -> Image<'static> {
    Image::new(icon.source())
        .tint(color)
        .fit_to_exact_size(Vec2::splat(size))
}

pub fn logo(size: f32) -> Image<'static> {
    Image::new(egui::include_image!("../assets/icons/logo.svg"))
        .fit_to_exact_size(Vec2::splat(size))
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    const MENU: &str = include_str!("../assets/icons/menu.svg");
    const LOGO: &str = include_str!("../assets/icons/logo.svg");
    const FIT_VIEW: &str = include_str!("../assets/icons/fit-view.svg");
    const ZOOM_IN: &str = include_str!("../assets/icons/zoom-in.svg");
    const ZOOM_OUT: &str = include_str!("../assets/icons/zoom-out.svg");
    const GRID: &str = include_str!("../assets/icons/grid.svg");

    #[rstest]
    #[case::menu(MENU)]
    #[case::logo(LOGO)]
    #[case::fit_view(FIT_VIEW)]
    #[case::zoom_in(ZOOM_IN)]
    #[case::zoom_out(ZOOM_OUT)]
    #[case::grid(GRID)]
    fn asset_is_valid_svg(#[case] svg: &str) {
        usvg::Tree::from_str(svg, &usvg::Options::default()).expect("SVG не парсится");
    }

    #[rstest]
    #[case::menu(MENU)]
    #[case::fit_view(FIT_VIEW)]
    #[case::zoom_in(ZOOM_IN)]
    #[case::zoom_out(ZOOM_OUT)]
    #[case::grid(GRID)]
    fn tintable_icon_is_white(#[case] svg: &str) {
        let lower = svg.to_lowercase();
        assert!(
            lower.contains("fill=\"#ffffff\"")
                || lower.contains("fill=\"#fff\"")
                || lower.contains("fill=\"white\""),
            "icon used with tint must be white"
        );
    }
}
