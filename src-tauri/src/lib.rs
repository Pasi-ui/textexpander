use std::sync::{Arc, Mutex};
use std::fs;
use tauri::Manager;
use std::thread;
use enigo::KeyboardControllable;

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

fn start_keyboard_hook(state: Arc<AppState>) {
    thread::spawn(move || {
        use rdev::{listen, Event, EventType, Key};
        
        let mut buffer = String::new();
        let mut enigo = enigo::Enigo::new();

        let _ = listen(move |event: Event| {
            match event.event_type {
                EventType::KeyPress(key) => {
                    // Convert key to character if possible
                    let char_opt = match key {
                        Key::Num0 => Some('0'),
                        Key::Num1 => Some('1'),
                        Key::Num2 => Some('2'),
                        Key::Num3 => Some('3'),
                        Key::Num4 => Some('4'),
                        Key::Num5 => Some('5'),
                        Key::Num6 => Some('6'),
                        Key::Num7 => Some('7'),
                        Key::Num8 => Some('8'),
                        Key::Num9 => Some('9'),
                        Key::KeyA => Some('a'),
                        Key::KeyB => Some('b'),
                        Key::KeyC => Some('c'),
                        Key::KeyD => Some('d'),
                        Key::KeyE => Some('e'),
                        Key::KeyF => Some('f'),
                        Key::KeyG => Some('g'),
                        Key::KeyH => Some('h'),
                        Key::KeyI => Some('i'),
                        Key::KeyJ => Some('j'),
                        Key::KeyK => Some('k'),
                        Key::KeyL => Some('l'),
                        Key::KeyM => Some('m'),
                        Key::KeyN => Some('n'),
                        Key::KeyO => Some('o'),
                        Key::KeyP => Some('p'),
                        Key::KeyQ => Some('q'),
                        Key::KeyR => Some('r'),
                        Key::KeyS => Some('s'),
                        Key::KeyT => Some('t'),
                        Key::KeyU => Some('u'),
                        Key::KeyV => Some('v'),
                        Key::KeyW => Some('w'),
                        Key::KeyX => Some('x'),
                        Key::KeyY => Some('y'),
                        Key::KeyZ => Some('z'),
                        _ => None,
                    };

                    if let Some(c) = char_opt {
                        buffer.push(c);
                        
                        // Read current shortcuts from Mutex
                        let shortcuts = state.shortcuts.lock().unwrap();
                        
                        // Check if any shortcut trigger matches the buffer
                        for shortcut in shortcuts.iter() {
                            if buffer.ends_with(&shortcut.trigger) {
                                // Remove the trigger from buffer by pressing backspace
                                for _ in 0..shortcut.trigger.len() {
                                    enigo.key_down(enigo::Key::Backspace);
                                    enigo.key_up(enigo::Key::Backspace);
                                }
                                
                                // Type the expansion
                                for c in shortcut.expansion.chars() {
                                    enigo.key_click(enigo::Key::Layout(c));
                                }
                                
                                // Clear buffer after match
                                buffer.clear();
                                break;
                            }
                        }
                        
                        // Limit buffer size to prevent excessive memory usage
                        if buffer.len() > 100 {
                            buffer.remove(0);
                        }
                    }
                }
                _ => {}
            }
        });
    });
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
            
            // Start the keyboard hook thread
            start_keyboard_hook(state);
            
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
