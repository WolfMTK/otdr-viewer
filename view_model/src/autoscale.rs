use model::types::SorTrace;

use crate::chart_view::{DistanceRange, LevelRange};

const PADDING: f64 = 0.06;
const MIN_PADDING_DB: f64 = 0.01;

pub fn levels_for_window(trace: &SorTrace, window: DistanceRange) -> Option<LevelRange> {
    let distances = &trace.distances_km;
    let first = distances
        .partition_point(|&d| d < window.start())
        .saturating_sub(1);
    let last = (distances.partition_point(|&d| d <= window.end()) + 1).min(distances.len());
    if last.saturating_sub(first) < 2 {
        return None;
    }

    let (min, max) = trace.levels_db[first..last]
        .iter()
        .fold((f64::INFINITY, f64::NEG_INFINITY), |(min, max), &l| (min.min(l), max.max(l)));
    Some(LevelRange::spanning(min, max, PADDING, MIN_PADDING_DB))
}

#[cfg(test)]
mod tests {
    use model::types::SorTrace;

    use crate::autoscale::levels_for_window;
    use crate::chart_view::DistanceRange;

    fn trace_with(level: impl Fn(usize, f64) -> f64) -> SorTrace {
        let distances_km: Vec<f64> = (0..=1000).map(|i| f64::from(i) * 0.01).collect();
        let levels_db = distances_km
            .iter()
            .enumerate()
            .map(|(i, &d)| level(i, d))
            .collect();
        SorTrace {
            distances_km,
            levels_db,
        }
    }

    fn window(start: f64, end: f64) -> DistanceRange {
        DistanceRange::new(start, end)
    }

    #[test]
    fn scale_hugs_a_small_splice_step() {
        let trace = trace_with(|_, d| 0.2 * d + if d >= 5.0 { 0.3 } else { 0.0 });
        let levels = levels_for_window(&trace, window(4.0, 6.0)).unwrap();
        assert!(levels.span() > 0.7 && levels.span() < 0.9, "span {}", levels.span());
    }

    #[test]
    fn reflection_peak_stays_on_the_scale() {
        let trace = trace_with(|i, d| if i == 500 { -30.0 } else { 0.2 * d });
        let levels = levels_for_window(&trace, window(4.0, 6.0)).unwrap();
        assert!(levels.min() < -30.0, "min {}", levels.min());
    }

    #[test]
    fn noise_after_the_end_of_fiber_stays_on_the_scale() {
        let trace = trace_with(|i, d| {
            if d <= 8.0 {
                0.2 * d
            } else {
                60.0 + (i % 2) as f64 * 5.0
            }
        });
        let levels = levels_for_window(&trace, window(6.0, 10.0)).unwrap();
        assert!(levels.max() >= 65.0, "max {}", levels.max());
    }

    #[test]
    fn window_between_two_samples_still_gets_a_scale() {
        let trace = trace_with(|_, d| d);
        let levels = levels_for_window(&trace, window(1.001, 1.004)).unwrap();
        assert!(levels.min() <= 1.0 + 1e-9 && levels.max() >= 1.01 - 1e-9);
    }

    #[test]
    fn window_outside_the_trace_gets_no_scale() {
        let trace = trace_with(|_, d| d);
        assert!(levels_for_window(&trace, window(100.0, 101.0)).is_none());
        assert!(levels_for_window(&trace, window(-5.0, -4.0)).is_none());
    }
}
