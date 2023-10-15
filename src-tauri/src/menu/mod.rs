pub mod file;

use tauri::{CustomMenuItem, Menu, MenuItem, Submenu};

pub fn build() -> Menu {
    let open_file = CustomMenuItem::new("open-file", "Open");

    let submenu = Submenu::new(
        "File",
        Menu::new()
            .add_item(open_file)
            .add_native_item(MenuItem::Separator)
            .add_native_item(MenuItem::CloseWindow)
            .add_native_item(MenuItem::Quit),
    );

    Menu::new().add_submenu(submenu)
}
