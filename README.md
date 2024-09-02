# PLUGIN-DESKTOP-UNDERLAY

Tauri plugin for making a window a desktop underlay, attached to the desktop, above the wallpaper, and below desktop icons.

- **Linux:** ✅ (Untested)
- **MacOS:** ✅ (Untested)
- **Windows:** ✅

## Examples

- [Desktop Clock](https://github.com/Charlie-XIAO/tauri-plugin-desktop-underlay/tree/main/examples/desktop-clock)

## Install

Install the core plugin by adding the following to your `src-tauri/Cargo.toml` file:

```toml
[dependencies]
tauri-plugin-desktop-underlay = "0.0.1"
```

You can install the JavaScript guest bindings using your preferred JavaScript package manager:

```bash
npm|yarn|pnpm add tauri-plugin-desktop-underlay-api
```

## Build from Source

If you want to try a local build, or you want to try the [examples](#examples) before determining whether to use this plugin, you may need to build the source code:

```bash
git clone https://github.com/Charlie-XIAO/tauri-plugin-desktop-underlay.git
pnpm install
pnpm build
cargo check
```

## Usage

First you need to register the core plugin with Tauri:

```rust
// src-tauri/src/main.rs

fn main() {
    tauri::Builder::default()
        // Initialize the desktop-underlay plugin
        .plugin(tauri_plugin_desktop_underlay::init())
        .run(tauri::generate_context!())
        .unwrap();
}
```

Afterwards all the plugin's APIs are available through the JavaScript guest bindings:

```typescript
import {
  setDesktopUnderlay,
  isDesktopUnderlay,
} from "tauri-plugin-desktop-underlay-api";

// --- Operate on the current window ---

// Determine if it is desktop underlay
const isUnderlay = await isDesktopUnderlay();

// Set as desktop underlay or reset to normal
await setDesktopUnderlay(true);
await setDesktopUnderlay(false);

// Application: Toggle between two modes
await setDesktopUnderlay(!(await isDesktopUnderlay()));

// --- Operate on another window with label "wallpaper" ---

// Determine if it is desktop underlay
const isUnderlay = await isDesktopUnderlay("wallpaper");

// Set as desktop underlay or reset to normal
await setDesktopUnderlay(true, "wallpaper");
await setDesktopUnderlay(false, "wallpaper");

// Application: Toggle between two modes
await setDesktopUnderlay(!(await isDesktopUnderlay("wallpaper")), "wallpaper");
```

If you only intend on using the APIs from Rust code, you can import the `DesktopUnderlayExt` extension on windows and webview windows:

```rust
use tauri_plugin_desktop_underlay::DesktopUnderlayExt;

let main_window = app.get_webview_window("main").unwrap();
let _ = main_window.set_desktop_underlay(true);
let _ = main_window.set_desktop_underlay(false);
```

## FAQ

See [FAQ](https://github.com/Charlie-XIAO/tauri-plugin-desktop-underlay/tree/main/FAQ.md).

## Contributing

Issues and PRs are welcome. Since I am only using Windows for desktop development, it would be nice to get feedback from users on different platforms so that I can fix problems to the best of my abilities.

## License

Copyright (c) 2024 Yao Xiao [@Charlie-XIAO](https://github.com/Charlie-XIAO); this project is released under the [MIT License](https://github.com/Charlie-XIAO/tauri-plugin-desktop-underlay/tree/main/LICENSE).
