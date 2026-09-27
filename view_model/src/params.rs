use model::types::SorSummary;

use crate::format::format_length_km;

const NO_VALUE: &str = "—";

#[derive(Debug, Clone, PartialEq)]
pub struct ParamSection {
    pub title: &'static str,
    pub rows: Vec<(&'static str, String)>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Params {
    pub title: String,
    pub subtitle: String,
    pub sections: Vec<ParamSection>,
}

pub fn params(summary: &SorSummary) -> Params {
    let wavelength = summary
        .wavelength_nm
        .map_or_else(dash, |w| format!("{w:.0} нм"));

    Params {
        title: non_empty(&summary.cable_id)
            .or_else(|| non_empty(&summary.fiber_id))
            .unwrap_or("Файл открыт")
            .to_string(),
        subtitle: wavelength.clone(),
        sections: vec![
            ParamSection {
                title: "ОБЪЕКТ",
                rows: vec![
                    ("Кабель", text(&summary.cable_id)),
                    ("Волокно", text(&summary.fiber_id)),
                    ("Оператор", text(&summary.operator)),
                    ("Комментарий", text(&summary.comments)),
                ],
            },
            ParamSection {
                title: "СВОДКА",
                rows: vec![
                    ("Длина волокна", summary.fiber_length_km.map_or_else(dash, format_length_km)),
                    ("Полные потери", number(summary.total_loss_db, 2, " дБ")),
                    ("Средн. затухание", average_attenuation(summary)),
                    ("ORL", number(summary.orl_db, 1, " дБ")),
                    ("Макс. отражение", max_reflection(summary)),
                    ("Событий", summary.events.len().to_string()),
                ],
            },
            ParamSection {
                title: "ПАРАМЕТРЫ ПРИБОРА",
                rows: vec![
                    ("Длина волны", wavelength),
                    ("Длит. импульса", pulse_widths(&summary.pulse_widths_ns)),
                    ("Модель", text(&summary.otdr_model)),
                    ("Производитель", text(&summary.otdr_supplier)),
                    ("Серийный номер", text(&summary.otdr_serial)),
                    ("Точек данных", summary.num_data_points.map_or_else(dash, |n| n.to_string())),
                ],
            },
        ],
    }
}

fn dash() -> String {
    NO_VALUE.to_string()
}

fn non_empty(value: &Option<String>) -> Option<&str> {
    value.as_deref().map(str::trim).filter(|s| !s.is_empty())
}

fn text(value: &Option<String>) -> String {
    non_empty(value).map_or_else(dash, str::to_string)
}

fn number(value: Option<f64>, precision: usize, suffix: &str) -> String {
    value.map_or_else(dash, |v| format!("{v:.precision$}{suffix}"))
}

fn average_attenuation(summary: &SorSummary) -> String {
    match (summary.total_loss_db, summary.fiber_length_km) {
        (Some(loss), Some(length)) if length > 0.0 => format!("{:.3} дБ/км", loss / length),
        _ => dash(),
    }
}

fn max_reflection(summary: &SorSummary) -> String {
    summary
        .events
        .iter()
        .map(|e| e.refl_db)
        .filter(|r| r.abs() > f64::EPSILON)
        .reduce(f64::max)
        .map_or_else(dash, |r| format!("{r:.1} дБ"))
}

fn pulse_widths(widths_ns: &[u16]) -> String {
    if widths_ns.is_empty() {
        return dash();
    }
    let joined = widths_ns
        .iter()
        .map(u16::to_string)
        .collect::<Vec<_>>()
        .join(", ");
    format!("{joined} нс")
}

#[cfg(test)]
mod tests {
    use model::types::{SorEvent, SorSummary};

    use crate::params::params;

    fn value(summary: &SorSummary, label: &str) -> String {
        params(summary)
            .sections
            .into_iter()
            .flat_map(|s| s.rows)
            .find(|(l, _)| *l == label)
            .map(|(_, v)| v)
            .unwrap_or_else(|| panic!("no row {label}"))
    }

    fn event(refl_db: f64) -> SorEvent {
        SorEvent {
            number: 1,
            distance_km: 1.0,
            loss_db: 0.1,
            refl_db,
            kind: String::new(),
            comments: String::new(),
        }
    }

    #[test]
    fn title_falls_back_from_cable_to_fiber_to_placeholder() {
        let mut summary = SorSummary {
            cable_id: Some("  ".into()),
            fiber_id: Some("F-7".into()),
            ..Default::default()
        };
        assert_eq!(params(&summary).title, "F-7");
        summary.fiber_id = None;
        assert_eq!(params(&summary).title, "Файл открыт");
    }

    #[test]
    fn average_attenuation_divides_loss_by_length() {
        let summary = SorSummary {
            total_loss_db: Some(3.0),
            fiber_length_km: Some(12.0),
            ..Default::default()
        };
        assert_eq!(value(&summary, "Средн. затухание"), "0.250 дБ/км");
    }

    #[test]
    fn average_attenuation_needs_positive_length() {
        let summary = SorSummary {
            total_loss_db: Some(3.0),
            fiber_length_km: Some(0.0),
            ..Default::default()
        };
        assert_eq!(value(&summary, "Средн. затухание"), "—");
    }

    #[test]
    fn max_reflection_ignores_missing_values() {
        let summary = SorSummary {
            events: vec![event(-50.0), event(0.0), event(-38.5)],
            ..Default::default()
        };
        assert_eq!(value(&summary, "Макс. отражение"), "-38.5 дБ");
    }

    #[test]
    fn all_pulse_widths_are_listed() {
        let summary = SorSummary {
            pulse_widths_ns: vec![100, 1000],
            ..Default::default()
        };
        assert_eq!(value(&summary, "Длит. импульса"), "100, 1000 нс");
    }
}
