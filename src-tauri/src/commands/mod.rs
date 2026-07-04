pub(crate) mod files;
pub(crate) mod recent_files;
pub(crate) mod sor;
pub(crate) mod window;

pub(crate) fn command_error(op: &str, user_msg: &str, e: impl std::fmt::Display) -> String {
    log::error!("{op}: {e}");
    format!("{user_msg}: {e}")
}
