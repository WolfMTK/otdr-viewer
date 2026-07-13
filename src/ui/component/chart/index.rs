use std::fmt::Write as _;

use leptos::prelude::*;
use leptos::web_sys;
use shared_types::SorEvent;
use stylance::import_style;
use wasm_bindgen::JsCast;

use crate::ui::context::ChartView;
use crate::ui::trace_math::{clamp_window, level_at, linspace, nice_step, svg_num as f};

import_style!(style, "index.module.css");

const VIEW_W: f64 = 1000.0;
const VIEW_H: f64 = 460.0;
const PAD_LEFT: f64 = 46.0;
const PAD_RIGHT: f64 = 12.0;
const PAD_TOP: f64 = 22.0;
const PAD_BOTTOM: f64 = 26.0;
const PLOT_W: f64 = VIEW_W - PAD_LEFT - PAD_RIGHT;
const PLOT_H: f64 = VIEW_H - PAD_TOP - PAD_BOTTOM;
const WHEEL_STEP: f64 = 1.2;

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

fn pointer_km(target: Option<web_sys::EventTarget>, client_x: f64, window: (f64, f64)) -> Option<f64> {
    let el = target?.dyn_into::<web_sys::Element>().ok()?;
    let rect = el.get_bounding_client_rect();
    let width = rect.width();
    if width <= 0.0 {
        return None;
    }
    let frac = ((client_x - rect.left()) / width).clamp(0.0, 1.0);
    let vb_x = frac * VIEW_W;
    let plot_frac = ((vb_x - PAD_LEFT) / PLOT_W).clamp(0.0, 1.0);
    Some(window.0 + plot_frac * (window.1 - window.0))
}

fn movement_km(target: Option<web_sys::EventTarget>, movement_x: f64, window: (f64, f64)) -> Option<f64> {
    let el = target?.dyn_into::<web_sys::Element>().ok()?;
    let width = el.get_bounding_client_rect().width();
    if width <= 0.0 {
        return None;
    }
    let vb_dx = (movement_x / width) * VIEW_W;
    let plot_dfrac = vb_dx / PLOT_W;
    Some(plot_dfrac * (window.1 - window.0))
}

