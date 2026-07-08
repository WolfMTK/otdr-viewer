use leptos::prelude::*;
use shared_types::SorData;

#[derive(Clone, Copy)]
pub struct RecentFilesVersion(pub RwSignal<u32>);

#[derive(Clone, Copy)]
pub struct OpenedSor(pub RwSignal<Option<SorData>>);

pub fn provide_app_context() {
    provide_context(RecentFilesVersion(RwSignal::new(0)));
    provide_context(OpenedSor(RwSignal::new(None)));
}
