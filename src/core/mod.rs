use anyhow::Result;
use tauri::{Runtime, Window};

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "windows")]
mod windows;

pub(crate) fn set_desktop_underlay<R: Runtime>(
    window: &Window<R>,
    desktop_underlay: bool,
) -> Result<()> {
    #[cfg(target_os = "linux")]
    {
        let gtk_window = window.gtk_window().unwrap();
        unsafe {
            if desktop_underlay {
                linux::set_underlay(gtk_window);
            } else {
                linux::unset_underlay(gtk_window);
            }
        }
    }

    #[cfg(target_os = "macos")]
    {
        use objc::runtime::Object;

        let ns_window = window.ns_window().unwrap() as *mut Object;
        unsafe {
            if desktop_underlay {
                macos::set_underlay(ns_window);
            } else {
                macos::unset_underlay(ns_window);
            }
        }
        Ok(())
    }

    #[cfg(target_os = "windows")]
    {
        let hwnd = window.hwnd().unwrap();
        unsafe {
            if desktop_underlay {
                windows::set_underlay(hwnd)?;
            } else {
                windows::unset_underlay(hwnd)?;
            }
        }
        Ok(())
    }
}
