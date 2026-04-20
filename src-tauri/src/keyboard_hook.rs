use std::collections::VecDeque;
use std::sync::Arc;
use std::thread;

use core_foundation::runloop::CFRunLoopRun;
use core_graphics::event::{CGEvent, CGEventFlags, CGEventType, CGKeyCode};

use crate::AppState;

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
}

const KCGHIDEventTap: u32 = 0;
const KCGHeadInsertEventTap: u32 = 0;
const KCGEventTapOptionDefault: u32 = 0;
const KCGEventMaskForAllEvents: u64 = 0xFFFFFFFFFFFFFFFF;

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
                            // Delete the trigger
                            for _ in 0..trigger_with_space.len() {
                                // Simulate backspace
                                let delete_event = CGEvent::new_keyboard_event(
                                    core_graphics::event_source::CGEventSource::new(
                                        core_graphics::event_source::CGEventSourceStateID::HIDSystemState,
                                    )
                                    .unwrap(),
                                    51, // backspace keycode
                                    true,
                                )
                                .unwrap();
                                delete_event.post(core_graphics::event::CGEventTapLocation::HID);
                            }

                            // Type the expansion
                            for c in shortcut.expansion.chars() {
                                let keycode = char_to_keycode(c);
                                if let Some(kc) = keycode {
                                    let key_event = CGEvent::new_keyboard_event(
                                        core_graphics::event_source::CGEventSource::new(
                                            core_graphics::event_source::CGEventSourceStateID::HIDSystemState,
                                        )
                                        .unwrap(),
                                        kc,
                                        true,
                                    )
                                    .unwrap();
                                    key_event.post(core_graphics::event::CGEventTapLocation::HID);
                                }
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
    // Simplified mapping, only lowercase letters and space
    match keycode {
        0 => Some('a'),
        1 => Some('s'),
        2 => Some('d'),
        3 => Some('f'),
        4 => Some('h'),
        5 => Some('g'),
        6 => Some('z'),
        7 => Some('x'),
        8 => Some('c'),
        9 => Some('v'),
        10 => Some('b'),
        11 => Some('q'),
        12 => Some('w'),
        13 => Some('e'),
        14 => Some('r'),
        15 => Some('y'),
        16 => Some('t'),
        17 => Some('1'),
        18 => Some('2'),
        19 => Some('3'),
        20 => Some('4'),
        21 => Some('6'),
        22 => Some('5'),
        23 => Some('='),
        24 => Some('9'),
        25 => Some('7'),
        26 => Some('-'),
        27 => Some('8'),
        28 => Some('0'),
        29 => Some(']'),
        30 => Some('o'),
        31 => Some('u'),
        32 => Some('['),
        33 => Some('i'),
        34 => Some('p'),
        35 => Some('l'),
        36 => Some('j'),
        37 => Some('\''),
        38 => Some('k'),
        39 => Some(';'),
        40 => Some('\\'),
        41 => Some(','),
        42 => Some('/'),
        43 => Some('n'),
        44 => Some('m'),
        45 => Some('.'),
        46 => Some('\t'),
        47 => Some(' '),
        48 => Some('`'),
        49 => Some('\n'), // return
        _ => None,
    }
}

fn char_to_keycode(c: char) -> Option<CGKeyCode> {
    match c {
        'a' => Some(0),
        's' => Some(1),
        'd' => Some(2),
        'f' => Some(3),
        'h' => Some(4),
        'g' => Some(5),
        'z' => Some(6),
        'x' => Some(7),
        'c' => Some(8),
        'v' => Some(9),
        'b' => Some(10),
        'q' => Some(11),
        'w' => Some(12),
        'e' => Some(13),
        'r' => Some(14),
        'y' => Some(15),
        't' => Some(16),
        '1' => Some(17),
        '2' => Some(18),
        '3' => Some(19),
        '4' => Some(20),
        '6' => Some(21),
        '5' => Some(22),
        '=' => Some(23),
        '9' => Some(24),
        '7' => Some(25),
        '-' => Some(26),
        '8' => Some(27),
        '0' => Some(28),
        ']' => Some(29),
        'o' => Some(30),
        'u' => Some(31),
        '[' => Some(32),
        'i' => Some(33),
        'p' => Some(34),
        'l' => Some(35),
        'j' => Some(36),
        '\'' => Some(37),
        'k' => Some(38),
        ';' => Some(39),
        '\\' => Some(40),
        ',' => Some(41),
        '/' => Some(42),
        'n' => Some(43),
        'm' => Some(44),
        '.' => Some(45),
        '\t' => Some(46),
        ' ' => Some(47),
        '`' => Some(48),
        '\n' => Some(49),
        _ => None,
    }
}

pub fn start_keyboard_hook(state: Arc<AppState>) {
    unsafe {
        STATE = Some(state);
    }

    thread::spawn(|| {
        unsafe {
            let tap = CGEventTapCreate(
                KCGHIDEventTap,
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