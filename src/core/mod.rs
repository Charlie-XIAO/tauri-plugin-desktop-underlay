use std::sync::Mutex;

use anyhow::Result;
use tauri::{Manager, Runtime, Window};

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "windows")]
mod windows;

macro_rules! dispatch_to_main_thread {
    ($window:expr, $f:expr) => {{
        let (tx, rx) = std::sync::mpsc::channel();
        let window = $window.clone();
        $window.run_on_main_thread(move || {
            let _ = tx.send($f(window));
        })?;
        rx.recv()?
    }};
}

/// A Tauri state to keep track of the desktop underlay windows.
#[derive(Default)]
pub struct DesktopUnderlayState(Mutex<Vec<String>>);

fn unchecked_set_desktop_underlay<R: Runtime>(
    window: &Window<R>,
    desktop_underlay: bool,
) -> Result<()> {
    if desktop_underlay {
        dispatch_to_main_thread!(window, |w: Window<R>| -> Result<()> {
            unsafe {
                #[cfg(target_os = "linux")]
                linux::set_underlay(w.gtk_window()?)?;
                #[cfg(target_os = "macos")]
                macos::set_underlay(w.ns_window()?)?;
                #[cfg(target_os = "windows")]
                windows::set_underlay(w.hwnd()?)?;
            }
            Ok(())
        })?;
        window
            .state::<DesktopUnderlayState>()
            .0
            .lock()
            .unwrap()
            .push(window.label().to_string());
    } else {
        dispatch_to_main_thread!(window, |w: Window<R>| -> Result<()> {
            unsafe {
                #[cfg(target_os = "linux")]
                linux::unset_underlay(w.gtk_window()?)?;
                #[cfg(target_os = "macos")]
                macos::unset_underlay(w.ns_window()?)?;
                #[cfg(target_os = "windows")]
                windows::unset_underlay(w.hwnd()?)?;
            }
            Ok(())
        })?;
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
    unchecked_set_desktop_underlay(window, desktop_underlay)?;
    Ok(desktop_underlay)
}
