use eframe::egui::{self, Color32, Image, ImageSource, Vec2};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Icon {
    Menu,
}

impl Icon {
    fn source(self) -> ImageSource<'static> {
        match self {
            Icon::Menu => egui::include_image!("../assets/icons/menu.svg"),
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

    #[rstest]
    #[case::menu(MENU)]
    #[case::logo(LOGO)]
    fn asset_is_valid_svg(#[case] svg: &str) {
        usvg::Tree::from_str(svg, &usvg::Options::default()).expect("SVG не парсится");
    }

    #[rstest]
    #[case::menu(MENU)]
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
