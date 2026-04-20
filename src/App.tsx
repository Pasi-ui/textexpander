import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";

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
    <main style={{ padding: "24px", fontFamily: "system-ui", maxWidth: "600px", margin: "0 auto", backgroundColor: "#121212", color: "white", minHeight: "100vh" }}>
      <h1 style={{ fontSize: "24px", marginBottom: "24px" }}>Text Expander</h1>
      <div style={{ background: "#1a1a1a", borderRadius: "8px", padding: "20px", marginBottom: "24px" }}>
        <input
          value={trigger}
          onChange={e => setTrigger(e.target.value)}
          placeholder="Trigger"
          style={{ width: "100%", padding: "10px", borderRadius: "6px", border: "1px solid #333", background: "#111", color: "#fff", marginBottom: "10px", boxSizing: "border-box" }}
        />
        <textarea
          value={expansion}
          onChange={e => setExpansion(e.target.value)}
          placeholder="Expansion"
          rows={3}
          style={{ width: "100%", padding: "10px", borderRadius: "6px", border: "1px solid #333", background: "#111", color: "#fff", marginBottom: "10px", boxSizing: "border-box", resize: "vertical" }}
        />
        <button onClick={addShortcut} style={{ background: "#007bff", color: "#fff", border: "none", padding: "10px 20px", borderRadius: "6px", cursor: "pointer", fontWeight: "bold", width: "100%" }}>
          Add Shortcut
        </button>
      </div>
      <div>
        {shortcuts.map(s => (
          <div key={s.id} style={{ display: "flex", alignItems: "center", gap: "12px", background: "#1a1a1a", borderRadius: "8px", padding: "14px 16px", marginBottom: "8px" }}>
            <span style={{ background: "#007bff", color: "#fff", padding: "4px 10px", borderRadius: "4px", fontWeight: "bold", fontSize: "13px" }}>
              {s.trigger}
            </span>
            <span style={{ color: "#ccc", fontSize: "13px", flex: 1 }}>{s.expansion}</span>
            <button onClick={() => deleteShortcut(s.id)} style={{ background: "#dc3545", color: "#fff", border: "none", padding: "4px 10px", borderRadius: "4px", cursor: "pointer" }}>
              Delete
            </button>
          </div>
        ))}
      </div>
    </main>
  );
}

export default App;import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";

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
    <main style={{ padding: "24px", fontFamily: "system-ui", maxWidth: "600px", margin: "0 auto", backgroundColor: "#121212", color: "white", minHeight: "100vh" }}>
      <h1 style={{ fontSize: "24px", marginBottom: "24px" }}>Text Expander</h1>
      <div style={{ background: "#1a1a1a", borderRadius: "8px", padding: "20px", marginBottom: "24px" }}>
        <input
          value={trigger}
          onChange={e => setTrigger(e.target.value)}
          placeholder="Trigger"
          style={{ width: "100%", padding: "10px", borderRadius: "6px", border: "1px solid #333", background: "#111", color: "#fff", marginBottom: "10px", boxSizing: "border-box" }}
        />
        <textarea
          value={expansion}
          onChange={e => setExpansion(e.target.value)}
          placeholder="Expansion"
          rows={3}
          style={{ width: "100%", padding: "10px", borderRadius: "6px", border: "1px solid #333", background: "#111", color: "#fff", marginBottom: "10px", boxSizing: "border-box", resize: "vertical" }}
        />
        <button onClick={addShortcut} style={{ background: "#007bff", color: "#fff", border: "none", padding: "10px 20px", borderRadius: "6px", cursor: "pointer", fontWeight: "bold", width: "100%" }}>
          Add Shortcut
        </button>
      </div>
      <div>
        {shortcuts.map(s => (
          <div key={s.id} style={{ display: "flex", alignItems: "center", gap: "12px", background: "#1a1a1a", borderRadius: "8px", padding: "14px 16px", marginBottom: "8px" }}>
            <span style={{ background: "#007bff", color: "#fff", padding: "4px 10px", borderRadius: "4px", fontWeight: "bold", fontSize: "13px" }}>
              {s.trigger}
            </span>
            <span style={{ color: "#ccc", fontSize: "13px", flex: 1 }}>{s.expansion}</span>
            <button onClick={() => deleteShortcut(s.id)} style={{ background: "#dc3545", color: "#fff", border: "none", padding: "4px 10px", borderRadius: "4px", cursor: "pointer" }}>
              Delete
            </button>
          </div>
        ))}
      </div>
    </main>
  );
}

