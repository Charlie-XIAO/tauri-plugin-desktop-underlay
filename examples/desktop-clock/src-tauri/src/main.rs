#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{generate_context, menu::{MenuBuilder, MenuItemBuilder}, Builder, Manager};
use tauri_plugin_desktop_underlay::DesktopUnderlayExt;

fn main() {
    Builder::default()
        .setup(|app| {
            let tray = app.tray_by_id("tray").unwrap();
            let menu = MenuBuilder::new(app).items(&[&MenuItemBuilder::with_id("toggle", "Toggle").build(app)?]).build()?;
            tray.set_menu(Some(menu)).unwrap();

            // Toggle when the menu item is clicked
            tray.on_menu_event(move |app_handle, event| match event.id().as_ref() {
                "toggle" => {
                    let main_window = app_handle.get_webview_window("main").unwrap();
                    main_window.set_desktop_underlay(!main_window.is_desktop_underlay()).unwrap();
                },
                _ => {},
            });

            Ok(())
        })
        .plugin(tauri_plugin_desktop_underlay::init())
        .run(generate_context!())
        .expect("Error while initializing the application");
}
