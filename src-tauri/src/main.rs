#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use anyhow::Result;
use bull::bulls::{BType, Bull, Bulls};
use plate::plates::Plate;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(num: usize) -> Result<Vec<Bull>, String> {
    if num <= 0 {
        return Err("must have a number of bull".into());
    }
    // println!("==========={}", num);
    Ok(Bulls::new().deal(num).unwrap())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
