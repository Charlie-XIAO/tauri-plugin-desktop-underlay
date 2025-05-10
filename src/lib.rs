// Copyright 2024 Yao Xiao
// SPDX-License-Identifier: MIT

use tauri::plugin::{Builder, TauriPlugin};
use tauri::{generate_handler, Manager, Runtime};

mod commands;
mod core;
mod ext;

pub use ext::DesktopUnderlayExt;

/// Initialize the desktop-underlay plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("desktop-underlay")
        .setup(|app_handle, _api| {
            app_handle.manage(core::DesktopUnderlayState::default());
            Ok(())
        })
        .invoke_handler(generate_handler![
            commands::set_desktop_underlay,
            commands::is_desktop_underlay
        ])
        .build()
}
