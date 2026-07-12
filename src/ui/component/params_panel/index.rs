use leptos::prelude::*;
use shared_types::SorSummary;
use stylance::import_style;

import_style!(style, "index.module.css");

fn row(label: &'static str, value: String) -> impl IntoView {
    view! {
        <div class=style::row>
            <span class=style::row_label>{label}</span>
            <span class=style::row_value>{value}</span>
        </div>
    }
}

fn opt_f64(v: Option<f64>, suffix: &str, precision: usize) -> String {
    match v {
        Some(v) => format!("{:.*}{}", precision, v, suffix),
        None => "—".to_string(),
    }
}

fn opt_str(v: &Option<String>) -> String {
    v.clone()
        .filter(|s| !s.trim().is_empty())
        .unwrap_or_else(|| "—".to_string())
}

#[component]
pub fn ParamsPanel(summary: SorSummary) -> impl IntoView {
    let title = summary
        .cable_id
        .clone()
        .or_else(|| summary.fiber_id.clone())
        .unwrap_or_else(|| "Файл открыт".to_string());

    let wavelength_label = summary
        .wavelength_nm
        .map(|w| format!("{w:.0} нм"))
        .unwrap_or_else(|| "—".to_string());

    let avg_attenuation = match (summary.total_loss_db, summary.fiber_length_km) {
        (Some(loss), Some(len)) if len > 0.0 => format!("{:.2} дБ/км", loss / len),
        _ => "—".to_string(),
    };

    let max_reflection = summary
        .events
        .iter()
        .map(|e| e.refl_db)
        .filter(|v| v.abs() > f64::EPSILON)
        .reduce(f64::max)
        .map(|v| format!("{v:.1} дБ"))
        .unwrap_or_else(|| "—".to_string());

    let pulse_width = summary
        .pulse_widths_ns
        .first()
        .map(|p| format!("{p} нс"))
        .unwrap_or_else(|| "—".to_string());

    let events_count = summary.events.len().to_string();

    view! {
        <aside class=style::panel>
            <div class=style::header>
                <span class=style::title>"Параметры измерения"</span>
                <span class=style::subtitle>{title}" · "{wavelength_label.clone()}</span>
            </div>
            <div class=style::section>
                <div class=style::section_title>"ОБЪЕКТ"</div>
                {row("Дата измерения", "—".to_string())}
                {row("Оператор", opt_str(&summary.operator))}
                {row("Тип волокна", "—".to_string())}
            </div>
            <div class=style::section>
                <div class=style::section_title>"СВОДКА"</div>
                {row("Длина волокна", opt_f64(summary.fiber_length_km, " км", 3))}
                {row("Полные потери", opt_f64(summary.total_loss_db, " дБ", 2))}
                {row("Средн. затухание", avg_attenuation)}
                {row("ORL", opt_f64(summary.orl_db, " дБ", 1))}
                {row("Макс. отражение", max_reflection)}
                {row("Бюджет потерь", "—".to_string())}
                {row("Событий", events_count)}
            </div>
            <div class=style::section>
                <div class=style::section_title>"ПАРАМЕТРЫ ПРИБОРА"</div>
                {row("Длина волны", wavelength_label)}
                {row("Длит. импульса", pulse_width)}
                {row("Модель прибора", opt_str(&summary.otdr_model))}
                {row("Производитель", opt_str(&summary.otdr_supplier))}
                {row("Диапазон", "—".to_string())}
                {row("Разрешение", "—".to_string())}
                {row("Время усреднения", "—".to_string())}
                {row("Коэф. рассеяния", "—".to_string())}
            </div>
            <div class=style::spacer></div>
            <div class=style::actions>
                <button class=style::secondary_btn>"Отчёт"</button>
                <button class=style::primary_btn>"Экспорт"</button>
            </div>
        </aside>
    }
}
