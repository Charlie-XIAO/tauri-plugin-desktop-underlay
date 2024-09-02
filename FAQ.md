# FAQ

This is a growing collection of FAQs.

## The window cannot receive mouse/keyboard events when set as desktop underlay.

This is the **expected behavior**: setting a window as a desktop underlay disables all types of user interactions with that window. It is indeed possible to use system APIs to listen to mouse and keyboard inputs and forward them to the window, but this is out of scope of this plugin.

Alternatively, you may consider using Tauri's `always_on_bottom` and `ignore_cursor_events` features to make a window that always stays below other windows and that the mouse can click through.
