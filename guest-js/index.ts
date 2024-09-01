import { invoke } from "@tauri-apps/api/core";

/**
 * Determines if the window should be a desktop underlay.
 *
 * @param desktopUnderlay Whether the window should be a desktop underlay.
 */
export async function setDesktopUnderlay(desktopUnderlay: boolean): Promise<void> {
  return await invoke<void>("plugin:desktop-underlay|set_desktop_underlay", {
    desktopUnderlay,
  });
}
