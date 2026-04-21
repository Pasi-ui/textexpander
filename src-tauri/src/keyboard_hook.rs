use std::collections::VecDeque;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use core_foundation::runloop::CFRunLoopRun;
use core_graphics::event::{CGEvent, CGEventType, CGKeyCode};
use objc::msg_send;
use objc::sel;
use objc::sel_impl;

use crate::AppState;

#[link(name = "ApplicationServices", kind = "framework")]
extern "C" {
    fn CGEventTapCreate(
        tap: u32,
        place: u32,
        options: u32,
        eventsOfInterest: u64,
        callback: extern "C" fn(
            proxy: *mut std::ffi::c_void,
            type_: CGEventType,
            event: *mut CGEvent,
            userInfo: *mut std::ffi::c_void,
        ) -> *mut CGEvent,
        userInfo: *mut std::ffi::c_void,
    ) -> *mut std::ffi::c_void;

    fn CFMachPortCreateRunLoopSource(
        allocator: *mut std::ffi::c_void,
        port: *mut std::ffi::c_void,
        order: i32,
    ) -> *mut std::ffi::c_void;

    fn CFRunLoopAddSource(rl: *mut std::ffi::c_void, source: *mut std::ffi::c_void, mode: *mut std::ffi::c_void);
    fn CFRunLoopGetCurrent() -> *mut std::ffi::c_void;

    fn CGEventGetIntegerValueField(event: *mut std::ffi::c_void, field: u32) -> i64;
    fn CGEventSetFlags(event: *mut std::ffi::c_void, flags: u64);
}

const KCGAnnotatedSessionEventTap: u32 = 2;
const KCGHeadInsertEventTap: u32 = 0;
const KCGEventTapOptionDefault: u32 = 0;
const KCGEventMaskForAllEvents: u64 = 0xFFFFFFFFFFFFFFFF;
const KCGEventFlagMaskCommand: u64 = 0x00000008;

static mut BUFFER: VecDeque<char> = VecDeque::new();
static mut STATE: Option<Arc<AppState>> = None;

extern "C" fn event_callback(
    _proxy: *mut std::ffi::c_void,
    type_: CGEventType,
    event: *mut CGEvent,
    _user_info: *mut std::ffi::c_void,
) -> *mut CGEvent {
    if type_ as u32 == 10 { // CGEventType::KeyDown
        unsafe {
            let event_ref = &*event;
            let keycode = CGEventGetIntegerValueField(event as *mut _, 9) as CGKeyCode; // kCGKeyboardEventKeycode
            let flags = CGEventGetIntegerValueField(event as *mut _, 17) as u64; // kCGKeyboardEventFlags

            // Convert keycode to char (simplified, only for letters and space)
            let ch = keycode_to_char(keycode, flags);

            if let Some(ch) = ch {
                BUFFER.push_back(ch);
                if BUFFER.len() > 100 {
                    BUFFER.pop_front();
                }

                // Check for triggers
                if let Some(state) = &STATE {
                    let shortcuts = state.shortcuts.lock().unwrap();
                    for shortcut in &*shortcuts {
                        let trigger_with_space = format!("{} ", shortcut.trigger);
                        let buffer_str: String = BUFFER.iter().collect();
                        if buffer_str.ends_with(&trigger_with_space) {
                            // Delete exactly trigger.length + 1 characters (trigger word + space)
                            let delete_count = shortcut.trigger.len() + 1;
                            for _ in 0..delete_count {
                                let source = core_graphics::event_source::CGEventSource::new(
                                    core_graphics::event_source::CGEventSourceStateID::HIDSystemState,
                                )
                                .unwrap();
                                let delete_down = CGEvent::new_keyboard_event(source.clone(), 0x33, true).unwrap();
                                delete_down.post(core_graphics::event::CGEventTapLocation::AnnotatedSession);
                                let delete_up = CGEvent::new_keyboard_event(source, 0x33, false).unwrap();
                                delete_up.post(core_graphics::event::CGEventTapLocation::AnnotatedSession);
                            }

                            // Check if expansion contains any non-ASCII characters or uppercase
                            let has_unicode = shortcut.expansion.chars().any(|c| {
                                !c.is_ascii() || (c.is_alphabetic() && c.is_uppercase())
                            });

                            // Add a delay to ensure all deletes are processed before next action
                            thread::sleep(Duration::from_millis(100));

                            if has_unicode {
                                // Use pasteboard for Unicode/mixed case text
                                if set_pasteboard_string(&shortcut.expansion) {
                                    // Add delay after setting clipboard before pasting
                                    thread::sleep(Duration::from_millis(50));
                                    paste_from_pasteboard();
                                }
                            } else {
                                // Use keyboard events for ASCII-only text
                                type_string_with_shift(&shortcut.expansion);
                            }

                            // Clear buffer
                            BUFFER.clear();
                            break;
                        }
                    }
                }
            }
        }
    }

    event
}

