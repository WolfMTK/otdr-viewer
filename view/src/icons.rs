use eframe::egui::{self, Color32, Image, ImageSource, Vec2};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Icon {
    Menu,
    FitView,
    ZoomIn,
    ZoomOut,
    Grid,
    Markers,
    Folder,
    Info,
    Upload,
    FolderOpen,
    Search,
    Trash,
    File,
    Collapse,
    Chart,
    Target,
    Close,
}

impl Icon {
    fn source(self) -> ImageSource<'static> {
        match self {
            Icon::Menu => egui::include_image!("../assets/icons/menu.svg"),
            Icon::FitView => egui::include_image!("../assets/icons/fit-view.svg"),
            Icon::ZoomIn => egui::include_image!("../assets/icons/zoom-in.svg"),
            Icon::ZoomOut => egui::include_image!("../assets/icons/zoom-out.svg"),
            Icon::Grid => egui::include_image!("../assets/icons/grid.svg"),
            Icon::Markers => egui::include_image!("../assets/icons/markers.svg"),
            Icon::Folder => egui::include_image!("../assets/icons/folder.svg"),
            Icon::Info => egui::include_image!("../assets/icons/info.svg"),
            Icon::Upload => egui::include_image!("../assets/icons/upload.svg"),
            Icon::FolderOpen => egui::include_image!("../assets/icons/folder-open.svg"),
            Icon::Search => egui::include_image!("../assets/icons/search.svg"),
            Icon::Trash => egui::include_image!("../assets/icons/trash.svg"),
            Icon::File => egui::include_image!("../assets/icons/file.svg"),
            Icon::Collapse => egui::include_image!("../assets/icons/collapse.svg"),
            Icon::Chart => egui::include_image!("../assets/icons/chart.svg"),
            Icon::Target => egui::include_image!("../assets/icons/target.svg"),
            Icon::Close => egui::include_image!("../assets/icons/close.svg"),
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
    const FIT_VIEW: &str = include_str!("../assets/icons/fit-view.svg");
    const ZOOM_IN: &str = include_str!("../assets/icons/zoom-in.svg");
    const ZOOM_OUT: &str = include_str!("../assets/icons/zoom-out.svg");
    const GRID: &str = include_str!("../assets/icons/grid.svg");
    const MARKERS: &str = include_str!("../assets/icons/markers.svg");
    const FOLDER: &str = include_str!("../assets/icons/folder.svg");
    const INFO: &str = include_str!("../assets/icons/info.svg");
    const UPLOAD: &str = include_str!("../assets/icons/upload.svg");
    const FOLDER_OPEN: &str = include_str!("../assets/icons/folder-open.svg");
    const SEARCH: &str = include_str!("../assets/icons/search.svg");
    const TRASH: &str = include_str!("../assets/icons/trash.svg");
    const FILE: &str = include_str!("../assets/icons/file.svg");
    const COLLAPSE: &str = include_str!("../assets/icons/collapse.svg");
    const CHART: &str = include_str!("../assets/icons/chart.svg");
    const TARGET: &str = include_str!("../assets/icons/target.svg");
    const CLOSE: &str = include_str!("../assets/icons/close.svg");

    #[rstest]
    #[case::menu(MENU)]
    #[case::fit_view(FIT_VIEW)]
    #[case::zoom_in(ZOOM_IN)]
    #[case::zoom_out(ZOOM_OUT)]
    #[case::grid(GRID)]
    #[case::markers(MARKERS)]
    #[case::folder(FOLDER)]
    #[case::info(INFO)]
    #[case::upload(UPLOAD)]
    #[case::folder_open(FOLDER_OPEN)]
    #[case::search(SEARCH)]
    #[case::trash(TRASH)]
    #[case::file(FILE)]
    #[case::collapse(COLLAPSE)]
    #[case::chart(CHART)]
    #[case::target(TARGET)]
    #[case::close(CLOSE)]
    fn asset_is_valid_svg(#[case] svg: &str) {
        usvg::Tree::from_str(svg, &usvg::Options::default()).expect("SVG не парсится");
    }

    #[rstest]
    #[case::menu(MENU)]
    #[case::fit_view(FIT_VIEW)]
    #[case::zoom_in(ZOOM_IN)]
    #[case::zoom_out(ZOOM_OUT)]
    #[case::grid(GRID)]
    #[case::markers(MARKERS)]
    #[case::folder(FOLDER)]
    #[case::info(INFO)]
    #[case::upload(UPLOAD)]
    #[case::folder_open(FOLDER_OPEN)]
    #[case::search(SEARCH)]
    #[case::trash(TRASH)]
    #[case::file(FILE)]
    #[case::collapse(COLLAPSE)]
    #[case::chart(CHART)]
    #[case::target(TARGET)]
    #[case::close(CLOSE)]
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
