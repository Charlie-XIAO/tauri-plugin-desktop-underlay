// Copyright 2024 Yao Xiao
// SPDX-License-Identifier: MIT

use anyhow::{bail, Result};
use windows::core::{s, BOOL};
use windows::Win32::Foundation::{HWND, LPARAM, WPARAM};
use windows::Win32::UI::WindowsAndMessaging::{
    EnumWindows, FindWindowA, FindWindowExA, SendMessageTimeoutA, SetParent, SMTO_NORMAL,
};

/// Helper function to find the WorkerW window.
unsafe extern "system" fn enum_window(window: HWND, ref_worker_w: LPARAM) -> BOOL {
    let shell_dll_def_view =
        FindWindowExA(Some(window), None, s!("SHELLDLL_DefView"), None).unwrap_or_default();
    if HWND::is_invalid(&shell_dll_def_view) {
        return true.into();
    }

    let worker_w = FindWindowExA(None, Some(window), s!("WorkerW"), None).unwrap_or_default();
    if HWND::is_invalid(&worker_w) {
        return true.into();
    }

    *(ref_worker_w.0 as *mut HWND) = worker_w;
    true.into()
}

/// Set the window as a desktop underlay.
pub(super) unsafe fn set_underlay(hwnd: HWND) -> Result<()> {
    let progman = FindWindowA(s!("Progman"), None)?;
    SendMessageTimeoutA(
        progman,
        0x052C,
        WPARAM(0x0000000D),
        LPARAM(0x00000001),
        SMTO_NORMAL,
        1000,
        None,
    );

    let mut worker_w = HWND::default();
    EnumWindows(Some(enum_window), LPARAM(&mut worker_w as *mut HWND as _))?;
    if HWND::is_invalid(&worker_w) {
        worker_w = FindWindowExA(Some(progman), None, s!("WorkerW"), None)?;
        if HWND::is_invalid(&worker_w) {
            bail!("Failed to find WorkerW");
        }
    }

    SetParent(hwnd, Some(worker_w))?;
    Ok(())
}

/// Unset the window from being a desktop underlay.
pub(super) unsafe fn unset_underlay(hwnd: HWND) -> Result<()> {
    SetParent(hwnd, None)?;
    Ok(())
}
