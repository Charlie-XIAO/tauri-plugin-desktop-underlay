# PLUGIN-DESKTOP-UNDERLAY

Tauri plugin to set a window as a desktop underlay. It is attached to the desktop,
staying above the wallpaper but below desktop icons.

- **Linux:** ✅ (Untested)
- **MacOS:** ✅ (Untested)
- **Windows:** ✅

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
import { setDesktopUnderlay } from "tauri-plugin-desktop-underlay-api";

setDesktopUnderlay(true);  // Set the current window as a desktop underlay
setDesktopUnderlay(false); // Reset the current window as a normal window
```

If you only intend on using the APIs from Rust code, you can import the `DesktopUnderlayExt` trait extension on windows and webview windows:

```rust
use tauri_plugin_desktop_underlay::DesktopUnderlayExt;

let main_window = app.get_window("main").unwrap();
let _ = main_window.set_desktop_underlay(true);  // Set the main window as a desktop underlay
let _ = main_window.set_desktop_underlay(false); // Reset the main window as a normal window
```

## Contributing

Issues and PRs are welcome. Since I am only using Windows for desktop development, it would be nice to get feedback from users on different platforms so that I can fix problems to the best of my abilities.

## License

Copyright (c) 2024 Yao Xiao (@Charlie-XIAO); this project is released under the [MIT License](./LICENSE).
