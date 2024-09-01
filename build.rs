const COMMANDS: &[&str] = &["set_desktop_underlay"];

fn main() {
    tauri_plugin::Builder::new(COMMANDS).build();
}
