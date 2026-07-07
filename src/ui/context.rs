use leptos::prelude::*;
use shared_types::SorData;

#[derive(Clone, Copy)]
pub struct RecentFilesVersion(pub RwSignal<u32>);

#[derive(Clone, Copy)]
pub struct OpenedSor(pub RwSignal<Option<SorData>>);

pub fn provide_recent_files_version() {
    provide_context(RecentFilesVersion(RwSignal::new(0)));
}
