use std::sync::{Arc, Mutex};
use std::fs;
use tauri::Manager;

mod keyboard_hook;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
struct Shortcut {
    id: String,
    trigger: String,
    expansion: String,
}

struct AppState {
    shortcuts: Mutex<Vec<Shortcut>>,
}

fn get_shortcuts_file_path(app: &tauri::AppHandle) -> std::path::PathBuf {
    let app_data_dir = app.path().app_data_dir().unwrap();
    app_data_dir.join("shortcuts.json")
}

fn load_shortcuts(app: &tauri::AppHandle) -> Vec<Shortcut> {
    let file_path = get_shortcuts_file_path(app);
    if file_path.exists() {
        let data = fs::read_to_string(file_path).unwrap_or_default();
        serde_json::from_str(&data).unwrap_or_default()
    } else {
        vec![]
    }
}

fn save_shortcuts(app: &tauri::AppHandle, shortcuts: &Vec<Shortcut>) {
    let file_path = get_shortcuts_file_path(app);
    if let Some(parent) = file_path.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    let data = serde_json::to_string(shortcuts).unwrap();
    fs::write(file_path, data).unwrap();
}

#[tauri::command]
fn add_shortcut(app: tauri::AppHandle, state: tauri::State<Arc<AppState>>, trigger: String, expansion: String) -> String {
    let mut shortcuts = state.shortcuts.lock().unwrap();
    let id = format!("{}", shortcuts.len() + 1);
    shortcuts.push(Shortcut {
        id: id.clone(),
        trigger,
        expansion,
    });
    save_shortcuts(&app, &shortcuts);
    id
}

#[tauri::command]
fn get_shortcuts(state: tauri::State<Arc<AppState>>) -> Vec<Shortcut> {
    state.shortcuts.lock().unwrap().clone()
}

#[tauri::command]
fn delete_shortcut(app: tauri::AppHandle, state: tauri::State<Arc<AppState>>, id: String) {
    let mut shortcuts = state.shortcuts.lock().unwrap();
    shortcuts.retain(|s| s.id != id);
    save_shortcuts(&app, &shortcuts);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let shortcuts = load_shortcuts(app.handle());
            let state = Arc::new(AppState {
                shortcuts: Mutex::new(shortcuts),
            });
            app.manage(state.clone());
            crate::keyboard_hook::start_keyboard_hook(state);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            add_shortcut,
            get_shortcuts,
            delete_shortcut
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
