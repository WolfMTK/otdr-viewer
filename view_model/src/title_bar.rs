use crate::window::WindowCommand;

pub const MENU_ITEMS: [&str; 4] = ["Файл", "Редактирование", "Вид", "Справка"];
const FILE_MENU: usize = 0;

#[derive(Debug, Default)]
pub struct TitleBarViewModel {
    menu_open: bool,
    maximized: bool,
    open_file_requested: bool,
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

    pub fn select_menu_item(&mut self, index: usize) {
        self.close_menu();
        if index == FILE_MENU {
            self.open_file_requested = true;
        }
    }

    pub fn take_open_file_request(&mut self) -> bool {
        std::mem::take(&mut self.open_file_requested)
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

#[cfg(test)]
mod tests {
    use crate::title_bar::TitleBarViewModel;
    use crate::window::WindowCommand;

    fn with_open_menu() -> TitleBarViewModel {
        let mut vm = TitleBarViewModel::new();
        vm.open_menu();
        vm
    }

    #[test]
    fn menu_closed_by_default() {
        assert!(!TitleBarViewModel::new().menu_open());
    }

    #[test]
    fn selecting_item_closes_menu() {
        let mut vm = with_open_menu();
        vm.select_menu_item(2);
        assert!(!vm.menu_open());
    }

    #[test]
    fn file_menu_requests_open_once() {
        let mut vm = with_open_menu();
        vm.select_menu_item(0);
        assert!(vm.take_open_file_request());
        assert!(!vm.take_open_file_request());
    }

    #[test]
    fn other_menu_items_do_not_request_open() {
        let mut vm = with_open_menu();
        vm.select_menu_item(1);
        assert!(!vm.take_open_file_request());
    }

    #[test]
    fn start_drag_closes_menu_and_emits_command() {
        let mut vm = with_open_menu();
        vm.start_drag();
        assert!(!vm.menu_open());
        assert_eq!(vm.take_commands(), [WindowCommand::StartDrag]);
    }

    #[test]
    fn toggle_maximize_inverts_current_state() {
        let mut vm = TitleBarViewModel::new();
        vm.toggle_maximize();
        vm.set_maximized(true);
        vm.toggle_maximize();
        assert_eq!(
            vm.take_commands(),
            [
                WindowCommand::SetMaximized(true),
                WindowCommand::SetMaximized(false)
            ]
        );
    }

    #[test]
    fn take_commands_drains_queue_in_order() {
        let mut vm = TitleBarViewModel::new();
        vm.minimize();
        vm.close_window();
        assert_eq!(vm.take_commands(), [WindowCommand::Minimize, WindowCommand::Close]);
        assert!(vm.take_commands().is_empty());
    }
}
