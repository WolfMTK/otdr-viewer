#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowCommand {
    Minimize,
    SetMaximized(bool),
    StartDrag,
    Close,
}
