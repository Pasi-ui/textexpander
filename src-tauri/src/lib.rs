use std::sync::{Arc, Mutex};

#[derive(Clone, serde::Serialize)]
struct Shortcut {
    id: String,
    trigger: String,
    expansion: String,
}

struct AppState {
    shortcuts: Mutex<Vec<Shortcut>>,
}

#[tauri::command]
fn add_shortcut(state: tauri::State<Arc<AppState>>, trigger: String, expansion: String) -> String {
    let mut shortcuts = state.shortcuts.lock().unwrap();
    let id = format!("{}", shortcuts.len() + 1);
    shortcuts.push(Shortcut { id: id.clone(), trigger, expansion });
    id
}

#[tauri::command]
fn get_shortcuts(state: tauri::State<Arc<AppState>>) -> Vec<Shortcut> {
    state.shortcuts.lock().unwrap().clone()
}

#[tauri::command]
fn delete_shortcut(state: tauri::State<Arc<AppState>>, id: String) {
    let mut shortcuts = state.shortcuts.lock().unwrap();
    shortcuts.retain(|s| s.id != id);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let state = Arc::new(AppState {
        shortcuts: Mutex::new(vec![
            Shortcut { id: "1".into(), trigger: "mfg".into(), expansion: "Mit freundlichen Grüßen".into() },
            Shortcut { id: "2".into(), trigger: "lg".into(), expansion: "Liebe Grüße".into() },
        ]),
    });

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(state)
        .invoke_handler(tauri::generate_handler![add_shortcut, get_shortcuts, delete_shortcut])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}