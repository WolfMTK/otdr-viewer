use crate::window::WindowCommand;

pub const MENU_ITEMS: [&str; 4] = ["Файл", "Редактирование", "Вид", "Справка"];

#[derive(Debug, Default)]
pub struct TitleBarViewModel {
    menu_open: bool,
    maximized: bool,
    commands: Vec<WindowCommand>,
}

impl TitleBarViewModel {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn menu_open(&self) -> bool {
        self.menu_open
    }

    pub fn menu_items(&self) -> &'static [&'static str] {
        &MENU_ITEMS
    }

    pub fn set_maximized(&mut self, maximized: bool) {
        self.maximized = maximized;
    }

    pub fn take_commands(&mut self) -> Vec<WindowCommand> {
        std::mem::take(&mut self.commands)
    }

    pub fn open_menu(&mut self) {
        self.menu_open = true;
    }

    pub fn close_menu(&mut self) {
        self.menu_open = false;
    }

    pub fn select_menu_item(&mut self, _index: usize) {
        self.close_menu();
    }

    pub fn minimize(&mut self) {
        self.commands.push(WindowCommand::Minimize);
    }

    pub fn toggle_maximize(&mut self) {
        self.commands
            .push(WindowCommand::SetMaximized(!self.maximized));
    }

    pub fn close_window(&mut self) {
        self.commands.push(WindowCommand::Close);
    }

    pub fn start_drag(&mut self) {
        self.close_menu();
        self.commands.push(WindowCommand::StartDrag);
    }
}
