// Copyright 2024 Yao Xiao
// SPDX-License-Identifier: MIT

use anyhow::Result;
use tauri::{
    command, generate_handler,
    plugin::{Builder, TauriPlugin},
    Runtime, WebviewWindow, Window,
};

mod core;

/// A window extension that provides desktop underlay functionalities.
///
/// Implemented for [`WebviewWindow`] and [`Window`].
pub trait DesktopUnderlayExt {
    /// Determines if the window should be a desktop underlay.
    fn set_desktop_underlay(&self, desktop_underlay: bool) -> Result<()>;
}

impl<R: Runtime> DesktopUnderlayExt for WebviewWindow<R> {
    fn set_desktop_underlay(&self, desktop_underlay: bool) -> Result<()> {
        self.as_ref().window().set_desktop_underlay(desktop_underlay)
    }
}

impl<R: Runtime> DesktopUnderlayExt for Window<R> {
    fn set_desktop_underlay(&self, desktop_underlay: bool) -> Result<()> {
        core::set_desktop_underlay(self, desktop_underlay)
    }
}

#[command]
async fn set_desktop_underlay<R: Runtime>(
    window: Window<R>,
    desktop_underlay: bool,
) -> tauri::Result<()> {
    window.set_desktop_underlay(desktop_underlay).map_err(Into::into)
}

/// Initialize the desktop-underlay plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("desktop-underlay")
        .invoke_handler(generate_handler![set_desktop_underlay])
        .build()
}
