use serde::Serialize;
use tauri::{CustomMenuItem, Menu, Submenu, WindowMenuEvent};

pub fn create_menu() -> Menu {
    Menu::new()
        .add_submenu(create_file_menu())
        .add_item(CustomMenuItem::new("about".to_string(), "About"))
}

fn create_file_menu() -> Submenu {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let new_connection = CustomMenuItem::new("new_connection".to_string(), "New Connection");
    let database_to_application = CustomMenuItem::new(
        "generate_application".to_string(),
        "Generate Application from Database",
    );
    let import = CustomMenuItem::new("import".to_string(), "Import");
    let export = CustomMenuItem::new("export".to_string(), "Export");
    let bump_log: CustomMenuItem = CustomMenuItem::new("bump_log".to_string(),"Bump Log");
    Submenu::new(
        "File",
        Menu::new()
            .add_item(new_connection)
            .add_item(import)
            .add_item(export)
            .add_item(database_to_application)
            .add_item(bump_log)
            .add_item(quit),
    )
}

#[derive(Clone, Serialize)]
struct MenuCommandBump(bool);

pub fn handle_menu_event(event: WindowMenuEvent) {
    match event.menu_item_id() {
        "quit" => {
            std::process::exit(0);
        }
        "new_connection" => event
            .window()
            .emit("new_connection", MenuCommandBump(true))
            .expect("Command new connection failed from menu"),
        "import" => event
            .window()
            .emit("import", MenuCommandBump(true))
            .expect("Command import failed from menu"),
        "export" => event
            .window()
            .emit("export", MenuCommandBump(true))
            .expect("Command export failed from menu"),
        "bump_log" => event
            .window()
            .emit("bump_log", MenuCommandBump(true))
            .expect("Command bump_log failed from menu"),
        "database_to_application" => event
            .window()
            .emit("database_to_application", MenuCommandBump(true))
            .expect("Command database to application failed from menu"),
        _ => {}
    };
}
