use std::path::Path;

use chrono::{DateTime, Local, Utc};

pub(crate) mod files;
pub(crate) mod recent_files;
pub(crate) mod sor;
pub(crate) mod window;

pub(crate) fn command_error(op: &str, user_msg: &str, e: impl std::fmt::Display) -> String {
    log::error!("{op}: {e}");
    format!("{user_msg}: {e}")
}

pub(crate) fn format_local_date(utc: DateTime<Utc>) -> String {
    utc.with_timezone(&Local).format("%d.%m.%Y").to_string()
}

pub(crate) fn path_to_string(path: &Path) -> String {
    path.to_string_lossy().to_string()
}
