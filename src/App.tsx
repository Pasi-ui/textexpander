import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

interface Shortcut {
  id: string;
  trigger: string;
  expansion: string;
}

function App() {
  const [shortcuts, setShortcuts] = useState<Shortcut[]>([]);
  const [trigger, setTrigger] = useState("");
  const [expansion, setExpansion] = useState("");

  async function loadShortcuts() {
    const result = await invoke<Shortcut[]>("get_shortcuts");
    setShortcuts(result);
  }

  async function addShortcut() {
    if (!trigger || !expansion) return;
    await invoke("add_shortcut", { trigger, expansion });
    setTrigger("");
    setExpansion("");
    loadShortcuts();
  }

  async function deleteShortcut(id: string) {
    await invoke("delete_shortcut", { id });
    loadShortcuts();
  }

  useEffect(() => { loadShortcuts(); }, []);

  return (
    <main style={{ padding: "24px", fontFamily: "system-ui", maxWidth: "600px", margin: "0 auto" }}>
      <h1 style={{ fontSize: "24px", marginBottom: "24px" }}>⌨️ TextExpander</h1>

      <div style={{ background: "#1a1a1a", borderRadius: "8px", padding: "20px", marginBottom: "24px" }}>
        <h2 style={{ fontSize: "14px", marginBottom: "16px", color: "#888" }}>NEUES KÜRZEL</h2>
        <input
          value={trigger}
          onChange={e => setTrigger(e.target.value)}
          placeholder="Kürzel (z.B. mfg)"
          style={{ width: "100%", padding: "10px", borderRadius: "6px", border: "1px solid #333", background: "#111", color: "#fff", marginBottom: "10px", boxSizing: "border-box" }}
        />
        <textarea
          value={expansion}
          onChange={e => setExpansion(e.target.value)}
          placeholder="Expansion (z.B. Mit freundlichen Grüßen)"
          rows={3}
          style={{ width: "100%", padding: "10px", borderRadius: "6px", border: "1px solid #333", background: "#111", color: "#fff", marginBottom: "10px", boxSizing: "border-box", resize: "vertical" }}
        />
        <button
          onClick={addShortcut}
          style={{ background: "#6ee7b7", color: "#000", border: "none", padding: "10px 20px", borderRadius: "6px", cursor: "pointer", fontWeight: "600", width: "100%" }}
          