use tauri::{
    menu::{Menu, MenuBuilder, MenuEvent, MenuItem, PredefinedMenuItem, SubmenuBuilder}, App, AppHandle, Wry
};

use crate::menu_handlers::{*};

fn create_item(
    app: &App,
    id: &'static str,
    label: &str,
    accelerator: Option<&str>,
    handler: Option<fn(&AppHandle<Wry>, MenuEvent)>
) -> MenuItem<Wry> {

    if handler.is_some() {
        app.on_menu_event(move |app_handle: &AppHandle, event| {
            if event.id().0.as_str() == id {
                handler.unwrap()(app_handle, event);
            }
        });
    }

    return MenuItem::with_id(app, id, label, true, accelerator).unwrap();
}

pub fn create_menu(app: &App) -> Menu<Wry> {
    let file_menu = SubmenuBuilder::new(app, "File")
        .items(&[
            &create_item(app, "new_file", "New File...", Some("CmdOrCtrl+N"), Some(new_file)),
            &create_item(app, "new_window", "New Window", Some("CmdOrCtrl+Shift+N"), None),
            &PredefinedMenuItem::separator(app).unwrap(),
            &create_item(app, "open_file", "Open File...", Some("CmdOrCtrl+O"), Some(open_file)),
            &create_item(app, "open_folder", "Open Folder...", Some("CmdOrCtrl+Shift+O"), None),
            &SubmenuBuilder::new(app, "Open Recent").build().unwrap(), // Empty submenu for now
            &PredefinedMenuItem::separator(app).unwrap(),
            &create_item(app, "save", "Save", Some("CmdOrCtrl+S"), None),
            &create_item(app, "save_as", "Save As...", Some("CmdOrCtrl+Shift+S"), None),
            &PredefinedMenuItem::separator(app).unwrap(),
            &create_item(app, "quit", "Quit", Some("CmdOrCtrl+Q"), None),
        ])
        .build()
        .unwrap();

    let edit_menu = SubmenuBuilder::new(app, "Edit")
        .items(&[
            &create_item(app, "undo", "Undo", Some("CmdOrCtrl+Z"), None),
            &create_item(app, "redo", "Redo", Some("CmdOrCtrl+Y"), None),
            &PredefinedMenuItem::separator(app).unwrap(),
            &PredefinedMenuItem::cut(app, None).unwrap(),
            &PredefinedMenuItem::copy(app, None).unwrap(),
            &PredefinedMenuItem::paste(app, None).unwrap(),
            &PredefinedMenuItem::separator(app).unwrap(),
            &create_item(app, "find", "Find", Some("CmdOrCtrl+F"), None),
            &create_item(app, "replace", "Replace", Some("CmdOrCtrl+H"), None),
            &PredefinedMenuItem::separator(app).unwrap(),
            &PredefinedMenuItem::select_all(app, None).unwrap(),
        ])
        .build()
        .unwrap();

    let view_menu = SubmenuBuilder::new(app, "View")
        .items(&[
            &create_item(app, "reload", "Reload", Some("F5"), None),
            &create_item(app, "force_reload", "Force Reload", Some("CmdOrCtrl+Shift+R"), None),
            &PredefinedMenuItem::separator(app).unwrap(),
            &create_item(app, "reset_zoom", "Reset Zoom", Some("CmdOrCtrl+0"), None),
            &create_item(app, "zoom_in", "Zoom In", Some("CmdOrCtrl+Plus"), None),
            &create_item(app, "zoom_out", "Zoom Out", Some("CmdOrCtrl+-"), None),
            &PredefinedMenuItem::separator(app).unwrap(),
            &create_item(app, "toggle_dev_tools", "Toggle Developer Tools", Some("F12"), None),
            &PredefinedMenuItem::separator(app).unwrap(),
            &create_item(app, "toggle_full_screen", "Toggle Full Screen", Some("F11"), None),
        ])
        .build()
        .unwrap();

    let window_menu = SubmenuBuilder::new(app, "Window")
        .items(&[
            &create_item(app, "minimize", "Minimize", Some("CmdOrCtrl+M"), None),
            &create_item(app, "close", "Close", Some("CmdOrCtrl+W"), None),
        ])
        .build()
        .unwrap();

    let help_menu = SubmenuBuilder::new(app, "Help")
        .items(&[&create_item(app, "about", "About", None, None)])
        .build()
        .unwrap();

    return MenuBuilder::new(app)
        .items(&[&file_menu, &edit_menu, &view_menu, &window_menu, &help_menu])
        .build()
        .unwrap();
}
