use tauri::{
    menu::{Menu, MenuBuilder, MenuItem, PredefinedMenuItem, SubmenuBuilder}, App, Wry
};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn create_menu_item(app: &App, id: &str, label: &str, accelerator: Option<&str>) -> MenuItem<Wry> {
    return MenuItem::with_id(app, id, label, true, accelerator).unwrap();
}

fn create_menu(app: &App) -> Menu<Wry> {

    let file_menu = SubmenuBuilder::new(app, "File")
        .items(&[
            &create_menu_item(app, "new_file", "New File...", Some("CmdOrCtrl+N")),
            &create_menu_item(app, "new_window", "New Window", Some("CmdOrCtrl+Shift+N")),
            &PredefinedMenuItem::separator(app).unwrap(),
            &create_menu_item(app, "open_file", "Open File...", Some("CmdOrCtrl+O")),
            &create_menu_item(app, "open_folder", "Open Folder...", Some("CmdOrCtrl+Shift+O")),
            &SubmenuBuilder::new(app, "Open Recent").build().unwrap(), // Empty submenu for now
            &PredefinedMenuItem::separator(app).unwrap(),
            &create_menu_item(app, "save", "Save", Some("CmdOrCtrl+S")),
            &create_menu_item(app, "save_as", "Save As...", Some("CmdOrCtrl+Shift+S")),
            &PredefinedMenuItem::separator(app).unwrap(),
            &create_menu_item(app, "quit", "Quit", Some("CmdOrCtrl+Q")),
        ])
        .build()
        .unwrap();

    let edit_menu = SubmenuBuilder::new(app, "Edit")
        .items(&[
            &create_menu_item(app, "undo", "Undo", Some("CmdOrCtrl+Z")),
            &create_menu_item(app, "redo", "Redo", Some("CmdOrCtrl+Y")),
            &PredefinedMenuItem::separator(app).unwrap(),
            &PredefinedMenuItem::cut(app, None).unwrap(),
            &PredefinedMenuItem::copy(app, None).unwrap(),
            &PredefinedMenuItem::paste(app, None).unwrap(),
            &PredefinedMenuItem::separator(app).unwrap(),
            &create_menu_item(app, "find", "Find", Some("CmdOrCtrl+F")),
            &create_menu_item(app, "replace", "Replace", Some("CmdOrCtrl+H")),
            &PredefinedMenuItem::separator(app).unwrap(),
            &PredefinedMenuItem::select_all(app, None).unwrap(),
        ])
        .build()
        .unwrap();

    let view_menu = SubmenuBuilder::new(app, "View")
        .items(&[
            &create_menu_item(app, "reload", "Reload", Some("F5")),
            &create_menu_item(app, "force_reload", "Force Reload", Some("F5")),
            &PredefinedMenuItem::separator(app).unwrap(),
            &create_menu_item(app, "reset_zoom", "Reset Zoom", Some("CmdOrCtrl+0")),
            &create_menu_item(app, "zoom_in", "Zoom In", Some("CmdOrCtrl+Plus")),
            &create_menu_item(app, "zoom_out", "Zoom Out", Some("CmdOrCtrl+-")),
            &PredefinedMenuItem::separator(app).unwrap(),
            &create_menu_item(app, "toggle_dev_tools", "Toggle Developer Tools", Some("F12")),
            &PredefinedMenuItem::separator(app).unwrap(),
            &create_menu_item(app, "toggle_full_screen", "Toggle Full Screen", Some("F11")),
        ])
        .build()
        .unwrap();

    let window_menu = SubmenuBuilder::new(app, "Window")
        .items(&[
            &create_menu_item(app, "minimize", "Minimize", Some("CmdOrCtrl+M")),
            &create_menu_item(app, "close", "Close", Some("CmdOrCtrl+W")),
        ])
        .build()
        .unwrap();

    let help_menu = SubmenuBuilder::new(app, "Help")
        .items(&[
            &create_menu_item(app, "about", "About", None),
        ])
        .build()
        .unwrap();

    return MenuBuilder::new(app).items(&[
        &file_menu,
        &edit_menu,
        &view_menu,
        &window_menu,
        &help_menu
    ]).build().unwrap();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            app.set_menu(create_menu(app))?;
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
