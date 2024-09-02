# Desktop Clock

This is a minimal desktop clock app. Its window can be toggled between being a normal window and being a desktop underlay, either through the manager window or through the tray icon.

## Running the App

Clone the repository and build from source if you have not done so (see [Build from Source](../../README.md#build-from-source)). Then run the app:

```bash
cd examples/desktop-clock
pnpm install
pnpm tauri dev
```

## Technical Details

- [`src/Manager.tsx`](./src/Manager.tsx): Toggle in the frontend.
- [`src-tauri/src/main.rs`](./src-tauri/src/main.rs): Toggle in the backend.
