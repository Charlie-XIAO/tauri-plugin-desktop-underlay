#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{Builder, Manager, generate_context};
use tauri_plugin_desktop_underlay::DesktopUnderlayExt;

fn main() {
    Builder::default()
        .setup(|app| {
            let clock = app.get_webview_window("clock").unwrap();
            clock.set_desktop_underlay(true)?;
            println!(
                "Desktop underlay enabled for clock window: {}",
                clock.is_desktop_underlay()
            );
            clock.show()?;
            Ok(())
        })
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_desktop_underlay::init())
        .run(generate_context!())
        .expect("Error while initializing the application");
}
