use crate::ui::trace_math::{level_at, linspace, nice_step, svg_num as f};
use leptos::prelude::*;
use shared_types::SorEvent;
use std::fmt::Write as _;
use stylance::import_style;

import_style!(style, "index.module.css");

const VIEW_W: f64 = 1000.0;
const VIEW_H: f64 = 460.0;
const PAD_LEFT: f64 = 46.0;
const PAD_RIGHT: f64 = 12.0;
const PAD_TOP: f64 = 22.0;
const PAD_BOTTOM: f64 = 26.0;

fn build_path(
    distances_km: &[f64],
    levels_db: &[f64],
    map_x: impl Fn(f64) -> f64,
    map_y: impl Fn(f64) -> f64,
) -> String {
    distances_km.iter().zip(levels_db).enumerate().fold(
        String::with_capacity(distances_km.len() * 14),
        |mut path, (i, (&d, &v))| {
            let (x, y) = (map_x(d), map_y(v));
            if i == 0 {
                let _ = write!(path, "M{x:.1} {y:.1}");
            } else {
                let _ = write!(path, " L{x:.1} {y:.1}");
            }
            path
        },
    )
}

#[component]
pub fn Chart(distances_km: Vec<f64>, levels_db: Vec<f64>, events: Vec<SorEvent>) -> impl IntoView {
    if distances_km.is_empty() || levels_db.is_empty() {
        return view! { <div class=style::empty>"Нет данных трассы для отображения"</div> }.into_any();
    }

    let d_min = 0.0_f64;
    let d_max = distances_km.iter().cloned().fold(f64::MIN, f64::max).max(0.001);

    let (v_min_raw, v_max_raw) = levels_db
        .iter()
        .fold((f64::MAX, f64::MIN), |(min, max), &v| (min.min(v), max.max(v)));
    let v_pad = ((v_max_raw - v_min_raw) * 0.06).max(0.5);
    let v_max = v_max_raw + v_pad;
    let v_min = v_min_raw - v_pad;

    let plot_w = VIEW_W - PAD_LEFT - PAD_RIGHT;
    let plot_h = VIEW_H - PAD_TOP - PAD_BOTTOM;

    let map_x = move |d: f64| PAD_LEFT + (d - d_min) / (d_max - d_min) * plot_w;
    let map_y = move |v: f64| PAD_TOP + (v - v_min) / (v_max - v_min) * plot_h;

    let path = build_path(&distances_km, &levels_db, map_x, map_y);

    let y_ticks = linspace(v_min_raw, v_max_raw, 6);
    let x_ticks = linspace(d_min, d_max, 6);

    let y_grid = y_ticks
        .iter()
        .map(|&v| {
            let y = map_y(v);
            view! {
                <line x1=f(PAD_LEFT) x2=f(VIEW_W - PAD_RIGHT) y1=f(y) y2=f(y) class=style::grid_line />
                <text x=f(PAD_LEFT - 8.0) y=f(y + 3.0) class=style::y_label text-anchor="end">
                    {format!("{v:.0}")}
                </text>
            }
        })
        .collect_view();

    let x_grid = x_ticks
        .iter()
        .map(|&d| {
            let x = map_x(d);
            view! {
                <line x1=f(x) x2=f(x) y1=f(PAD_TOP) y2=f(VIEW_H - PAD_BOTTOM) class=style::grid_line />
                <text x=f(x) y=f(VIEW_H - PAD_BOTTOM + 16.0) class=style::x_label text-anchor="middle">
                    {format!("{d:.1}")}
                </text>
            }
        })
        .collect_view();

    let top_step = nice_step(d_max - d_min);
    let top_ticks = std::iter::successors(Some(top_step), move |&v| Some(v + top_step))
        .take_while(|&v| v < d_max)
        .map(|d| {
            let x = map_x(d);
            view! {
                <line x1=f(x) x2=f(x) y1=f(PAD_TOP - 8.0) y2=f(PAD_TOP) class=style::top_tick />
                <text x=f(x) y=f(PAD_TOP - 11.0) class=style::top_tick_label text-anchor="middle">
                    {format!("{d:.0}")}
                </text>
            }
        })
        .collect_view();

    let mut candidates: Vec<&SorEvent> = events.iter().filter(|e| e.distance_km > 0.05).collect();
    candidates.sort_by(|a, b| a.distance_km.partial_cmp(&b.distance_km).unwrap());
    let ab: Option<(f64, f64)> = match candidates.len() {
        0 | 1 => None,
        n => Some((candidates[0].distance_km, candidates[n - 1].distance_km)),
    };

    let ab_overlay = ab.map(|(da, db)| {
        let xa = map_x(da);
        let xb = map_x(db);
        let badge = move |x: f64, label: &'static str| {
            view! {
                <rect x=f(x - 10.0) y=f(PAD_TOP - 18.0) width="20" height="16" rx="4" class=style::ab_badge />
                <text x=f(x) y=f(PAD_TOP - 6.5) class=style::ab_badge_label text-anchor="middle">{label}</text>
            }
        };
        view! {
            <rect
                x=f(xa.min(xb)) y=f(PAD_TOP)
                width=f((xb - xa).abs()) height=f(VIEW_H - PAD_TOP - PAD_BOTTOM)
                class=style::ab_band
            />
            <line x1=f(xa) x2=f(xa) y1=f(PAD_TOP) y2=f(VIEW_H - PAD_BOTTOM) class=style::ab_line />
            <line x1=f(xb) x2=f(xb) y1=f(PAD_TOP) y2=f(VIEW_H - PAD_BOTTOM) class=style::ab_line />
            {badge(xa, "A")}
            {badge(xb, "B")}
        }
    });

    let tooltip = ab.and_then(|(da, db)| {
        let loss = level_at(&distances_km, &levels_db, da)
            .zip(level_at(&distances_km, &levels_db, db))
            .map(|(la, lb)| la - lb);
        let span_km = db - da;
        let atten = loss.filter(|_| span_km > 0.0).map(|l| l / span_km);
        loss.map(|loss| {
            view! {
                <div class=style::tooltip>
                    <div class=style::tooltip_row>
                        <span class=style::tooltip_label>"A → B"</span>
                        <span class=style::tooltip_value>{format!("{span_km:.2} км")}</span>
                    </div>
                    <div class=style::tooltip_row>
                        <span class=style::tooltip_label>"Потери"</span>
                        <span class=style::tooltip_value>{format!("{loss:.2} дБ")}</span>
                    </div>
                    <div class=style::tooltip_row>
                        <span class=style::tooltip_label>"Затухание"</span>
                        <span class=style::tooltip_value>
                            {atten.map(|a| format!("{a:.3} дБ/км")).unwrap_or_else(|| "—".to_string())}
                        </span>
                    </div>
                </div>
            }
        })
    });

    view! {
        <div class=style::chart_wrap>
            <svg
                class=style::chart_svg
                viewBox=format!("0 0 {VIEW_W} {VIEW_H}")
                preserveAspectRatio="none"
            >
                <rect
                    x=f(PAD_LEFT) y=f(PAD_TOP)
                    width=f(plot_w) height=f(plot_h)
                    class=style::plot_bg
                />
                {y_grid}
                {x_grid}
                {ab_overlay}
                {top_ticks}
                <path d=path class=style::trace_line />
            </svg>

        {tooltip}
        </div>
    }
    .into_any()
}
