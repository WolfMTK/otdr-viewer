use model::types::{SorEvent, SorTrace};

use crate::events::attenuation_db_per_km;

const MIN_EVENT_DISTANCE_KM: f64 = 0.05;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Marker {
    A,
    B,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Markers {
    pub a_km: f64,
    pub b_km: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MarkerMeasurement {
    pub span_km: f64,
    pub loss_db: f64,
    pub attenuation_db_per_km: Option<f64>,
}

impl MarkerMeasurement {
    pub fn labels(&self) -> [(&'static str, String); 3] {
        [
            ("A → B", format!("{:.3} км", self.span_km)),
            ("Потери", format!("{:.2} дБ", self.loss_db)),
            (
                "Затухание",
                self.attenuation_db_per_km
                    .map_or_else(|| "—".to_string(), |a| format!("{a:.3} дБ/км")),
            ),
        ]
    }
}

impl Markers {
    pub fn get(&self, marker: Marker) -> f64 {
        match marker {
            Marker::A => self.a_km,
            Marker::B => self.b_km,
        }
    }

    pub fn set(&mut self, marker: Marker, km: f64) {
        match marker {
            Marker::A => self.a_km = km,
            Marker::B => self.b_km = km,
        }
    }

    pub fn measure(&self, trace: &SorTrace) -> Option<MarkerMeasurement> {
        let (from, to) = (self.a_km.min(self.b_km), self.a_km.max(self.b_km));
        let loss_db = trace.level_at(to)? - trace.level_at(from)?;
        Some(MarkerMeasurement {
            span_km: to - from,
            loss_db,
            attenuation_db_per_km: attenuation_db_per_km(trace, from, to),
        })
    }
}

pub fn ab_endpoints(events: &[SorEvent], trace: &SorTrace) -> Markers {
    let mut distances: Vec<f64> = events
        .iter()
        .map(|e| e.distance_km)
        .filter(|&d| d > MIN_EVENT_DISTANCE_KM)
        .collect();
    distances.sort_by(f64::total_cmp);
    match (distances.first(), distances.last()) {
        (Some(&first), Some(&last)) if last > first => Markers {
            a_km: first,
            b_km: last,
        },
        _ => trace_quarters(trace),
    }
}

fn trace_quarters(trace: &SorTrace) -> Markers {
    let start = trace.distances_km.first().copied().unwrap_or(0.0);
    let end = trace.distances_km.last().copied().unwrap_or(1.0);
    let span = end - start;
    Markers {
        a_km: start + span * 0.25,
        b_km: start + span * 0.75,
    }
}

#[cfg(test)]
mod tests {
    use model::types::{SorEvent, SorTrace};

    use crate::markers::{Marker, Markers, ab_endpoints};

    fn trace() -> SorTrace {
        SorTrace {
            distances_km: vec![0.0, 2.0, 4.0, 6.0, 8.0],
            levels_db: vec![0.0, 1.0, 2.0, 3.0, 4.0],
        }
    }

    fn event(distance_km: f64) -> SorEvent {
        SorEvent {
            number: 1,
            distance_km,
            loss_db: 0.0,
            refl_db: 0.0,
            kind: String::new(),
            comments: String::new(),
        }
    }

    #[test]
    fn endpoints_span_first_and_last_event_skipping_launch() {
        let events = [event(6.0), event(0.01), event(2.0), event(4.0)];
        assert_eq!(
            ab_endpoints(&events, &trace()),
            Markers {
                a_km: 2.0,
                b_km: 6.0
            }
        );
    }

    #[test]
    fn too_few_events_fall_back_to_trace_quarters() {
        assert_eq!(
            ab_endpoints(&[event(4.0)], &trace()),
            Markers {
                a_km: 2.0,
                b_km: 6.0
            }
        );
    }

    #[test]
    fn measurement_is_positive_and_ignores_marker_order() {
        let markers = Markers {
            a_km: 6.0,
            b_km: 2.0,
        };
        let measurement = markers.measure(&trace()).unwrap();
        assert!((measurement.span_km - 4.0).abs() < 1e-9);
        assert!((measurement.loss_db - 2.0).abs() < 1e-9);
        assert!((measurement.attenuation_db_per_km.unwrap() - 0.5).abs() < 1e-9);
    }

    #[test]
    fn markers_on_same_point_have_no_attenuation() {
        let markers = Markers {
            a_km: 4.0,
            b_km: 4.0,
        };
        assert_eq!(markers.measure(&trace()).unwrap().attenuation_db_per_km, None);
    }

    #[test]
    fn set_moves_only_the_chosen_marker() {
        let mut markers = Markers {
            a_km: 1.0,
            b_km: 2.0,
        };
        markers.set(Marker::B, 5.0);
        assert_eq!(markers.get(Marker::A), 1.0);
        assert_eq!(markers.get(Marker::B), 5.0);
    }
}
