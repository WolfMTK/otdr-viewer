use view_model::chart_view::DistanceRange;

const MAX_TICKS: f64 = 7.0;
const MANTISSAS: [f64; 4] = [1.0, 2.0, 2.5, 5.0];

pub fn linspace(min: f64, max: f64, count: usize) -> Vec<f64> {
    if count < 2 || (max - min).abs() < f64::EPSILON {
        return vec![min];
    }
    let step = (max - min) / (count - 1) as f64;
    (0..count).map(|i| min + step * i as f64).collect()
}

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

    use crate::chart::scale::{linspace, nice_step, visible_range};

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

    #[test]
    fn linspace_includes_both_ends() {
        assert_eq!(linspace(0.0, 10.0, 3), [0.0, 5.0, 10.0]);
    }

    #[test]
    fn linspace_degenerate_range_is_single_value() {
        assert_eq!(linspace(5.0, 5.0, 6), [5.0]);
    }
}
