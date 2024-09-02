// Copyright 2024 Yao Xiao
// SPDX-License-Identifier: MIT

use anyhow::Result;
use tauri::{
    command, generate_handler,
    plugin::{Builder, TauriPlugin},
    Manager, Runtime, WebviewWindow, Window,
};

mod core;

/// A window extension that provides desktop underlay functionalities.
///
/// Implemented for [`WebviewWindow`] and [`Window`].
pub trait DesktopUnderlayExt {
    /// Set the window as desktop underlay or revert it to normal.
    ///
    /// If `desktop_underlay` is `true`, the window will be set as desktop underlay
    /// (no-op if it already is). If `desktop_underlay` is `false`, the window will be
    /// reverted to normal (no-op if it is not yet desktop underlay).
    fn set_desktop_underlay(&self, desktop_underlay: bool) -> Result<()>;

    /// Check if the window is desktop underlay.
    fn is_desktop_underlay(&self) -> bool;
}

impl<R: Runtime> DesktopUnderlayExt for WebviewWindow<R> {
    fn set_desktop_underlay(&self, desktop_underlay: bool) -> Result<()> {
        self.as_ref().window().set_desktop_underlay(desktop_underlay)
    }

    fn is_desktop_underlay(&self) -> bool {
        self.as_ref().window().is_desktop_underlay()
    }
}

impl<R: Runtime> DesktopUnderlayExt for Window<R> {
    fn set_desktop_underlay(&self, desktop_underlay: bool) -> Result<()> {
        core::set_desktop_underlay(self, desktop_underlay)
    }

    fn is_desktop_underlay(&self) -> bool {
        core::is_desktop_underlay(self)
    }
}

#[command]
async fn set_desktop_underlay<R: Runtime>(
    window: Window<R>,
    desktop_underlay: bool,
    label: Option<String>,
) -> tauri::Result<()> {
    let target_window = {
        if let Some(label) = label {
            window
                .get_webview_window(label.as_str())
                .ok_or(anyhow::anyhow!("Window not found: {label}"))?
                .as_ref()
                .window()
        } else {
            window
        }
    };
    target_window.set_desktop_underlay(desktop_underlay).map_err(Into::into)
}

#[command]
async fn is_desktop_underlay<R: Runtime>(
    window: Window<R>,
    label: Option<String>,
) -> tauri::Result<bool> {
    let target_window = {
        if let Some(label) = label {
            window
                .get_webview_window(label.as_str())
                .ok_or(anyhow::anyhow!("Window not found: {label}"))?
                .as_ref()
                .window()
        } else {
            window
        }
    };
    Ok(target_window.is_desktop_underlay())
}

/// Initialize the desktop-underlay plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("desktop-underlay")
        .setup(|app_handle, _api| {
            app_handle.manage(core::DesktopUnderlayState::default());
            Ok(())
        })
        .invoke_handler(generate_handler![set_desktop_underlay, is_desktop_underlay])
        .build()
}
