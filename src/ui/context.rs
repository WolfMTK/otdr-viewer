use leptos::prelude::*;
use shared_types::SorData;

use crate::ui::trace_math::{pan_window, zoom_window};

#[derive(Clone, Copy)]
pub struct RecentFilesVersion(pub RwSignal<u32>);

#[derive(Clone, Copy)]
pub struct OpenedSor(pub RwSignal<Option<SorData>>);

#[derive(Clone, Copy)]
pub struct CompareMode(pub RwSignal<bool>);

#[derive(Clone, Copy)]
pub struct ChartView {
    pub full: RwSignal<(f64, f64)>,
    pub view: RwSignal<Option<(f64, f64)>>,
}

impl ChartView {
    const STEP: f64 = 1.4;

    fn from_context() -> Self {
        use_context::<ChartView>().expect("ChartView is provided at app root")
    }

    pub fn window(&self) -> (f64, f64) {
        self.view.get().unwrap_or_else(|| self.full.get())
    }

    pub fn fit(&self) {
        self.view.set(None);
    }

    pub fn is_fitted(&self) -> bool {
        self.view.get().is_none()
    }

    pub fn zoom_at(&self, factor: f64, center: f64) {
        let full = self.full.get_untracked();
        let next = zoom_window(self.window_untracked(), full, factor, center);
        self.apply(next, full);
    }

    pub fn zoom_center(&self, factor: f64) {
        let (lo, hi) = self.window_untracked();
        self.zoom_at(factor, (lo + hi) / 2.0);
    }

    pub fn zoom_in(&self) {
        self.zoom_center(Self::STEP);
    }

    pub fn zoom_out(&self) {
        self.zoom_center(1.0 / Self::STEP);
    }

    pub fn pan(&self, delta: f64) {
        let full = self.full.get_untracked();
        let next = pan_window(self.window_untracked(), full, delta);
        self.apply(next, full);
    }

    fn apply(&self, next: (f64, f64), full: (f64, f64)) {
        if next.0 <= full.0 + f64::EPSILON && next.1 >= full.1 - f64::EPSILON {
            self.view.set(None);
        } else {
            self.view.set(Some(next));
        }
    }

    fn window_untracked(&self) -> (f64, f64) {
        self.view.get_untracked().unwrap_or_else(|| self.full.get_untracked())
    }
}

pub fn provide_app_context() {
    provide_context(RecentFilesVersion(RwSignal::new(0)));
    provide_context(OpenedSor(RwSignal::new(None)));
    provide_context(CompareMode(RwSignal::new(false)));
    provide_context(ChartView {
        full: RwSignal::new((0.0, 1.0)),
        view: RwSignal::new(None),
    });
}

impl ChartView {
    pub fn use_context() -> Self {
        Self::from_context()
    }
}
