use view_model::chart_view::DistanceRange;

const MAX_TICKS: f64 = 7.0;
const MANTISSAS: [f64; 4] = [1.0, 2.0, 2.5, 5.0];
const TICK_EPSILON: f64 = 1e-9;

pub fn nice_step(range: f64) -> f64 {
    if !(range.is_finite() && range > 0.0) {
        return 1.0;
    }
    let raw = range / MAX_TICKS;
    let magnitude = 10f64.powf(raw.log10().floor());
    MANTISSAS
        .into_iter()
        .map(|m| m * magnitude)
        .find(|&s| s >= raw)
        .unwrap_or(10.0 * magnitude)
}

pub fn ticks(start: f64, end: f64, step: f64) -> Vec<f64> {
    if !(step > 0.0 && start <= end) {
        return Vec::new();
    }
    let first = (start / step - TICK_EPSILON).ceil() as i64;
    let last = (end / step + TICK_EPSILON).floor() as i64;
    (first..=last).map(|i| i as f64 * step).collect()
}

pub fn tick_decimals(step: f64) -> usize {
    (0..=6)
        .find(|&places| {
            let scaled = step * 10f64.powi(places as i32);
            (scaled - scaled.round()).abs() < TICK_EPSILON * scaled.abs().max(1.0)
        })
        .unwrap_or(6)
}

pub fn visible_range(distances: &[f64], range: DistanceRange) -> Option<(usize, usize)> {
    if distances.is_empty() {
        return None;
    }
    let first = distances
        .partition_point(|&d| d < range.start())
        .saturating_sub(1);
    let last = (distances.partition_point(|&d| d <= range.end()) + 1).min(distances.len());
    (first < last).then_some((first, last))
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use view_model::chart_view::DistanceRange;

    use crate::chart::scale::{nice_step, tick_decimals, ticks, visible_range};

    const DISTANCES: [f64; 6] = [0.0, 1.0, 2.0, 3.0, 4.0, 5.0];

    #[rstest]
    #[case::adds_neighbours(1.5, 3.5, Some((1, 5)))]
    #[case::wider_than_data(-10.0, 10.0, Some((0, 6)))]
    fn visible(#[case] start: f64, #[case] end: f64, #[case] expected: Option<(usize, usize)>) {
        let range = DistanceRange::new(start, end);
        assert_eq!(visible_range(&DISTANCES, range), expected);
    }

    #[test]
    fn visible_range_empty_is_none() {
        assert_eq!(visible_range(&[], DistanceRange::new(0.0, 1.0)), None);
    }

    #[rstest]
    #[case::exact_boundary(7.0, 1.0)]
    #[case::picks_two(12.4, 2.0)]
    #[case::below_one(3.0, 0.5)]
    #[case::deep_zoom(0.05, 0.01)]
    #[case::long_trace(600.0, 100.0)]
    #[case::beyond_old_max(2000.0, 500.0)]
    #[case::zero_range(0.0, 1.0)]
    fn step(#[case] range: f64, #[case] expected: f64) {
        let actual = nice_step(range);
        assert!(
            (actual - expected).abs() < 1e-12,
            "range {range}: expected {expected}, got {actual}"
        );
    }

    fn assert_ticks(actual: Vec<f64>, expected: &[f64]) {
        assert_eq!(actual.len(), expected.len(), "{actual:?} vs {expected:?}");
        for (a, e) in actual.iter().zip(expected) {
            assert!((a - e).abs() < 1e-9, "{actual:?} vs {expected:?}");
        }
    }

    #[test]
    fn ticks_skip_bounds_between_multiples() {
        assert_ticks(ticks(0.05, 0.31, 0.1), &[0.1, 0.2, 0.3]);
    }

    #[test]
    fn ticks_keep_bound_lost_to_float_error() {
        assert_ticks(ticks(0.0, 0.3, 0.1), &[0.0, 0.1, 0.2, 0.3]);
    }

    #[test]
    fn ticks_on_negative_levels() {
        assert_ticks(ticks(-23.0, -3.0, 5.0), &[-20.0, -15.0, -10.0, -5.0]);
    }

    #[rstest]
    #[case::integer(5.0, 0)]
    #[case::one_place(2.5, 1)]
    #[case::two_places(0.25, 2)]
    #[case::small(0.01, 2)]
    fn decimals(#[case] step: f64, #[case] expected: usize) {
        assert_eq!(tick_decimals(step), expected);
    }
}