export default App;import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";

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
    <main style={{ padding: "24px", fontFamily: "system-ui", maxWidth: "600px", margin: "0 auto", backgroundColor: "#121212", color: "white", minHeight: "100vh" }}>
      <h1 style={{ fontSize: "24px", marginBottom: "24px" }}>Text Expander</h1>
      <div style={{ background: "#1a1a1a", borderRadius: "8px", padding: "20px", marginBottom: "24px" }}>
        <input
          value={trigger}
          onChange={e => setTrigger(e.target.value)}
          placeholder="Trigger"
          style={{ width: "100%", padding: "10px", borderRadius: "6px", border: "1px solid #333", background: "#111", color: "#fff", marginBottom: "10px", boxSizing: "border-box" }}
        />
        <textarea
          value={expansion}
          onChange={e => setExpansion(e.target.value)}
          placeholder="Expansion"
          rows={3}
          style={{ width: "100%", padding: "10px", borderRadius: "6px", border: "1px solid #333", background: "#111", color: "#fff", marginBottom: "10px", boxSizing: "border-box", resize: "vertical" }}
        />
        <button onClick={addShortcut} style={{ background: "#007bff", color: "#fff", border: "none", padding: "10px 20px", borderRadius: "6px", cursor: "pointer", fontWeight: "bold", width: "100%" }}>
          Add Shortcut
        </button>
      </div>
      <div>
        {shortcuts.map(s => (
          <div key={s.id} style={{ display: "flex", alignItems: "center", gap: "12px", background: "#1a1a1a", borderRadius: "8px", padding: "14px 16px", marginBottom: "8px" }}>
            <span style={{ background: "#007bff", color: "#fff", padding: "4px 10px", borderRadius: "4px", fontWeight: "bold", fontSize: "13px" }}>
              {s.trigger}
            </span>
            <span style={{ color: "#ccc", fontSize: "13px", flex: 1 }}>{s.expansion}</span>
            <button onClick={() => deleteShortcut(s.id)} style={{ background: "#dc3545", color: "#fff", border: "none", padding: "4px 10px", borderRadius: "4px", cursor: "pointer" }}>
              Delete
            </button>
          </div>
        ))}
      </div>
    </main>
  );
}

export default App;cat > /Users/pascalzimmermann/textexpander/src/App.tsx << 'EOF'
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
      <h1 style={{ fontSize: "24px", marginBottom: "24px" }}>TextExpander</h1>
      <div style={{ background: "#1a1a1a", borderRadius: "8px", padding: "20px", marginBottom: "24px" }}>
        <input
          value={trigger}
          onChange={e => setTrigger(e.target.value)}
          placeholder="Kuerzel (z.B. mfg)"
          style={{ width: "100%", padding: "10px", borderRadius: "6px", border: "1px solid #333", background: "#111", color: "#fff", marginBottom: "10px", boxSizing: "border-box" }}
        />
        <textarea
          value={expansion}
          onChange={e => setExpansion(e.target.value)}
          placeholder="Expansion"
          rows={3}
          style={{ width: "100%", padding: "10px", borderRadius: "6px", border: "1px solid #333", background: "#111", color: "#fff", marginBottom: "10px", boxSizing: "border-box", resize: "vertical" }}
        />
        <button onClick={addShortcut} style={{ background: "#6ee7b7", color: "#000", border: "none", padding: "10px 20px", borderRadius: "6px", cursor: "pointer", fontWeight: "bold", width: "100%" }}>
          Hinzufuegen
        </button>
      </div>
      <div>
        {shortcuts.map(s => (
          <div key={s.id} style={{ display: "flex", alignItems: "center", gap: "12px", background: "#1a1a1a", borderRadius: "8px", padding: "14px 16px", marginBottom: "8px" }}>
            <span style={{ background: "#6ee7b7", color: "#000", padding: "4px 10px", borderRadius: "4px", fontWeight: "bold", fontSize: "13px" }}>
              {s.trigger}
            </span>
            <span style={{ color: "#ccc", fontSize: "13px", flex: 1 }}>{s.expansion}</span>
            <button onClick={() => deleteShortcut(s.id)} style={{ background: "none", border: "1px solid #333", color: "#666", padding: "4px 10px", borderRadius: "4px", cursor: "pointer" }}>
              X
            </button>
          </div>
        ))}
      </div>
    </main>
  );
}

