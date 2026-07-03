use leptos::prelude::*;

#[derive(Clone, Copy)]
pub struct RecentFilesVersion(pub RwSignal<u32>);

pub fn provide_recent_files_version() {
    provide_context(RecentFilesVersion(RwSignal::new(0)));
}
