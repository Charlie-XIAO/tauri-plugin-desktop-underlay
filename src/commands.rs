// Copyright 2024 Yao Xiao
// SPDX-License-Identifier: MIT

use tauri::{command, Manager, Runtime, Window};

use crate::DesktopUnderlayExt;

#[command]
pub(crate) async fn set_desktop_underlay<R: Runtime>(
    window: Window<R>,
    desktop_underlay: bool,
    label: Option<String>,
) -> tauri::Result<()> {
    let target_window = {
        if let Some(label) = label {
            window
                .get_webview_window(label.as_str())
                .ok_or(anyhow::anyhow!("Window not found: {label}"))?
                .as_ref()
                .window()
        } else {
            window
        }
    };
    target_window
        .set_desktop_underlay(desktop_underlay)
        .map_err(Into::into)
}

#[command]
pub(crate) async fn is_desktop_underlay<R: Runtime>(
    window: Window<R>,
    label: Option<String>,
) -> tauri::Result<bool> {
    let target_window = {
        if let Some(label) = label {
            window
                .get_webview_window(label.as_str())
                .ok_or(anyhow::anyhow!("Window not found: {label}"))?
                .as_ref()
                .window()
        } else {
            window
        }
    };
    Ok(target_window.is_desktop_underlay())
}