fn keycode_to_char(keycode: CGKeyCode, _flags: u64) -> Option<char> {
    match keycode {
        0x00 => Some('a'),
        0x01 => Some('s'),
        0x02 => Some('d'),
        0x03 => Some('f'),
        0x04 => Some('h'),
        0x05 => Some('g'),
        0x06 => Some('z'),
        0x07 => Some('x'),
        0x08 => Some('c'),
        0x09 => Some('v'),
        0x0B => Some('b'),
        0x0C => Some('q'),
        0x0D => Some('w'),
        0x0E => Some('e'),
        0x0F => Some('r'),
        0x10 => Some('y'),
        0x11 => Some('t'),
        0x12 => Some('1'),
        0x13 => Some('2'),
        0x14 => Some('3'),
        0x15 => Some('4'),
        0x16 => Some('6'),
        0x17 => Some('5'),
        0x18 => Some('='),
        0x19 => Some('9'),
        0x1A => Some('7'),
        0x1B => Some('-'),
        0x1C => Some('8'),
        0x1D => Some('0'),
        0x1E => Some(']'),
        0x1F => Some('o'),
        0x20 => Some('u'),
        0x21 => Some('['),
        0x22 => Some('i'),
        0x23 => Some('p'),
        0x24 => Some('\n'),
        0x25 => Some('l'),
        0x26 => Some('j'),
        0x27 => Some('\''),
        0x28 => Some('k'),
        0x29 => Some(';'),
        0x2A => Some('\\'),
        0x2B => Some(','),
        0x2C => Some('/'),
        0x2D => Some('n'),
        0x2E => Some('m'),
        0x2F => Some('.'),
        0x30 => Some('\t'),
        0x31 => Some(' '),
        0x32 => Some('`'),
        0x33 => Some('\x08'), // backspace
        _ => None,
    }
}

fn char_to_keycode(c: char) -> Option<(CGKeyCode, bool)> {
    // Returns (keycode, needs_shift)
    match c {
        'a' => Some((0x00, false)),
        's' => Some((0x01, false)),
        'd' => Some((0x02, false)),
        'f' => Some((0x03, false)),
        'h' => Some((0x04, false)),
        'g' => Some((0x05, false)),
        'z' => Some((0x06, false)),
        'x' => Some((0x07, false)),
        'c' => Some((0x08, false)),
        'v' => Some((0x09, false)),
        'b' => Some((0x0B, false)),
        'q' => Some((0x0C, false)),
        'w' => Some((0x0D, false)),
        'e' => Some((0x0E, false)),
        'r' => Some((0x0F, false)),
        'y' => Some((0x10, false)),
        't' => Some((0x11, false)),
        '1' => Some((0x12, false)),
        '2' => Some((0x13, false)),
        '3' => Some((0x14, false)),
        '4' => Some((0x15, false)),
        '6' => Some((0x16, false)),
        '5' => Some((0x17, false)),
        '=' => Some((0x18, false)),
        '9' => Some((0x19, false)),
        '7' => Some((0x1A, false)),
        '-' => Some((0x1B, false)),
        '8' => Some((0x1C, false)),
        '0' => Some((0x1D, false)),
        ']' => Some((0x1E, false)),
        'o' => Some((0x1F, false)),
        'u' => Some((0x20, false)),
        '[' => Some((0x21, false)),
        'i' => Some((0x22, false)),
        'p' => Some((0x23, false)),
        'l' => Some((0x25, false)),
        'j' => Some((0x26, false)),
        '\'' => Some((0x27, false)),
        'k' => Some((0x28, false)),
        ';' => Some((0x29, false)),
        '\\' => Some((0x2A, false)),
        ',' => Some((0x2B, false)),
        '/' => Some((0x2C, false)),
        'n' => Some((0x2D, false)),
        'm' => Some((0x2E, false)),
        '.' => Some((0x2F, false)),
        '\t' => Some((0x30, false)),
        ' ' => Some((0x31, false)),
        '`' => Some((0x32, false)),
        // Uppercase letters with Shift
        'A' => Some((0x00, true)),
        'S' => Some((0x01, true)),
        'D' => Some((0x02, true)),
        'F' => Some((0x03, true)),
        'H' => Some((0x04, true)),
        'G' => Some((0x05, true)),
        'Z' => Some((0x06, true)),
        'X' => Some((0x07, true)),
        'C' => Some((0x08, true)),
        'V' => Some((0x09, true)),
        'B' => Some((0x0B, true)),
        'Q' => Some((0x0C, true)),
        'W' => Some((0x0D, true)),
        'E' => Some((0x0E, true)),
        'R' => Some((0x0F, true)),
        'Y' => Some((0x10, true)),
        'T' => Some((0x11, true)),
        'O' => Some((0x1F, true)),
        'U' => Some((0x20, true)),
        'I' => Some((0x22, true)),
        'P' => Some((0x23, true)),
        'L' => Some((0x25, true)),
        'J' => Some((0x26, true)),
        'K' => Some((0x28, true)),
        'N' => Some((0x2D, true)),
        'M' => Some((0x2E, true)),
        _ => None,
    }
}

