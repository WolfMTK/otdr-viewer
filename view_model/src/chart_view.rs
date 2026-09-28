const MIN_SPAN_FRACTION: f64 = 1.0 / 10_000.0;
const MAX_SPAN_FACTOR: f64 = 20.0;
const MIN_OVERLAP: f64 = 0.5;
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
        let span = self
            .span()
            .clamp(full_span * MIN_SPAN_FRACTION, full_span * MAX_SPAN_FACTOR);
        let overlap = MIN_OVERLAP * span.min(full_span);
        let lowest_start = full.start + overlap - span;
        let highest_start = full.end - overlap;
        let start = (self.middle() - span / 2.0).clamp(lowest_start, highest_start);
        Self {
            start,
            end: start + span,
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

    fn middle(self) -> f64 {
        (self.min + self.max) / 2.0
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
    visible_levels: Option<LevelRange>,
}

impl ChartView {
    pub fn new(full: DistanceRange, levels: LevelRange) -> Self {
        Self {
            full,
            visible: None,
            levels,
            visible_levels: None,
        }
    }

    pub fn full(&self) -> DistanceRange {
        self.full
    }

    pub fn visible(&self) -> DistanceRange {
        self.visible.unwrap_or(self.full)
    }

    pub fn levels(&self) -> LevelRange {
        self.visible_levels.unwrap_or(self.levels)
    }

    pub fn is_fitted(&self) -> bool {
        self.visible.is_none() && self.visible_levels.is_none()
    }

    pub fn fit(&mut self) {
        self.visible = None;
        self.visible_levels = None;
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

    pub fn zoom_levels_at(&mut self, factor: f64, anchor_db: f64) {
        let current = self.levels();
        let factor = if factor > 0.0 { factor } else { 1.0 };
        let anchor = anchor_db.clamp(current.min, current.max);
        let min_span = self.levels.span() * MIN_SPAN_FRACTION;
        let span = (current.span() / factor).max(min_span);
        let share = (anchor - current.min) / current.span();
        let min = anchor - share * span;
        self.set_levels(LevelRange {
            min,
            max: min + span,
        });
    }

    pub fn pan(&mut self, delta_km: f64, delta_db: f64) {
        self.set_visible(self.visible().pan(self.full, delta_km));
        if delta_db != 0.0 {
            self.set_levels(self.levels().shifted(delta_db));
        }
    }

    fn set_visible(&mut self, range: DistanceRange) {
        let tolerance = self.full.span() * FIT_TOLERANCE;
        let is_full = (range.start() - self.full.start()).abs() <= tolerance
            && (range.end() - self.full.end()).abs() <= tolerance;
        self.visible = (!is_full).then_some(range);
    }

    fn set_levels(&mut self, range: LevelRange) {
        let full = self.levels;
        let full_span = full.span();
        let span = range
            .span()
            .clamp(full_span * MIN_SPAN_FRACTION, full_span * MAX_SPAN_FACTOR);
        let overlap = MIN_OVERLAP * span.min(full_span);
        let min =
            (range.middle() - span / 2.0).clamp(full.min + overlap - span, full.max - overlap);
        let tolerance = full_span * FIT_TOLERANCE;
        let is_full =
            (min - full.min).abs() <= tolerance && (min + span - full.max).abs() <= tolerance;
        self.visible_levels = (!is_full).then_some(LevelRange {
            min,
            max: min + span,
        });
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ChartSettings {
    pub grid_visible: bool,
    pub markers_visible: bool,
    pub events_visible: bool,
    pub ideal_visible: bool,
}

impl Default for ChartSettings {
    fn default() -> Self {
        Self {
            grid_visible: true,
            markers_visible: true,
            events_visible: true,
            ideal_visible: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::chart_view::{
        ChartView, DistanceRange, LevelRange, MAX_SPAN_FACTOR, MIN_SPAN_FRACTION,
    };

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
    #[case::out_to_full(range(25.0, 75.0), 0.5, 50.0, full())]
    #[case::out_near_edge_may_pass_start(range(0.0, 20.0), 0.5, 5.0, range(-5.0, 35.0))]
    #[case::out_beyond_full_keeps_anchor(range(0.0, 100.0), 0.5, 25.0, range(-25.0, 175.0))]
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
    fn zooming_far_out_is_limited_and_keeps_trace_on_screen() {
        let mut chart = chart();
        chart.zoom_at(1e-6, 0.0);
        let visible = chart.visible();
        assert!((visible.span() - full().span() * MAX_SPAN_FACTOR).abs() < 1e-6);
        assert!(visible.start() <= full().end() && visible.end() >= full().start());
        assert!(!chart.is_fitted());
        chart.fit();
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

    #[test]
    fn level_zoom_keeps_anchor_in_place() {
        let mut chart = chart();
        let before = chart.levels();
        let anchor = before.min() + before.span() * 0.25;
        chart.zoom_levels_at(2.0, anchor);
        let after = chart.levels();
        assert!((after.span() - before.span() / 2.0).abs() < 1e-9);
        let share_before = (anchor - before.min()) / before.span();
        let share_after = (anchor - after.min()) / after.span();
        assert!((share_before - share_after).abs() < 1e-9);
        assert!(!chart.is_fitted());
    }

    #[test]
    fn level_zoom_back_out_returns_full_scale() {
        let mut chart = chart();
        let before = chart.levels();
        chart.zoom_levels_at(2.0, before.middle());
        chart.zoom_levels_at(0.5, before.middle());
        assert!(chart.is_fitted());
    }

    #[test]
    fn deep_level_zoom_respects_min_span() {
        let mut chart = chart();
        let full = chart.levels();
        chart.zoom_levels_at(1e9, full.middle());
        assert!(chart.levels().span() >= full.span() * MIN_SPAN_FRACTION - 1e-12);
    }

    #[test]
    fn fit_resets_level_zoom() {
        let mut chart = chart();
        let before = chart.levels();
        chart.zoom_levels_at(3.0, before.middle());
        chart.fit();
        assert_eq!(chart.levels(), before);
    }

    #[test]
    fn level_zoom_can_go_beyond_full_scale() {
        let mut chart = chart();
        let before = chart.levels();
        chart.zoom_levels_at(0.5, before.middle());
        assert!((chart.levels().span() - before.span() * 2.0).abs() < 1e-9);
    }
}
