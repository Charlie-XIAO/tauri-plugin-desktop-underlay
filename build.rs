const COMMANDS: &[&str] = &[
    "is_desktop_underlay",
    "set_desktop_underlay",
    "toggle_desktop_underlay",
];

fn main() {
    tauri_plugin::Builder::new(COMMANDS).build();
}
