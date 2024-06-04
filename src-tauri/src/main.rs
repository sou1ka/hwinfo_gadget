// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::api::dialog;
use tauri::{CustomMenuItem, Manager, SystemTray, SystemTrayMenu, SystemTrayMenuItem, SystemTrayEvent};
use std::collections::HashMap;
use std::vec::Vec;
use winreg::enums::*;
use winreg::RegKey;
use regex::Regex;

#[tauri::command]
fn get_hwifo() -> String {
    let hwinfo = RegKey::predef(HKEY_CURRENT_USER).open_subkey_with_flags("SOFTWARE\\HWiNFO64\\VSB", KEY_READ).unwrap();
    let enums = hwinfo.enum_values();
    let mut list: Vec<HashMap<String, String>> = Vec::new();
    let mut count = 0;
    let mut map: HashMap<String, String> = HashMap::new();

    for (name, value) in enums.map(|x| x.unwrap()) {
        let reg = Regex::new(r"\d+").unwrap();
        let cap = reg.captures(&name).unwrap();
        let num: usize = cap[0].parse().unwrap();
        let nm = name.replace(&num.to_string(), "");

        if count != num {
            count = num;

            if !map.is_empty() {
                list.push(map.clone());
            }
        }

        map.insert(nm, value.to_string());
    }

    list.push(map.clone()); // last

    format!("{}", serde_json::to_string(&list).unwrap())
}

fn main() {
    let exit = CustomMenuItem::new("exit".to_string(), "Exit");
    let vers = CustomMenuItem::new("vers".to_string(), "Version");
    let view = CustomMenuItem::new("view".to_string(), "View");
    let lock = CustomMenuItem::new("lock".to_string(), "Lock Window");
    let top = CustomMenuItem::new("top".to_string(), "Always Top");
    let tray_menu = SystemTrayMenu::new()
        .add_item(view)
        .add_item(lock)
        .add_item(top)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(vers)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(exit.clone());

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_hwifo])
        .setup(|app| {
            let window = app.app_handle().get_window("hwinfo_gadget").unwrap();
            let app_handle = app.app_handle();
            std::thread::spawn(move || loop {
                app_handle.emit_all("hwinfo_refresh", get_hwifo()).unwrap();
                std::thread::sleep(std::time::Duration::from_millis(500));
            });

            Ok(())
        })
        .system_tray(SystemTray::new().with_menu(tray_menu))
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::DoubleClick {
                position: _,
                size: _,
                ..
            } => {
                let window = app.get_window("hwinfo_gadget").unwrap();
                window.unminimize().unwrap();
                window.show().unwrap();
                window.set_focus().unwrap();
                window.set_decorations(true).unwrap();
                window.set_always_on_top(false).unwrap();
            }
            SystemTrayEvent::MenuItemClick { id, .. } => {
                match id.as_str() {
                "view" => {
                    let window = app.get_window("hwinfo_gadget").unwrap();
                    window.unminimize().unwrap();
                    window.show().unwrap();
                    window.set_focus().unwrap();
                    window.set_decorations(true).unwrap();
                    window.set_always_on_top(false).unwrap();
                }
                "vers" => {
                    let window = app.get_window("hwinfo_gadget").unwrap();
                    let context = tauri::generate_context!();
                    let package = context.package_info();
                    let msg: String = format!("HWiNFO Gadget\r\n\r\nVersion {}\n\rAuthor: sou1ka @sou1ka", package.version);
                    dialog::message(Some(&window), &package.name, &msg);
                }
                "lock" => {
                    let window = app.get_window("hwinfo_gadget").unwrap();
                    window.set_decorations(false).unwrap();
                }
                "top" => {
                    let window = app.get_window("hwinfo_gadget").unwrap();
                    window.set_always_on_top(true).unwrap();
                }
                "exit" => {
                    std::process::exit(0);
                }
                _ => {}
                }
            }
            _ => {}
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
