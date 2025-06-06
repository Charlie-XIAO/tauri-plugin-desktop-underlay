use anyhow::Result;
use tauri::{Runtime, WebviewWindow, Window};

/// A window extension that provides desktop underlay functionalities.
pub trait DesktopUnderlayExt {
    /// Check whether the window is desktop underlay.
    fn is_desktop_underlay(&self) -> bool;

    /// Set the window as desktop underlay or revert it to normal.
    ///
    /// If `desktop_underlay` is `true`, the window will be set as desktop
    /// underlay (no-op if it already is). If `desktop_underlay` is `false`,
    /// the window will be reverted to normal (no-op if it is not yet
    /// desktop underlay).
    fn set_desktop_underlay(&self, desktop_underlay: bool) -> Result<()>;

    /// Toggle the desktop underlay state of the window.
    ///
    /// If the window is currently desktop underlay, it will be reverted to
    /// normal. Otherwise, it will be set as desktop underlay. Returns whether
    /// the window is desktop underlay after the operation.
    fn toggle_desktop_underlay(&self) -> Result<bool>;
}

impl<R: Runtime> DesktopUnderlayExt for Window<R> {
    fn is_desktop_underlay(&self) -> bool {
        crate::core::is_desktop_underlay(self)
    }

    fn set_desktop_underlay(&self, desktop_underlay: bool) -> Result<()> {
        crate::core::set_desktop_underlay(self, desktop_underlay)
    }

    fn toggle_desktop_underlay(&self) -> Result<bool> {
        crate::core::toggle_desktop_underlay(self)
    }
}

impl<R: Runtime> DesktopUnderlayExt for WebviewWindow<R> {
    fn is_desktop_underlay(&self) -> bool {
        self.as_ref().window().is_desktop_underlay()
    }

    fn set_desktop_underlay(&self, desktop_underlay: bool) -> Result<()> {
        self.as_ref()
            .window()
            .set_desktop_underlay(desktop_underlay)
    }

    fn toggle_desktop_underlay(&self) -> Result<bool> {
        self.as_ref().window().toggle_desktop_underlay()
    }
}
