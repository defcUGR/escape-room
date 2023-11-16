// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn verify(code: &str) -> bool {
    let true_key = std::fs::read_to_string(
        home::home_dir()
            .expect("cannot find home dir")
            .join("escape_room_key.txt"),
    )
    .expect("error reading key file");
    return code == &true_key;
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![verify])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
