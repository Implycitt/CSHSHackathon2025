use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io::Write;

mod config;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![config::test])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
