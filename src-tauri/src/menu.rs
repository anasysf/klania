use tauri::{Menu, CustomMenuItem, Submenu, MenuItem};

pub fn make_menu() -> Menu {
    Menu::new()
        .add_submenu(make_file_submenu())
}

fn make_file_submenu() -> Submenu {
    let open_file = CustomMenuItem::new("open-file", "Open")
        .accelerator("CommandOrControl+O");

    Submenu::new(
        "File",
        Menu::new()
            .add_item(open_file)
            .add_native_item(MenuItem::Separator)
            .add_native_item(MenuItem::Quit),
    )
}
