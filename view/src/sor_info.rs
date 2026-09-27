use eframe::egui::{
    self, Align, Color32, Id, Layout, Margin, Modal, Rect, RichText, Sense, Shadow, Stroke,
    StrokeKind, Ui, vec2,
};

use crate::icons::{Icon, icon};
use crate::theme;
use crate::widgets::{self, IconButton, IconStyle};

const WIDTH: f32 = 466.0;
const RADIUS: u8 = 14;
const PADDING: i8 = 28;
const SHADOW: Shadow = Shadow {
    offset: [0, 20],
    blur: 60,
    spread: 0,
    color: Color32::from_black_alpha(64),
};
const HEADER_TILE: Tile = Tile {
    size: 52.0,
    radius: 12,
    icon_size: 24.0,
    fill: theme::ACCENT_SOFT,
    border: false,
};
const SECTION_TILE: Tile = Tile {
    size: 36.0,
    radius: 8,
    icon_size: 16.0,
    fill: theme::BG,
    border: true,
};
const CLOSE_STYLE: IconStyle = IconStyle {
    normal: theme::TEXT_FAINT,
    hover: theme::TEXT_HEADING,
    active: theme::TEXT_HEADING,
};
const TITLE_SIZE: f32 = 20.0;
const SUBTITLE_SIZE: f32 = 14.0;
const SECTION_TITLE_SIZE: f32 = 13.5;
const SECTION_TEXT_SIZE: f32 = 13.0;
const NOTE_SIZE: f32 = 12.0;
const SECTION_GAP: f32 = 14.0;

struct Section {
    icon: Icon,
    title: &'static str,
    text: &'static str,
}

const SECTIONS: [Section; 3] = [
    Section {
        icon: Icon::File,
        title: "Стандартный формат",
        text: "SOR — Standard OTDR Record, формат Telcordia SR-4731 (Bellcore). \
               Один файл хранит одно измерение рефлектометром.",
    },
    Section {
        icon: Icon::Chart,
        title: "Что внутри",
        text: "Кривая обратного рассеяния (точки dB по расстоянию), список событий, \
               параметры импульса и длины волны, данные о волокне и приборе.",
    },
    Section {
        icon: Icon::Target,
        title: "Зачем нужен",
        text: "Позволяет увидеть потери, отражения, сварные стыки, разъёмы и обрывы \
               вдоль всей оптической линии и измерить расстояние до них.",
    },
];

struct Tile {
    size: f32,
    radius: u8,
    icon_size: f32,
    fill: Color32,
    border: bool,
}

pub fn show(ctx: &egui::Context, open: &mut bool) {
    if !*open {
        return;
    }
    let frame = egui::Frame::new()
        .fill(theme::BG_SURFACE)
        .corner_radius(RADIUS)
        .shadow(SHADOW)
        .inner_margin(Margin::same(PADDING));
    let modal = Modal::new(Id::new("sor-info"))
        .frame(frame)
        .show(ctx, |ui| {
            ui.set_width(WIDTH);
            let mut close = header(ui);
            ui.add_space(18.0);
            intro(ui);
            ui.add_space(22.0);
            for (index, section) in SECTIONS.iter().enumerate() {
                if index > 0 {
                    divider(ui);
                }
                section_row(ui, section);
            }
            ui.add_space(26.0);
            close |= footer(ui);
            close
        });
    if modal.inner || modal.should_close() {
        *open = false;
    }
}

fn header(ui: &mut Ui) -> bool {
    ui.horizontal_top(|ui| {
        tile(ui, &HEADER_TILE, Icon::Info);
        ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
            let close = IconButton::new(Icon::Close)
                .style(CLOSE_STYLE)
                .size(28.0, 16.0)
                .flat();
            ui.add(close).on_hover_text("Закрыть").clicked()
        })
        .inner
    })
    .inner
}

fn intro(ui: &mut Ui) {
    ui.label(
        RichText::new("Что такое файл .sor?")
            .size(TITLE_SIZE)
            .strong()
            .color(theme::INK),
    );
    ui.add_space(6.0);
    ui.label(
        RichText::new("Файл с результатом измерения оптического волокна рефлектометром (OTDR).")
            .size(SUBTITLE_SIZE)
            .color(theme::TEXT_MUTED),
    );
}

fn section_row(ui: &mut Ui, section: &Section) {
    ui.horizontal_top(|ui| {
        ui.spacing_mut().item_spacing.x = 14.0;
        tile(ui, &SECTION_TILE, section.icon);
        ui.vertical(|ui| {
            ui.spacing_mut().item_spacing.y = 4.0;
            ui.label(
                RichText::new(section.title)
                    .size(SECTION_TITLE_SIZE)
                    .strong()
                    .color(theme::TEXT_HEADING),
            );
            ui.label(
                RichText::new(section.text)
                    .size(SECTION_TEXT_SIZE)
                    .color(theme::TEXT_MUTED),
            );
        });
    });
}

fn divider(ui: &mut Ui) {
    ui.add_space(SECTION_GAP);
    let (rect, _) = ui.allocate_exact_size(vec2(ui.available_width(), 1.0), Sense::hover());
    ui.painter()
        .hline(rect.x_range(), rect.center().y, Stroke::new(1.0, theme::BORDER));
    ui.add_space(SECTION_GAP);
}

fn footer(ui: &mut Ui) -> bool {
    ui.horizontal(|ui| {
        ui.label(
            RichText::new("Расширения: .sor · совместимо с EXFO, VIAVI, Yokogawa")
                .size(NOTE_SIZE)
                .color(theme::TEXT_FAINT),
        );
        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            widgets::primary_button(ui, None, "Понятно").clicked()
        })
        .inner
    })
    .inner
}

fn tile(ui: &mut Ui, style: &Tile, glyph: Icon) {
    let (rect, _) = ui.allocate_exact_size(vec2(style.size, style.size), Sense::hover());
    ui.painter().rect_filled(rect, style.radius, style.fill);
    if style.border {
        ui.painter().rect_stroke(
            rect,
            style.radius,
            Stroke::new(1.0, theme::BORDER),
            StrokeKind::Inside,
        );
    }
    let glyph_rect = Rect::from_center_size(rect.center(), vec2(style.icon_size, style.icon_size));
    icon(glyph, theme::ACCENT, style.icon_size).paint_at(ui, glyph_rect);
}