#[component]
pub fn Chart(distances_km: Vec<f64>, levels_db: Vec<f64>, events: Vec<SorEvent>) -> impl IntoView {
    if distances_km.is_empty() || levels_db.is_empty() {
        return view! { <div class=style::empty>"Нет данных трассы для отображения"</div> }.into_any();
    }

    let chart_view = ChartView::use_context();

    let d_min = 0.0_f64;
    let d_max = distances_km.iter().cloned().fold(f64::MIN, f64::max).max(0.001);

    Effect::new(move |_| {
        chart_view.full.set((d_min, d_max));
        chart_view.view.set(None);
    });

    let (v_min_raw, v_max_raw) = levels_db
        .iter()
        .fold((f64::MAX, f64::MIN), |(min, max), &v| (min.min(v), max.max(v)));
    let v_pad = ((v_max_raw - v_min_raw) * 0.06).max(0.5);
    let v_max = v_max_raw + v_pad;
    let v_min = v_min_raw - v_pad;

    let window = Memo::new(move |_| clamp_window(chart_view.window(), (d_min, d_max)));
    let map_x = move |d: f64| {
        let (lo, hi) = window.get();
        let span = (hi - lo).max(f64::EPSILON);
        PAD_LEFT + (d - lo) / span * PLOT_W
    };
    let map_y = move |v: f64| PAD_TOP + (v - v_min) / (v_max - v_min) * PLOT_H;

    let dist_path = distances_km.clone();
    let lvl_path = levels_db.clone();
    let path = move || {
        let (lo, hi) = window.get();
        let start = dist_path.partition_point(|&d| d < lo).saturating_sub(1);
        let end = (dist_path.partition_point(|&d| d <= hi) + 1).min(dist_path.len());
        if start >= end {
            return String::new();
        }
        build_path(&dist_path[start..end], &lvl_path[start..end], map_x, map_y)
    };

    let y_ticks = linspace(v_min_raw, v_max_raw, 6);
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

    let x_grid = move || {
        let (lo, hi) = window.get();
        linspace(lo, hi, 6)
            .into_iter()
            .map(|d| {
                let x = map_x(d);
                view! {
                    <line x1=f(x) x2=f(x) y1=f(PAD_TOP) y2=f(VIEW_H - PAD_BOTTOM) class=style::grid_line />
                    <text x=f(x) y=f(VIEW_H - PAD_BOTTOM + 16.0) class=style::x_label text-anchor="middle">
                        {format!("{d:.1}")}
                    </text>
                }
            })
            .collect_view()
    };

    let top_ticks = move || {
        let (lo, hi) = window.get();
        let top_step = nice_step(hi - lo);
        let first = (lo / top_step).ceil() * top_step;
        std::iter::successors(Some(first), move |&v| Some(v + top_step))
            .take_while(move |&v| v < hi)
            .filter(move |&v| v > lo)
            .map(|d| {
                let x = map_x(d);
                view! {
                    <line x1=f(x) x2=f(x) y1=f(PAD_TOP - 8.0) y2=f(PAD_TOP) class=style::top_tick />
                    <text x=f(x) y=f(PAD_TOP - 11.0) class=style::top_tick_label text-anchor="middle">
                        {format!("{d:.0}")}
                    </text>
                }
            })
            .collect_view()
    };

    let mut candidates: Vec<&SorEvent> = events.iter().filter(|e| e.distance_km > 0.05).collect();
    candidates.sort_by(|a, b| a.distance_km.partial_cmp(&b.distance_km).unwrap());
    let ab: Option<(f64, f64)> = match candidates.len() {
        0 | 1 => None,
        n => Some((candidates[0].distance_km, candidates[n - 1].distance_km)),
    };

    let ab_overlay = ab.map(|(da, db)| {
        move || {
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

    let dragging = RwSignal::new(false);

    let on_wheel = move |ev: web_sys::WheelEvent| {
        ev.prevent_default();
        let factor = if ev.delta_y() < 0.0 {
            WHEEL_STEP
        } else {
            1.0 / WHEEL_STEP
        };
        let center =
            pointer_km(ev.current_target(), ev.client_x() as f64, window.get_untracked()).unwrap_or_else(|| {
                let (lo, hi) = window.get_untracked();
                (lo + hi) / 2.0
            });
        chart_view.zoom_at(factor, center);
    };

    let on_mouse_down = move |ev: web_sys::MouseEvent| {
        ev.prevent_default();
        dragging.set(true);
    };
    let on_mouse_move = move |ev: web_sys::MouseEvent| {
        if !dragging.get_untracked() {
            return;
        }
        if let Some(delta) = movement_km(ev.current_target(), ev.movement_x() as f64, window.get_untracked()) {
            chart_view.pan(-delta);
        }
    };
    let stop_drag = move |_: web_sys::MouseEvent| dragging.set(false);

    let cursor_class = move || {
        if dragging.get() {
            stylance::classes!(style::chart_svg, style::grabbing)
        } else {
            style::chart_svg.to_string()
        }
    };

    view! {
        <div class=style::chart_wrap>
            <svg
                class=cursor_class
                viewBox=format!("0 0 {VIEW_W} {VIEW_H}")
                preserveAspectRatio="none"
                on:wheel=on_wheel
                on:mousedown=on_mouse_down
                on:mousemove=on_mouse_move
                on:mouseup=stop_drag
                on:mouseleave=stop_drag
            >
                <defs>
                    <clipPath id="chart-plot-clip">
                        <rect x=f(PAD_LEFT) y=f(PAD_TOP) width=f(PLOT_W) height=f(PLOT_H) />
                    </clipPath>
                </defs>
                <rect
                    x=f(PAD_LEFT) y=f(PAD_TOP)
                    width=f(PLOT_W) height=f(PLOT_H)
                    class=style::plot_bg
                />
                {y_grid}
                {move || x_grid()}
                {ab_overlay.map(|f| view! { <g clip-path="url(#chart-plot-clip)">{move || f()}</g> })}
                {move || top_ticks()}
                <path clip-path="url(#chart-plot-clip)" d=move || path() class=style::trace_line />
            </svg>

        {tooltip}
        </div>
    }
    .into_any()
}
