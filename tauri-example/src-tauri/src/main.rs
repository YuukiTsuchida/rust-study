#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::sync::atomic;
use tauri::Manager;
use tauri::{CustomMenuItem, Menu, MenuItem, Submenu};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn test(name: &str, age: i32) -> String {
    println!("called test function name({}), age({})", name, age);
    format!("execute test function from {}", name)
}

#[tauri::command]
fn test_return_result_err() -> Result<String, String> {
    Err("failed".into())
}

#[tauri::command]
fn test_return_result_ok() -> Result<String, String> {
    Ok("success".into())
}

#[tauri::command]
async fn async_command() -> String {
    let result = other_async_function().await;
    result
}

async fn other_async_function() -> String {
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    format!("est")
}

#[tauri::command]
fn receive_window_object(window: tauri::Window) {
    if window.is_devtools_open() {
        window.close_devtools();
    } else {
        window.open_devtools();
    }
}

#[derive(Clone, serde::Serialize)]
struct Payload {
  message: String,
}

#[tauri::command]
fn receive_app_object(app: tauri::AppHandle){
    // app.emit_all("test-event", Payload { message: "Tauri is awesome!".into() }).unwrap();
    app.get_window("main").unwrap().emit("test-event", Payload { message: "Tauri is awesome!".into() }).unwrap();
}

struct StateValue(atomic::AtomicI32);

#[tauri::command]
fn use_state(state_value: tauri::State<StateValue>) {
    let mut value = state_value.0.load(atomic::Ordering::SeqCst);
    println!(
        "state value is {}",
        value
    );
    value += 1;
    state_value.0.store(value, atomic::Ordering::SeqCst);
}

fn main() {
    let submenu = Submenu::new("File", Menu::new()
                .add_item(CustomMenuItem::new("quit".to_string(), "Quit"))
                .add_item(CustomMenuItem::new("close".to_string(), "Close"))
                .add_native_item(MenuItem::Cut));
    let submenu2 = Submenu::new("File2", Menu::new()
                .add_item(CustomMenuItem::new("quit".to_string(), "Quit"))
                .add_item(CustomMenuItem::new("close".to_string(), "Close"))
                .add_native_item(MenuItem::Cut)
                .add_native_item(MenuItem::Minimize));
    
    let menu = Menu::new()
        .add_submenu(submenu)
        .add_submenu(submenu2);
    // let menu = Menu::os_default("app_name");

    tauri::Builder::default()
        .setup(|app| {
            #[cfg(debug_assertions)] // only include this code on debug builds
            {
                let window = app.get_window("main").unwrap();
                window.open_devtools();
            }

            app.listen_global("test-event", |event| {
                println!("test-name with payload {:?}", event.payload());
            });

            // app.once_global("test-event", |event| {
            //     println!("once-global with {:?}", event.payload());
            // });
            // app.once_global(event, handler)

            let window = app.get_window("main").unwrap();
            let id = window.listen("window-event", |event| {
                println!("window event with payload {:?}", event.payload());
            });

            Ok(())
        })
        .menu(menu)
        .on_menu_event(|event|{
            match event. menu_item_id() {
                "quit" => {
                    std::process::exit(0);
                }
                "close" => {
                    event.window().close().unwrap();
                }
                _ => { }
            }
        })
        .manage(StateValue(atomic::AtomicI32::new(42)))
        .invoke_handler(tauri::generate_handler![
            greet,
            test,
            test_return_result_err,
            test_return_result_ok,
            async_command,
            receive_window_object,
            use_state,
            receive_app_object
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
