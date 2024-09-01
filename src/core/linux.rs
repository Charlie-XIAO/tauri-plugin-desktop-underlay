use gdk::WindowTypeHint;
use gtk::{prelude::GtkWindowExt, ApplicationWindow};

/// Set the window as a desktop underlay.
pub(super) unsafe fn set_underlay(gtk_window: ApplicationWindow) {
    gtk_window.set_type_hint(WindowTypeHint::Desktop);
}

/// Unset the window from being a desktop underlay.
pub(super) unsafe fn unset_underlay(gtk_window: ApplicationWindow) {
    gtk_window.set_type_hint(WindowTypeHint::Normal);
}