fn set_pasteboard_string(text: &str) -> bool {
    unsafe {
        let pb_class = objc::runtime::Class::get("NSPasteboard");
        if let Some(pb_class) = pb_class {
            let pasteboard: *mut objc::runtime::Object = msg_send![pb_class, generalPasteboard];
            if !pasteboard.is_null() {
                let ns_string_class = objc::runtime::Class::get("NSString");
                if let Some(ns_string_class) = ns_string_class {
                    let ns_string: *mut objc::runtime::Object = 
                        msg_send![ns_string_class, stringWithUTF8String: text.as_ptr() as *const i8];
                    if !ns_string.is_null() {
                        let pb_type_class = objc::runtime::Class::get("NSPasteboardTypeString");
                        if let Some(pb_type_class) = pb_type_class {
                            let _: () = msg_send![pasteboard, clearContents];
                            let result: bool = msg_send![pasteboard, setString:ns_string forType:pb_type_class];
                            return result;
                        }
                    }
                }
            }
        }
    }
    false
}

fn type_string_with_shift(text: &str) {
    for c in text.chars() {
        if let Some((keycode, needs_shift)) = char_to_keycode(c) {
            let source = core_graphics::event_source::CGEventSource::new(
                core_graphics::event_source::CGEventSourceStateID::HIDSystemState,
            )
            .unwrap();
            
            if needs_shift {
                // Send Shift down
                let shift_down = CGEvent::new_keyboard_event(source.clone(), 0x38, true).unwrap();
                shift_down.post(core_graphics::event::CGEventTapLocation::AnnotatedSession);
            }
            
            let key_down = CGEvent::new_keyboard_event(source.clone(), keycode, true).unwrap();
            key_down.post(core_graphics::event::CGEventTapLocation::AnnotatedSession);
            let key_up = CGEvent::new_keyboard_event(source.clone(), keycode, false).unwrap();
            key_up.post(core_graphics::event::CGEventTapLocation::AnnotatedSession);
            
            if needs_shift {
                // Send Shift up
                let shift_up = CGEvent::new_keyboard_event(source, 0x38, false).unwrap();
                shift_up.post(core_graphics::event::CGEventTapLocation::AnnotatedSession);
            }
        }
    }
}

unsafe fn paste_from_pasteboard() {
    // Send Cmd+V to paste using CGEventSetFlags
    let source = core_graphics::event_source::CGEventSource::new(
        core_graphics::event_source::CGEventSourceStateID::HIDSystemState,
    )
    .unwrap();
    
    // V keycode is 0x09
    let v_keycode: CGKeyCode = 0x09;
    
    // Create V key down event with Command flag
    let v_down = CGEvent::new_keyboard_event(source.clone(), v_keycode, true).unwrap();
    CGEventSetFlags(&v_down as *const _ as *mut std::ffi::c_void, KCGEventFlagMaskCommand);
    v_down.post(core_graphics::event::CGEventTapLocation::AnnotatedSession);
    
    // Small delay between key down and key up for proper sequencing
    thread::sleep(Duration::from_millis(20));
    
    // Create V key up event with Command flag
    let v_up = CGEvent::new_keyboard_event(source, v_keycode, false).unwrap();
    CGEventSetFlags(&v_up as *const _ as *mut std::ffi::c_void, KCGEventFlagMaskCommand);
    v_up.post(core_graphics::event::CGEventTapLocation::AnnotatedSession);
}

pub fn start_keyboard_hook(state: Arc<AppState>) {
    unsafe {
        STATE = Some(state);
    }

    thread::spawn(|| {
        unsafe {
            let tap = CGEventTapCreate(
                KCGAnnotatedSessionEventTap,
                KCGHeadInsertEventTap,
                KCGEventTapOptionDefault,
                KCGEventMaskForAllEvents,
                event_callback,
                std::ptr::null_mut(),
            );

            if tap.is_null() {
                eprintln!("Failed to create event tap");
                return;
            }

            let source = CFMachPortCreateRunLoopSource(std::ptr::null_mut(), tap, 0);
            let rl = CFRunLoopGetCurrent();
            CFRunLoopAddSource(rl, source, core_foundation::runloop::kCFRunLoopCommonModes as *mut _);

            CFRunLoopRun();
        }
    });
}