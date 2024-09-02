import { setDesktopUnderlay, isDesktopUnderlay } from "tauri-plugin-desktop-underlay-api";
import "./Manager.css";

function Manager() {
  async function toggleDesktopUnderlay() {
    await setDesktopUnderlay(!await isDesktopUnderlay("main"), "main");
  }

  return (
    <div className="container">
      <button className="action" onClick={toggleDesktopUnderlay}>Toggle</button>
      <div className="caption">
        Click the button to toggle the main clock window between normal and desktop
        underlay modes. Alternatively, you can find in the system tray a Tauri icon
        saying "Desktop Clock". Right click to open and select "Toggle" from the menu to
        achieve the same effect.
      </div>
    </div>
  );
}

export default Manager;
