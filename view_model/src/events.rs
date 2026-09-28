use model::types::{SorEvent, SorTrace};

const NO_VALUE: &str = "—";

#[derive(Debug, Clone, PartialEq)]
pub struct EventRow {
    pub number: u16,
    pub kind: &'static str,
    pub distance: String,
    pub loss: String,
    pub reflection: String,
    pub attenuation: String,
}

pub fn event_kind_label(kind: &str, is_first: bool) -> &'static str {
    match kind {
        "reflection" if is_first => "Разъём (старт)",
        "reflection" => "Разъём",
        "loss/drop/gain" => "Сварной стык",
        "end of fiber" => "Конец волокна",
        "saturated reflection" => "Отражение (насыщение)",
        _ => "Неизвестно",
    }
}

pub fn attenuation_db_per_km(trace: &SorTrace, from_km: f64, to_km: f64) -> Option<f64> {
    let span = to_km - from_km;
    if span <= 0.0 {
        return None;
    }
    let from = trace.level_at(from_km)?;
    let to = trace.level_at(to_km)?;
    Some((to - from) / span)
}

pub fn event_rows(events: &[SorEvent], trace: &SorTrace) -> Vec<EventRow> {
    let mut sorted: Vec<&SorEvent> = events.iter().collect();
    sorted.sort_by(|a, b| a.distance_km.total_cmp(&b.distance_km));

    let mut previous_km = None;
    sorted
        .into_iter()
        .enumerate()
        .map(|(index, event)| {
            let attenuation = previous_km
                .and_then(|from| attenuation_db_per_km(trace, from, event.distance_km))
                .map_or_else(|| NO_VALUE.to_string(), |a| format!("{a:.3}"));
            previous_km = Some(event.distance_km);
            EventRow {
                number: event.number,
                kind: event_kind_label(&event.kind, index == 0),
                distance: format!("{:.3}", event.distance_km),
                loss: format!("{:.2}", event.loss_db),
                reflection: reflection_label(event.refl_db),
                attenuation,
            }
        })
        .collect()
}

fn reflection_label(refl_db: f64) -> String {
    if refl_db.abs() > f64::EPSILON {
        format!("{refl_db:.1}")
    } else {
        NO_VALUE.to_string()
    }
}

#[cfg(test)]
mod tests {
    use model::types::{SorEvent, SorTrace};
    use rstest::rstest;

    use crate::events::{attenuation_db_per_km, event_kind_label, event_rows};

    fn trace() -> SorTrace {
        SorTrace {
            distances_km: vec![0.0, 1.0, 2.0, 3.0, 4.0],
            levels_db: vec![0.0, 0.3, 0.6, 0.9, 1.2],
        }
    }

    fn event(number: u16, distance_km: f64, kind: &str, refl_db: f64) -> SorEvent {
        SorEvent {
            number,
            distance_km,
            loss_db: 0.12,
            refl_db,
            kind: kind.to_string(),
            comments: String::new(),
        }
    }

    #[rstest]
    #[case::start_connector("reflection", true, "Разъём (старт)")]
    #[case::connector("reflection", false, "Разъём")]
    #[case::splice("loss/drop/gain", false, "Сварной стык")]
    #[case::end("end of fiber", false, "Конец волокна")]
    #[case::saturated("saturated reflection", false, "Отражение (насыщение)")]
    #[case::unknown("something else", false, "Неизвестно")]
    fn kind_labels(#[case] kind: &str, #[case] is_first: bool, #[case] expected: &str) {
        assert_eq!(event_kind_label(kind, is_first), expected);
    }

    #[test]
    fn attenuation_is_positive_along_the_fiber() {
        let value = attenuation_db_per_km(&trace(), 1.0, 3.0).unwrap();
        assert!((value - 0.3).abs() < 1e-9, "{value}");
    }

    #[rstest]
    #[case::zero_span(2.0, 2.0)]
    #[case::backwards(3.0, 1.0)]
    fn attenuation_needs_forward_span(#[case] from: f64, #[case] to: f64) {
        assert_eq!(attenuation_db_per_km(&trace(), from, to), None);
    }

    #[test]
    fn rows_are_sorted_by_distance_and_first_has_no_attenuation() {
        let events = [
            event(2, 3.0, "end of fiber", 0.0),
            event(1, 1.0, "reflection", -45.0),
        ];
        let rows = event_rows(&events, &trace());

        assert_eq!(rows.iter().map(|r| r.number).collect::<Vec<_>>(), [1, 2]);
        assert_eq!(rows[0].kind, "Разъём (старт)");
        assert_eq!(rows[0].attenuation, "—");
        assert_eq!(rows[1].attenuation, "0.300");
    }

    #[test]
    fn missing_reflection_is_a_dash() {
        let rows = event_rows(&[event(1, 1.0, "loss/drop/gain", 0.0)], &trace());
        assert_eq!(rows[0].reflection, "—");
    }
}