export default App;
EOFcat > /Users/pascalzimmermann/textexpander/src/App.tsx << 'EOF'
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
      <h1 style={{ fontSize: "24px", marginBottom: "24px" }}>TextExpander</h1>
      <div style={{ background: "#1a1a1a", borderRadius: "8px", padding: "20px", marginBottom: "24px" }}>
        <input
          value={trigger}
          onChange={e => setTrigger(e.target.value)}
          placeholder="Kuerzel (z.B. mfg)"
          style={{ width: "100%", padding: "10px", borderRadius: "6px", border: "1px solid #333", background: "#111", color: "#fff", marginBottom: "10px", boxSizing: "border-box" }}
        />
        <textarea
          value={expansion}
          onChange={e => setExpansion(e.target.value)}
          placeholder="Expansion"
          rows={3}
          style={{ width: "100%", padding: "10px", borderRadius: "6px", border: "1px solid #333", background: "#111", color: "#fff", marginBottom: "10px", boxSizing: "border-box", resize: "vertical" }}
        />
        <button onClick={addShortcut} style={{ background: "#6ee7b7", color: "#000", border: "none", padding: "10px 20px", borderRadius: "6px", cursor: "pointer", fontWeight: "bold", width: "100%" }}>
          Hinzufuegen
        </button>
      </div>
      <div>
        {shortcuts.map(s => (
          <div key={s.id} style={{ display: "flex", alignItems: "center", gap: "12px", background: "#1a1a1a", borderRadius: "8px", padding: "14px 16px", marginBottom: "8px" }}>
            <span style={{ background: "#6ee7b7", color: "#000", padding: "4px 10px", borderRadius: "4px", fontWeight: "bold", fontSize: "13px" }}>
              {s.trigger}
            </span>
            <span style={{ color: "#ccc", fontSize: "13px", flex: 1 }}>{s.expansion}</span>
            <button onClick={() => deleteShortcut(s.id)} style={{ background: "none", border: "1px solid #333", color: "#666", padding: "4px 10px", borderRadius: "4px", cursor: "pointer" }}>
              X
            </button>
          </div>
        ))}
      </div>
    </main>
  );
}

export default App;
EOFcat > /Users/pascalzimmermann/textexpander/src/App.tsx << 'EOF'
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
      <h1 style={{ fontSize: "24px", marginBottom: "24px" }}>TextExpander</h1>
      <div style={{ background: "#1a1a1a", borderRadius: "8px", padding: "20px", marginBottom: "24px" }}>
        <input
          value={trigger}
          onChange={e => setTrigger(e.target.value)}
          placeholder="Kuerzel (z.B. mfg)"
          style={{ width: "100%", padding: "10px", borderRadius: "6px", border: "1px solid #333", background: "#111", color: "#fff", marginBottom: "10px", boxSizing: "border-box" }}
        />
        <textarea
          value={expansion}
          onChange={e => setExpansion(e.target.value)}
          placeholder="Expansion"
          rows={3}
          style={{ width: "100%", padding: "10px", borderRadius: "6px", border: "1px solid #333", background: "#111", color: "#fff", marginBottom: "10px", boxSizing: "border-box", resize: "vertical" }}
        />
        <button onClick={addShortcut} style={{ background: "#6ee7b7", color: "#000", border: "none", padding: "10px 20px", borderRadius: "6px", cursor: "pointer", fontWeight: "bold", width: "100%" }}>
          Hinzufuegen
        </button>
      </div>
      <div>
        {shortcuts.map(s => (
          <div key={s.id} style={{ display: "flex", alignItems: "center", gap: "12px", background: "#1a1a1a", borderRadius: "8px", padding: "14px 16px", marginBottom: "8px" }}>
            <span style={{ background: "#6ee7b7", color: "#000", padding: "4px 10px", borderRadius: "4px", fontWeight: "bold", fontSize: "13px" }}>
              {s.trigger}
            </span>
            <span style={{ color: "#ccc", fontSize: "13px", flex: 1 }}>{s.expansion}</span>
            <button onClick={() => deleteShortcut(s.id)} style={{ background: "none", border: "1px solid #333", color: "#666", padding: "4px 10px", borderRadius: "4px", cursor: "pointer" }}>
              X
            </button>
          </div>
        ))}
      </div>
    </main>
  );
}

export default App;
