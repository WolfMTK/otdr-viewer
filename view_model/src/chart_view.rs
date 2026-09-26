const MIN_SPAN_FRACTION: f64 = 1.0 / 10_000.0;
const PAN_OVERSHOOT: f64 = 0.5;
const BUTTON_ZOOM_STEP: f64 = 1.4;
const FIT_TOLERANCE: f64 = 1e-9;
const LEVEL_PADDING: f64 = 0.06;
const MIN_LEVEL_PADDING_DB: f64 = 0.5;

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

    pub fn middle(self) -> f64 {
        (self.start + self.end) / 2.0
    }

    pub fn clamp_to(self, full: Self) -> Self {
        let full_span = full.span().max(f64::EPSILON);
        let span = self.span().clamp(full_span * MIN_SPAN_FRACTION, full_span);
        let middle = self.middle();
        let (start, end) = (middle - span / 2.0, middle + span / 2.0);

        let overshoot = span * PAN_OVERSHOOT;
        let lowest_start = full.start - overshoot;
        let highest_end = full.end + overshoot;
        let shift = (lowest_start - start).max(0.0) - (end - highest_end).max(0.0);
        Self {
            start: start + shift,
            end: end + shift,
        }
    }

    pub fn zoom(self, full: Self, factor: f64, anchor: f64) -> Self {
        let factor = if factor > 0.0 { factor } else { 1.0 };
        let anchor = anchor.clamp(self.start, self.end);
        let zoomed = Self::new(
            anchor - (anchor - self.start) / factor,
            anchor + (self.end - anchor) / factor,
        );
        if zoomed.span() >= full.span() {
            return full;
        }
        zoomed.clamp_to(full)
    }

    pub fn pan(self, full: Self, delta_km: f64) -> Self {
        Self {
            start: self.start + delta_km,
            end: self.end + delta_km,
        }
        .clamp_to(full)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LevelRange {
    min: f64,
    max: f64,
}

impl LevelRange {
    pub fn around(levels: &[f64]) -> Self {
        let (min, max) = levels
            .iter()
            .fold((f64::INFINITY, f64::NEG_INFINITY), |(min, max), &l| (min.min(l), max.max(l)));
        if !(min.is_finite() && max.is_finite()) {
            return Self { min: 0.0, max: 1.0 };
        }
        let padding = ((max - min) * LEVEL_PADDING).max(MIN_LEVEL_PADDING_DB);
        Self {
            min: min - padding,
            max: max + padding,
        }
    }

    pub fn min(self) -> f64 {
        self.min
    }

    pub fn max(self) -> f64 {
        self.max
    }

    pub fn span(self) -> f64 {
        self.max - self.min
    }

    fn shifted(self, delta_db: f64) -> Self {
        Self {
            min: self.min + delta_db,
            max: self.max + delta_db,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ChartView {
    full: DistanceRange,
    visible: Option<DistanceRange>,
    levels: LevelRange,
    level_shift_db: f64,
}

impl ChartView {
    pub fn new(full: DistanceRange, levels: LevelRange) -> Self {
        Self {
            full,
            visible: None,
            levels,
            level_shift_db: 0.0,
        }
    }

    pub fn full(&self) -> DistanceRange {
        self.full
    }

    pub fn visible(&self) -> DistanceRange {
        self.visible.unwrap_or(self.full)
    }

    pub fn levels(&self) -> LevelRange {
        self.levels.shifted(self.level_shift_db)
    }

    pub fn is_fitted(&self) -> bool {
        self.visible.is_none() && self.level_shift_db == 0.0
    }

    pub fn fit(&mut self) {
        self.visible = None;
        self.level_shift_db = 0.0;
    }

    pub fn zoom_at(&mut self, factor: f64, anchor_km: f64) {
        self.set_visible(self.visible().zoom(self.full, factor, anchor_km));
    }

    pub fn zoom_in(&mut self) {
        self.zoom_at(BUTTON_ZOOM_STEP, self.visible().middle());
    }

    pub fn zoom_out(&mut self) {
        self.zoom_at(1.0 / BUTTON_ZOOM_STEP, self.visible().middle());
    }

    pub fn pan(&mut self, delta_km: f64, delta_db: f64) {
        self.set_visible(self.visible().pan(self.full, delta_km));
        let limit = self.levels.span();
        self.level_shift_db = (self.level_shift_db + delta_db).clamp(-limit, limit);
    }

    fn set_visible(&mut self, range: DistanceRange) {
        let tolerance = self.full.span() * FIT_TOLERANCE;
        let covers_full = range.start() <= self.full.start() + tolerance
            && range.end() >= self.full.end() - tolerance;
        self.visible = (!covers_full).then_some(range);
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ChartSettings {
    pub grid_visible: bool,
}

impl Default for ChartSettings {
    fn default() -> Self {
        Self { grid_visible: true }
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::chart_view::{ChartView, DistanceRange, LevelRange, MIN_SPAN_FRACTION};

    fn range(start: f64, end: f64) -> DistanceRange {
        DistanceRange::new(start, end)
    }

    fn full() -> DistanceRange {
        range(0.0, 100.0)
    }

    fn chart() -> ChartView {
        ChartView::new(full(), LevelRange::around(&[0.0, 30.0]))
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
    #[case::out_near_edge_may_pass_start(range(0.0, 20.0), 0.5, 5.0, range(-5.0, 35.0))]
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
    #[case::past_end_up_to_half_window(range(80.0, 100.0), 50.0, range(90.0, 110.0))]
    #[case::past_start_up_to_half_window(range(0.0, 20.0), -50.0, range(-10.0, 10.0))]
    #[case::whole_trace_can_move(range(0.0, 100.0), 30.0, range(30.0, 130.0))]
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

    #[test]
    fn new_chart_shows_whole_trace() {
        let chart = chart();
        assert!(chart.is_fitted());
        assert_eq!(chart.visible(), full());
    }

    #[test]
    fn fit_after_zoom_shows_whole_trace() {
        let mut chart = chart();
        chart.zoom_in();
        assert!(!chart.is_fitted());
        chart.fit();
        assert!(chart.is_fitted());
    }

    #[test]
    fn zooming_back_out_counts_as_fitted() {
        let mut chart = chart();
        chart.zoom_in();
        chart.zoom_out();
        assert!(chart.is_fitted());
    }

    #[test]
    fn zooming_out_of_shifted_view_returns_whole_trace() {
        let mut chart = chart();
        chart.zoom_in();
        chart.pan(30.0, 0.0);
        chart.zoom_at(0.01, 50.0);
        assert!(chart.is_fitted());
    }

    #[test]
    fn button_zoom_keeps_middle() {
        let mut chart = chart();
        chart.zoom_in();
        assert!((chart.visible().middle() - full().middle()).abs() < 1e-9);
    }

    #[test]
    fn horizontal_pan_leaves_fitted_view_and_fit_restores_it() {
        let mut chart = chart();
        chart.pan(10.0, 0.0);
        assert!(!chart.is_fitted());
        chart.fit();
        assert_eq!(chart.visible(), full());
    }

    #[test]
    fn vertical_pan_shifts_levels_and_fit_resets_it() {
        let mut chart = chart();
        let before = chart.levels();
        chart.pan(0.0, 5.0);
        assert!((chart.levels().min() - before.min() - 5.0).abs() < 1e-9);
        assert!(!chart.is_fitted());
        chart.fit();
        assert_eq!(chart.levels(), before);
    }

    #[test]
    fn vertical_pan_keeps_trace_reachable() {
        let mut chart = chart();
        let before = chart.levels();
        chart.pan(0.0, 1e6);
        assert!(chart.levels().min() <= before.max());
    }

    #[test]
    fn horizontal_zoom_keeps_level_scale() {
        let mut chart = chart();
        let before = chart.levels();
        chart.zoom_in();
        assert_eq!(chart.levels(), before);
    }

    #[test]
    fn level_range_pads_both_sides() {
        let levels = LevelRange::around(&[10.0, 30.0]);
        assert!(levels.min() < 10.0 && levels.max() > 30.0);
    }

    #[test]
    fn flat_trace_still_has_height() {
        assert!(LevelRange::around(&[-7.0, -7.0]).span() > 0.0);
    }
}
