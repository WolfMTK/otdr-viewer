use model::types::{SorEvent, SorTrace};

const SPEED_OF_LIGHT_KM_S: f64 = 299_792.458;
const TYPICAL_GROUP_INDEX: f64 = 1.468;
const MIN_POINTS: usize = 5;
const MARGIN_BEFORE_EVENT: f64 = 0.25;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Segment {
    pub from_km: f64,
    pub to_km: f64,
    pub from_db: f64,
    pub to_db: f64,
}

pub fn pulse_length_km(pulse_ns: u16) -> f64 {
    f64::from(pulse_ns) * 1e-9 * SPEED_OF_LIGHT_KM_S / TYPICAL_GROUP_INDEX
}

pub fn ideal_trace(trace: &SorTrace, events: &[SorEvent], pulse_km: f64) -> Vec<Segment> {
    let mut boundaries: Vec<f64> = events.iter().map(|e| e.distance_km).collect();
    boundaries.sort_by(f64::total_cmp);
    boundaries.dedup();

    boundaries
        .windows(2)
        .filter_map(|pair| {
            let (from_km, to_km) = (pair[0], pair[1]);
            let fit_from = from_km + pulse_km;
            let fit_to = to_km - pulse_km * MARGIN_BEFORE_EVENT;
            let (slope, intercept) = fit_line(trace, fit_from, fit_to)?;
            Some(Segment {
                from_km,
                to_km,
                from_db: intercept + slope * from_km,
                to_db: intercept + slope * to_km,
            })
        })
        .collect()
}

fn fit_line(trace: &SorTrace, from_km: f64, to_km: f64) -> Option<(f64, f64)> {
    if to_km <= from_km {
        return None;
    }
    let d = &trace.distances_km;
    let first = d.partition_point(|&x| x < from_km);
    let last = d.partition_point(|&x| x <= to_km);
    let (xs, ys) = (&d[first..last], &trace.levels_db[first..last]);
    if xs.len() < MIN_POINTS {
        return None;
    }

    let n = xs.len() as f64;
    let mean_x = xs.iter().sum::<f64>() / n;
    let mean_y = ys.iter().sum::<f64>() / n;
    let (sxy, sxx) = xs.iter().zip(ys).fold((0.0, 0.0), |(sxy, sxx), (&x, &y)| {
        let dx = x - mean_x;
        (sxy + dx * (y - mean_y), sxx + dx * dx)
    });
    if sxx <= f64::EPSILON {
        return None;
    }
    let slope = sxy / sxx;
    Some((slope, mean_y - slope * mean_x))
}

#[cfg(test)]
mod tests {
    use model::types::{SorEvent, SorTrace};

    use crate::ideal::{Segment, ideal_trace, pulse_length_km};

    fn event(distance_km: f64) -> SorEvent {
        SorEvent {
            number: 0,
            distance_km,
            loss_db: 0.0,
            refl_db: 0.0,
            kind: String::new(),
            comments: String::new(),
        }
    }

    fn stepped_trace() -> SorTrace {
        let distances_km: Vec<f64> = (0..=1000).map(|i| i as f64 * 0.01).collect();
        let levels_db = distances_km
            .iter()
            .map(|&d| 0.2 * d + if d >= 5.0 { 0.5 } else { 0.0 })
            .collect();
        SorTrace {
            distances_km,
            levels_db,
        }
    }

    fn assert_close(a: f64, b: f64) {
        assert!((a - b).abs() < 1e-6, "{a} vs {b}");
    }

    #[test]
    fn step_between_segments_equals_splice_loss() {
        let events = [event(0.0), event(5.0), event(10.0)];
        let segments = ideal_trace(&stepped_trace(), &events, 0.1);

        assert_eq!(segments.len(), 2);
        let (before, after) = (segments[0], segments[1]);
        assert_close(before.to_km, 5.0);
        assert_close(after.from_km, 5.0);
        assert_close(after.from_db - before.to_db, 0.5);
    }

    #[test]
    fn segments_follow_the_fiber_slope() {
        let segments = ideal_trace(&stepped_trace(), &[event(0.0), event(5.0)], 0.1);
        let Segment {
            from_km,
            to_km,
            from_db,
            to_db,
        } = segments[0];
        assert_close((to_db - from_db) / (to_km - from_km), 0.2);
    }

    #[test]
    fn segment_shorter_than_the_pulse_is_skipped() {
        let events = [event(5.0), event(5.05), event(10.0)];
        let segments = ideal_trace(&stepped_trace(), &events, 0.1);
        assert_eq!(segments.len(), 1);
        assert_close(segments[0].from_km, 5.05);
    }

    #[test]
    fn events_are_used_in_distance_order() {
        let events = [event(10.0), event(0.0), event(5.0)];
        assert_eq!(ideal_trace(&stepped_trace(), &events, 0.1).len(), 2);
    }

    #[test]
    fn one_microsecond_pulse_is_about_two_hundred_meters() {
        assert!((pulse_length_km(1000) - 0.204).abs() < 0.001);
    }
}
