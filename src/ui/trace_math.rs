const MIN_SPAN_FRACTION: f64 = 1.0 / 10_000.0;

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

pub fn clamp_window(window: (f64, f64), full: (f64, f64)) -> (f64, f64) {
    let (fl, fh) = (full.0.min(full.1), full.0.max(full.1));
    let full_span = (fh - fl).max(f64::EPSILON);
    let min_span = (full_span * MIN_SPAN_FRACTION).max(f64::EPSILON);

    let (mut lo, mut hi) = (window.0.min(window.1), window.0.max(window.1));

    if hi - lo < min_span {
        let center = (lo + hi) / 2.0;
        lo = center - min_span / 2.0;
        hi = center + min_span / 2.0;
    }

    if hi - lo >= full_span {
        return (fl, fh);
    }

    if lo < fl {
        let d = fl - lo;
        lo += d;
        hi += d;
    }
    if hi > fh {
        let d = hi - fh;
        lo -= d;
        hi -= d;
    }

    (lo.max(fl), hi.min(fh))
}

pub fn zoom_window(window: (f64, f64), full: (f64, f64), factor: f64, center: f64) -> (f64, f64) {
    let (lo, hi) = (window.0.min(window.1), window.0.max(window.1));
    let factor = if factor <= 0.0 { 1.0 } else { factor };
    let c = center.clamp(lo, hi);
    let new_lo = c - (c - lo) / factor;
    let new_hi = c + (hi - c) / factor;
    clamp_window((new_lo, new_hi), full)
}

pub fn pan_window(window: (f64, f64), full: (f64, f64), delta: f64) -> (f64, f64) {
    clamp_window((window.0 + delta, window.1 + delta), full)
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::ui::trace_math::{clamp_window, level_at, linspace, nice_step, pan_window, zoom_window};

    fn approx(a: (f64, f64), b: (f64, f64)) {
        assert!(
            (a.0 - b.0).abs() < 1e-9 && (a.1 - b.1).abs() < 1e-9,
            "expected {b:?}, got {a:?}"
        );
    }

    #[rstest]
    fn zoom_in_around_center_keeps_midpoint() {
        approx(zoom_window((0.0, 100.0), (0.0, 100.0), 2.0, 50.0), (25.0, 75.0));
    }

    #[rstest]
    fn zoom_in_toward_cursor_keeps_cursor_fixed() {
        approx(zoom_window((0.0, 100.0), (0.0, 100.0), 2.0, 40.0), (20.0, 70.0));
    }

    #[rstest]
    fn zoom_in_at_left_edge_clamps() {
        approx(zoom_window((0.0, 100.0), (0.0, 100.0), 2.0, 0.0), (0.0, 50.0));
    }

    #[rstest]
    fn zoom_out_beyond_full_snaps_to_full() {
        approx(zoom_window((25.0, 75.0), (0.0, 100.0), 0.5, 50.0), (0.0, 100.0));
    }

    #[rstest]
    fn zoom_out_partial() {
        approx(zoom_window((40.0, 60.0), (0.0, 100.0), 0.5, 50.0), (30.0, 70.0));
    }

    #[rstest]
    fn pan_shifts_and_clamps_without_shrinking() {
        approx(pan_window((20.0, 40.0), (0.0, 100.0), 10.0), (30.0, 50.0));
        approx(pan_window((80.0, 100.0), (0.0, 100.0), 50.0), (80.0, 100.0));
        approx(pan_window((0.0, 20.0), (0.0, 100.0), -50.0), (0.0, 20.0));
    }

    #[rstest]
    fn deep_zoom_respects_min_span() {
        let w = zoom_window((49.9, 50.1), (0.0, 100.0), 1000.0, 50.0);
        assert!(w.1 - w.0 >= (100.0 / 10_000.0) - 1e-9, "span too small: {w:?}");
    }

    #[rstest]
    fn clamp_handles_inverted_input() {
        let w = clamp_window((10.0, 5.0), (100.0, 0.0));
        assert!(w.0 < w.1 && w.0 >= 0.0 && w.1 <= 100.0, "got {w:?}");
    }

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
