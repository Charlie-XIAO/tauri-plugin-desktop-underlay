import { invoke } from "@tauri-apps/api/core";

/**
 * Check whether the window is desktop underlay.
 *
 * @param label - The label of the window. If not provided, the current window
 *  will be used.
 * @returns Whether the window is desktop underlay.
 */
export async function isDesktopUnderlay(label?: string): Promise<boolean> {
  return await invoke<boolean>("plugin:desktop-underlay|is_desktop_underlay", {
    label,
  });
}

/**
 * Set the window as desktop underlay or revert it to normal.
 *
 * @param desktopUnderlay If `true`, the window will be set as desktop underlay
 *  (no-op if it already is). If `false`, the window will be reverted to normal
 *  (no-op if it is not yet desktop underlay).
 * @param label The label of the window. If not provided, the current window
 *  will be used.
 */
export async function setDesktopUnderlay(desktopUnderlay: boolean, label?: string): Promise<void> {
  return await invoke<void>("plugin:desktop-underlay|set_desktop_underlay", {
    desktopUnderlay,
    label,
  });
}

/**
 * Toggle the desktop underlay state of the window.
 *
 * @param label The label of the window. If not provided, the current window
 *  will be used.
 * @returns Whether the window is desktop underlay after the operation.
 */
export async function toggleDesktopUnderlay(label?: string): Promise<boolean> {
  return await invoke<boolean>("plugin:desktop-underlay|toggle_desktop_underlay", {
    label,
  });
}
