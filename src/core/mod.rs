// Copyright 2024 Yao Xiao
// SPDX-License-Identifier: MIT

use std::sync::Mutex;

use anyhow::Result;
use tauri::{Manager, Runtime, Window};

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "windows")]
mod windows;

#[derive(Default)]
pub(crate) struct DesktopUnderlayState(Mutex<Vec<String>>);

macro_rules! internal_set_desktop_underlay {
    ($window:expr, $desktop_underlay:expr, $set_underlay:expr, $unset_underlay:expr) => {{
        unsafe {
            if $desktop_underlay {
                $set_underlay()?;
                $window
                    .state::<DesktopUnderlayState>()
                    .0
                    .lock()
                    .unwrap()
                    .push($window.label().to_string());
            } else {
                $unset_underlay()?;
                $window
                    .state::<DesktopUnderlayState>()
                    .0
                    .lock()
                    .unwrap()
                    .retain(|label| label != &$window.label().to_string());
            }
        }
    }};
}

pub(crate) fn set_desktop_underlay<R: Runtime>(
    window: &Window<R>,
    desktop_underlay: bool,
) -> Result<()> {
    if desktop_underlay == is_desktop_underlay(window) {
        // Either the window is already a desktop underlay and we are trying to set it
        // again, or the window is not a desktop underlay and we are trying to unset it;
        // in either case, we perform no operation
        return Ok(());
    }

    #[cfg(target_os = "linux")]
    {
        let gtk_window = window.gtk_window().unwrap();
        internal_set_desktop_underlay!(
            window,
            desktop_underlay,
            || linux::set_underlay(gtk_window),
            || linux::unset_underlay(gtk_window)
        );
        Ok(())
    }

    #[cfg(target_os = "macos")]
    {
        let ns_window = window.ns_window().unwrap() as *mut objc::runtime::Object;
        internal_set_desktop_underlay!(
            window,
            desktop_underlay,
            || macos::set_underlay(ns_window),
            || macos::unset_underlay(ns_window)
        );
        Ok(())
    }

    #[cfg(target_os = "windows")]
    {
        let hwnd = window.hwnd().unwrap();
        internal_set_desktop_underlay!(
            window,
            desktop_underlay,
            || windows::set_underlay(hwnd),
            || windows::unset_underlay(hwnd)
        );
        Ok(())
    }
}

pub(crate) fn is_desktop_underlay<R: Runtime>(window: &Window<R>) -> bool {
    window
        .state::<DesktopUnderlayState>()
        .0
        .lock()
        .unwrap()
        .contains(&window.label().to_string())
}
