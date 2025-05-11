use std::sync::Mutex;

use anyhow::Result;
use tauri::{Manager, Runtime, Window};

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "windows")]
mod windows;

/// A Tauri state to keep track of the desktop underlay windows.
#[derive(Default)]
pub struct DesktopUnderlayState(Mutex<Vec<String>>);

fn unchecked_set_desktop_underlay<R: Runtime>(
    window: &Window<R>,
    desktop_underlay: bool,
) -> Result<()> {
    if desktop_underlay {
        unsafe {
            #[cfg(target_os = "linux")]
            linux::set_underlay(window.gtk_window()?)?;
            #[cfg(target_os = "macos")]
            macos::set_underlay(window.ns_window()?)?;
            #[cfg(target_os = "windows")]
            windows::set_underlay(window.hwnd()?)?;
        }
        window
            .state::<DesktopUnderlayState>()
            .0
            .lock()
            .unwrap()
            .push(window.label().to_string());
    } else {
        unsafe {
            #[cfg(target_os = "linux")]
            linux::unset_underlay(window.gtk_window()?)?;
            #[cfg(target_os = "macos")]
            macos::unset_underlay(window.ns_window()?)?;
            #[cfg(target_os = "windows")]
            windows::unset_underlay(window.hwnd()?)?;
        }
        window
            .state::<DesktopUnderlayState>()
            .0
            .lock()
            .unwrap()
            .retain(|label| label != &window.label().to_string());
    }

    Ok(())
}

pub fn is_desktop_underlay<R: Runtime>(window: &Window<R>) -> bool {
    window
        .state::<DesktopUnderlayState>()
        .0
        .lock()
        .unwrap()
        .contains(&window.label().to_string())
}

pub fn set_desktop_underlay<R: Runtime>(window: &Window<R>, desktop_underlay: bool) -> Result<()> {
    if desktop_underlay == is_desktop_underlay(window) {
        return Ok(());
    }
    unchecked_set_desktop_underlay(window, desktop_underlay)
}

pub fn toggle_desktop_underlay<R: Runtime>(window: &Window<R>) -> Result<bool> {
    let desktop_underlay = !is_desktop_underlay(window);
    set_desktop_underlay(window, desktop_underlay)?;
    Ok(desktop_underlay)
}
