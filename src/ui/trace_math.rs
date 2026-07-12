pub fn linspace(min: f64, max: f64, count: usize) -> Vec<f64> {
    if count < 2 || (max - min).abs() < f64::EPSILON {
        return vec![min];
    }
    let step = (max - min) / (count as f64 - 1.0);
    (0..count).map(|i| min + step * i as f64).collect()
}

pub fn nice_step(range: f64) -> f64 {
    const CANDIDATES: [f64; 8] = [1.0, 2.0, 5.0, 10.0, 20.0, 25.0, 50.0, 100.0];
    CANDIDATES
        .iter()
        .cloned()
        .find(|&s| range / s <= 7.0)
        .unwrap_or(*CANDIDATES.last().unwrap())
}

pub fn svg_num(v: f64) -> String {
    format!("{v:.2}")
}

pub fn level_at(distances: &[f64], levels: &[f64], target: f64) -> Option<f64> {
    if distances.is_empty() {
        return None;
    }

    let i = distances.partition_point(|&d| d < target);

    let best_idx = match i {
        0 => 0,
        n if n >= distances.len() => distances.len() - 1,
        n => {
            let (left, right) = (distances[n - 1], distances[n]);
            if (target - left).abs() <= (right - target).abs() {
                n - 1
            } else {
                n
            }
        }
    };

    levels.get(best_idx).copied()
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::ui::trace_math::{level_at, linspace, nice_step};

    #[rstest]
    fn linspace_basic() {
        assert_eq!(linspace(0.0, 10.0, 3), vec![0.0, 5.0, 10.0]);
    }

    #[rstest]
    fn linspace_degenerate_range_returns_single_value() {
        assert_eq!(linspace(5.0, 5.0, 6), vec![5.0]);
    }

    #[rstest]
    fn nice_step_picks_smallest_that_fits() {
        assert_eq!(nice_step(12.4), 2.0);
        assert_eq!(nice_step(3.0), 1.0);
        assert_eq!(nice_step(600.0), 100.0);
    }

    #[rstest]
    fn level_at_finds_nearest() {
        let d = vec![0.0, 1.0, 2.0, 3.0];
        let l = vec![10.0, 20.0, 30.0, 40.0];
        assert_eq!(level_at(&d, &l, 1.4), Some(20.0));
        assert_eq!(level_at(&d, &l, 1.6), Some(30.0));
    }

    #[rstest]
    fn level_at_empty_is_none() {
        assert_eq!(level_at(&[], &[], 1.0), None);
    }
}
