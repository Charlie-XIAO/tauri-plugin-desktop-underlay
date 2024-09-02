// Copyright 2024 Yao Xiao
// SPDX-License-Identifier: MIT

use std::ptr::null_mut;

use anyhow::{bail, Result};
use windows::{
    core::s,
    Win32::{
        Foundation::{BOOL, HWND, LPARAM, WPARAM},
        UI::WindowsAndMessaging::{
            EnumWindows, FindWindowA, FindWindowExA, GetWindowLongA, SendMessageA,
            SetParent, SetWindowLongA, SystemParametersInfoA, GWL_EXSTYLE,
            SPIF_UPDATEINIFILE, SPI_SETDESKWALLPAPER, WS_EX_LAYERED,
        },
    },
};

unsafe extern "system" fn enum_window(window: HWND, ref_worker_w: LPARAM) -> BOOL {
    match FindWindowExA(window, None, s!("SHELLDLL_DefView"), None) {
        Ok(hwnd) if hwnd != HWND(null_mut()) => {},
        _ => return true.into(),
    };

    let worker_w = match FindWindowExA(None, window, s!("WorkerW"), None) {
        Ok(hwnd) if hwnd != HWND(null_mut()) => hwnd,
        _ => return true.into(),
    };

    *(ref_worker_w.0 as *mut HWND) = worker_w;
    false.into()
}

/// Set the window as a desktop underlay.
pub(super) unsafe fn set_underlay(hwnd: HWND) -> Result<()> {
    let progman = FindWindowA(s!("Progman"), None)?;
    SendMessageA(progman, 0x052C, WPARAM(0x0000000D), LPARAM(0));
    SendMessageA(progman, 0x052C, WPARAM(0x0000000D), LPARAM(1));

    // `EnumWindows` always returns the `Err` invariant even though it succeeds;
    // regardless it suffices to check if `worker_w` is updated to non-null
    let mut worker_w = HWND(null_mut());
    let _ = EnumWindows(Some(enum_window), LPARAM(&mut worker_w as *mut HWND as _));
    if worker_w == HWND(null_mut()) {
        bail!("Failed to find WorkerW");
    }

    // Set as a layered window; this will always hide the borders and title bar
    SetWindowLongA(
        hwnd,
        GWL_EXSTYLE,
        GetWindowLongA(hwnd, GWL_EXSTYLE) | (WS_EX_LAYERED.0 as i32),
    );

    SetParent(hwnd, worker_w)?;
    Ok(())
}

/// Unset the window from being a desktop underlay.
pub(super) unsafe fn unset_underlay(hwnd: HWND) -> Result<()> {
    SetParent(hwnd, HWND(null_mut()))?;

    // Reset from being a layered window
    SetWindowLongA(
        hwnd,
        GWL_EXSTYLE,
        GetWindowLongA(hwnd, GWL_EXSTYLE) & !(WS_EX_LAYERED.0 as i32),
    );

    // Refresh the desktop
    SystemParametersInfoA(
        SPI_SETDESKWALLPAPER,
        0,
        Some(null_mut()),
        SPIF_UPDATEINIFILE,
    )?;
    Ok(())
}
