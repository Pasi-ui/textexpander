import Cocoa
import Foundation

var buffer = ""
var shortcuts: [String: String] = [:]

func loadShortcuts() {
    let appSupport = FileManager.default.urls(for: .applicationSupportDirectory, in: .userDomainMask).first!
    let file = appSupport.appendingPathComponent("textexpander/shortcuts.json")
    if let data = try? Data(contentsOf: file),
       let json = try? JSONSerialization.jsonObject(with: data) as? [[String: String]] {
        shortcuts = [:]
        for item in json {
            if let t = item["trigger"], let e = item["expansion"] {
                shortcuts[t] = e
            }
        }
    }
}

func pasteText(_ text: String) {
    NSPasteboard.general.clearContents()
    NSPasteboard.general.setString(text, forType: .string)
    Thread.sleep(forTimeInterval: 0.05)
    let src = CGEventSource(stateID: .hidSystemState)
    let vDown = CGEvent(keyboardEventSource: src, virtualKey: 9, keyDown: true)
    let vUp = CGEvent(keyboardEventSource: src, virtualKey: 9, keyDown: false)
    vDown?.flags = .maskCommand
    vUp?.flags = .maskCommand
    vDown?.post(tap: .cghidEventTap)
    vUp?.post(tap: .cghidEventTap)
}

let eventMask = (1 << CGEventType.keyDown.rawValue)
let tap = CGEvent.tapCreate(
    tap: .cghidEventTap,
    place: .headInsertEventTap,
    options: .defaultTap,
    eventsOfInterest: CGEventMask(eventMask),
    callback: { _, _, event, _ in
        let keycode = event.getIntegerValueField(.keyboardEventKeycode)
        if keycode == 49 || keycode == 36 {
            loadShortcuts()
            if let expansion = shortcuts[buffer] {
                let count = buffer.count + 1
                for _ in 0..<count {
                    let src = CGEventSource(stateID: .hidSystemState)
                    let bsDown = CGEvent(keyboardEventSource: src, virtualKey: 51, keyDown: true)
                    let bsUp = CGEvent(keyboardEventSource: src, virtualKey: 51, keyDown: false)
                    bsDown?.post(tap: .cghidEventTap)
                    bsUp?.post(tap: .cghidEventTap)
                    Thread.sleep(forTimeInterval: 0.02)
                }
                Thread.sleep(forTimeInterval: 0.1)
                pasteText(expansion)
            }
            buffer = ""
        } else {
            let chars = event.character(unicodeStringLength: 1)
            if let c = chars, c.count == 1 {
                buffer.append(contentsOf: c)
                if buffer.count > 50 { buffer = "" }
            }
        }
        return Unmanaged.passRetained(event)
    },
    userInfo: nil
)

if let tap = tap {
    let runLoopSource = CFMachPortCreateRunLoopSource(kCFAllocatorDefault, tap, 0)
    CFRunLoopAddSource(CFRunLoopGetCurrent(), runLoopSource, .commonModes)
    CGEvent.tapEnable(tap: tap, enable: true)
    CFRunLoopRun()
}
