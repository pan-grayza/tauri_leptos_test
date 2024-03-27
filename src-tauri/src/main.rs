// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::env;
use tauri::SystemTray;

mod helpers;

// use windows::{core::Result, Win32::Foundation::*};
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    let os_info: String = vec![env::consts::OS, env::consts::ARCH, env::consts::FAMILY]
        .into_iter()
        .map(|s| s.to_string() + " ")
        .collect();

    format!(
        "Hello, {}! You've been greeted from Rust! \n here is your OS info: {}",
        name, os_info
    )
}

#[tauri::command]
fn is_light_theme() -> String {
    let theme = helpers::is_light_theme();
    format!("{}", theme)
}

fn main() {
    let system_tray = SystemTray::new();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, is_light_theme])
        .system_tray(system_tray)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
