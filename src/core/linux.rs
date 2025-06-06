use anyhow::Result;
use gdk::WindowTypeHint;
use gtk::prelude::GtkWindowExt;
use gtk::ApplicationWindow;

/// Set the window as a desktop underlay.
pub(super) unsafe fn set_underlay(gtk_window: ApplicationWindow) -> Result<()> {
    gtk_window.set_type_hint(WindowTypeHint::Desktop);
    Ok(())
}

/// Unset the window from being a desktop underlay.
pub(super) unsafe fn unset_underlay(gtk_window: ApplicationWindow) -> Result<()> {
    gtk_window.set_type_hint(WindowTypeHint::Normal);
    Ok(())
}
