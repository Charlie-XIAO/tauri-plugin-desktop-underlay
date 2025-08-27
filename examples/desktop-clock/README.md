# Desktop Clock

This is a minimal desktop clock app. It will start a normal manager window and an underlay clock window. One can toggle the clock window between normal and underlay with the manager.

https://github.com/user-attachments/assets/894a5afb-269e-4158-af13-add266a69576

## Running the App

Clone the repository and build from source if you have not done so (see [Build from Source](../../README.md#build-from-source)). Then run the app:

```bash
pnpm -C examples/desktop-clock/ tauri dev
```

## Technical Details

- See [`src/manager/App.tsx`](./src/manager/App.tsx) for how to toggle the clock window from the manager window.
- See [`src-tauri/src/main.rs`](./src-tauri/src/main.rs) for how to set the clock window as desktop underlay on startup. In particular, in order to avoid the clock window popping up initially, it is configured to be invisible, set as underlay on startup, and then shown.
