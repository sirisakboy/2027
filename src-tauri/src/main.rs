#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod ecu;
mod rom;
mod serial;
mod commands;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![
            commands::list_ports,
            commands::connect_ecu,
            commands::read_rom,
            commands::write_rom,
            commands::calculate_cc,
            commands::get_ecu_info,
            commands::reset_flash_count,
            commands::simulate_flash
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
