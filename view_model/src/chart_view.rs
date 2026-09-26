const MIN_SPAN_FRACTION: f64 = 1.0 / 10_000.0;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DistanceRange {
    start: f64,
    end: f64,
}

impl DistanceRange {
    pub fn new(a: f64, b: f64) -> Self {
        Self {
            start: a.min(b),
            end: a.max(b),
        }
    }

    pub fn start(self) -> f64 {
        self.start
    }

    pub fn end(self) -> f64 {
        self.end
    }

    pub fn span(self) -> f64 {
        self.end - self.start
    }

    pub fn clamp_to(self, full: Self) -> Self {
        let full_span = full.span().max(f64::EPSILON);
        let min_span = full_span * MIN_SPAN_FRACTION;

        let range = if self.span() < min_span {
            let middle = (self.start + self.end) / 2.0;
            Self::new(middle - min_span / 2.0, middle + min_span / 2.0)
        } else {
            self
        };
        if range.span() >= full_span {
            return full;
        }

        let shift = (full.start - range.start).max(0.0) - (range.end - full.end).max(0.0);
        Self {
            start: range.start + shift,
            end: range.end + shift,
        }
    }

    pub fn zoom(self, full: Self, factor: f64, anchor: f64) -> Self {
        let factor = if factor > 0.0 { factor } else { 1.0 };
        let anchor = anchor.clamp(self.start, self.end);
        Self::new(anchor - (anchor - self.start) / factor, anchor + (self.end - anchor) / factor)
            .clamp_to(full)
    }

    pub fn pan(self, full: Self, delta_km: f64) -> Self {
        Self {
            start: self.start + delta_km,
            end: self.end + delta_km,
        }
        .clamp_to(full)
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::chart_view::{DistanceRange, MIN_SPAN_FRACTION};

    fn range(start: f64, end: f64) -> DistanceRange {
        DistanceRange::new(start, end)
    }

    fn full() -> DistanceRange {
        range(0.0, 100.0)
    }

    fn assert_approx(actual: DistanceRange, expected: DistanceRange) {
        assert!(
            (actual.start() - expected.start()).abs() < 1e-9
                && (actual.end() - expected.end()).abs() < 1e-9,
            "expected {expected:?}, got {actual:?}"
        );
    }

    #[rstest]
    #[case::in_keeps_anchor_fixed(range(0.0, 100.0), 2.0, 40.0, range(20.0, 70.0))]
    #[case::out_partial(range(40.0, 60.0), 0.5, 50.0, range(30.0, 70.0))]
    #[case::out_snaps_to_full(range(25.0, 75.0), 0.5, 50.0, full())]
    #[case::out_near_edge_shifts(range(0.0, 20.0), 0.5, 5.0, range(0.0, 40.0))]
    fn zoom(
        #[case] visible: DistanceRange,
        #[case] factor: f64,
        #[case] anchor: f64,
        #[case] expected: DistanceRange,
    ) {
        assert_approx(visible.zoom(full(), factor, anchor), expected);
    }

    #[rstest]
    #[case::inside(range(20.0, 40.0), 10.0, range(30.0, 50.0))]
    #[case::stops_at_end(range(80.0, 100.0), 50.0, range(80.0, 100.0))]
    #[case::stops_at_start(range(0.0, 20.0), -50.0, range(0.0, 20.0))]
    fn pan(#[case] visible: DistanceRange, #[case] delta_km: f64, #[case] expected: DistanceRange) {
        assert_approx(visible.pan(full(), delta_km), expected);
    }

    #[test]
    fn deep_zoom_respects_min_span() {
        let zoomed = range(49.9, 50.1).zoom(full(), 1000.0, 50.0);
        assert!(
            zoomed.span() >= full().span() * MIN_SPAN_FRACTION - 1e-9,
            "span too small: {zoomed:?}"
        );
    }
}
