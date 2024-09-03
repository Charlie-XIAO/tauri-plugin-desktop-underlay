// Copyright 2024 Yao Xiao
// SPDX-License-Identifier: MIT

//! Tauri plugin for making a window a desktop underlay, attached to the desktop, above
//! the wallpaper, and below desktop icons.

use anyhow::Result;
use tauri::{Runtime, WebviewWindow, Window};

/// A window extension that provides desktop underlay functionalities.
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

impl<R: Runtime> DesktopUnderlayExt for Window<R> {
    fn set_desktop_underlay(&self, desktop_underlay: bool) -> Result<()> {
        crate::core::set_desktop_underlay(self, desktop_underlay)
    }

    fn is_desktop_underlay(&self) -> bool {
        crate::core::is_desktop_underlay(self)
    }
}

impl<R: Runtime> DesktopUnderlayExt for WebviewWindow<R> {
    fn set_desktop_underlay(&self, desktop_underlay: bool) -> Result<()> {
        self.as_ref().window().set_desktop_underlay(desktop_underlay)
    }

    fn is_desktop_underlay(&self) -> bool {
        self.as_ref().window().is_desktop_underlay()
    }
}
