import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";

interface Shortcut {
  id: string;
  trigger: string;
  expansion: string;
}

function App() {
  const [shortcuts, setShortcuts] = useState<Shortcut[]>([]);
  const [searchTerm, setSearchTerm] = useState("");
  const [selectedShortcut, setSelectedShortcut] = useState<Shortcut | null>(null);
  const [formTrigger, setFormTrigger] = useState("");
  const [formExpansion, setFormExpansion] = useState("");

  useEffect(() => {
    loadShortcuts();
  }, []);

  async function loadShortcuts() {
    const result = await invoke<Shortcut[]>("get_shortcuts");
    setShortcuts(result);
  }

  async function addShortcut() {
    if (!formTrigger || !formExpansion) return;
    await invoke("add_shortcut", { trigger: formTrigger, expansion: formExpansion });
    setFormTrigger("");
    setFormExpansion("");
    setSelectedShortcut(null);
    loadShortcuts();
  }

  async function deleteShortcut(id: string) {
    await invoke("delete_shortcut", { id });
    if (selectedShortcut?.id === id) {
      setSelectedShortcut(null);
      setFormTrigger("");
      setFormExpansion("");
    }
    loadShortcuts();
  }

  function selectShortcut(shortcut: Shortcut) {
    setSelectedShortcut(shortcut);
    setFormTrigger(shortcut.trigger);
    setFormExpansion(shortcut.expansion);
  }

  function newShortcut() {
    setSelectedShortcut(null);
    setFormTrigger("");
    setFormExpansion("");
  }

  const filteredShortcuts = shortcuts.filter(shortcut =>
    shortcut.trigger.toLowerCase().includes(searchTerm.toLowerCase()) ||
    shortcut.expansion.toLowerCase().includes(searchTerm.toLowerCase())
  );

  const appStyle: React.CSSProperties = {
    display: 'flex',
    height: '100vh',
    backgroundColor: '#f5f5f7',
    fontFamily: '-apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif',
  };

  const sidebarStyle: React.CSSProperties = {
    width: '320px',
    backgroundColor: 'white',
    borderRight: '1px solid #e1e5e9',
    display: 'flex',
    flexDirection: 'column',
    boxShadow: '0 0 20px rgba(0,0,0,0.05)',
  };

  const mainContentStyle: React.CSSProperties = {
    flex: 1,
    display: 'flex',
    flexDirection: 'column',
    backgroundColor: 'white',
    margin: '20px',
    borderRadius: '12px',
    boxShadow: '0 2px 20px rgba(0,0,0,0.08)',
    overflow: 'hidden',
  };

  const headerStyle: React.CSSProperties = {
    padding: '20px 24px',
    borderBottom: '1px solid #e1e5e9',
    backgroundColor: '#fafbfc',
  };

  const searchInputStyle: React.CSSProperties = {
    width: '100%',
    padding: '8px 12px',
    border: '1px solid #d1d5db',
    borderRadius: '6px',
    fontSize: '14px',
    outline: 'none',
    backgroundColor: 'white',
  };

  const shortcutsListStyle: React.CSSProperties = {
    flex: 1,
    overflowY: 'auto',
  };

  const shortcutItemStyle: React.CSSProperties = {
    padding: '12px 24px',
    borderBottom: '1px solid #f0f0f0',
    cursor: 'pointer',
    transition: 'background-color 0.15s',
  };

  const shortcutTriggerStyle: React.CSSProperties = {
    fontWeight: '600',
    color: '#1d1d1f',
    fontSize: '14px',
    marginBottom: '4px',
  };

  const shortcutExpansionStyle: React.CSSProperties = {
    color: '#86868b',
    fontSize: '13px',
    overflow: 'hidden',
    textOverflow: 'ellipsis',
    whiteSpace: 'nowrap',
  };

  const formStyle: React.CSSProperties = {
    padding: '32px',
    flex: 1,
    display: 'flex',
    flexDirection: 'column',
  };

  const formTitleStyle: React.CSSProperties = {
    fontSize: '24px',
    fontWeight: '600',
    color: '#1d1d1f',
    marginBottom: '8px',
  };

  const formSubtitleStyle: React.CSSProperties = {
    color: '#86868b',
    marginBottom: '32px',
    fontSize: '14px',
  };

  const inputGroupStyle: React.CSSProperties = {
    marginBottom: '20px',
  };

  const labelStyle: React.CSSProperties = {
    display: 'block',
    fontSize: '14px',
    fontWeight: '500',
    color: '#1d1d1f',
    marginBottom: '8px',
  };

  const inputStyle: React.CSSProperties = {
    width: '100%',
    padding: '12px 16px',
    border: '1px solid #d1d5db',
    borderRadius: '8px',
    fontSize: '14px',
    outline: 'none',
    backgroundColor: 'white',
    transition: 'border-color 0.15s',
  };

  const textareaStyle: React.CSSProperties = {
    ...inputStyle,
    resize: 'vertical',
    minHeight: '120px',
  };

  const buttonGroupStyle: React.CSSProperties = {
    display: 'flex',
    gap: '12px',
    marginTop: 'auto',
    paddingTop: '24px',
  };

  const primaryButtonStyle: React.CSSProperties = {
    padding: '12px 24px',
    backgroundColor: '#007aff',
    color: 'white',
    border: 'none',
    borderRadius: '8px',
    fontSize: '14px',
    fontWeight: '500',
    cursor: 'pointer',
    transition: 'background-color 0.15s',
  };

  const deleteButtonStyle: React.CSSProperties = {
    padding: '6px 12px',
    backgroundColor: '#ff3b30',
    color: 'white',
    border: 'none',
    borderRadius: '6px',
    fontSize: '12px',
    fontWeight: '500',
    cursor: 'pointer',
    transition: 'background-color 0.15s',
  };

  return (
    <div style={appStyle}>
      {/* Sidebar */}
      <div style={sidebarStyle}>
        <div style={headerStyle}>
          <h1 style={{ fontSize: '18px', fontWeight: '600', color: '#1d1d1f', margin: '0 0 16px 0' }}>
            TextExpander
          </h1>
          <input
            type="text"
            placeholder="Suchen..."
            value={searchTerm}
            onChange={(e) => setSearchTerm(e.target.value)}
            style={searchInputStyle}
          />
        </div>
        <div style={shortcutsListStyle}>
          <div
            style={{
              ...shortcutItemStyle,
              backgroundColor: selectedShortcut === null ? '#e3f2fd' : 'transparent',
              borderLeft: selectedShortcut === null ? '3px solid #007aff' : 'none',
            }}
            onClick={newShortcut}
          >
            <div style={shortcutTriggerStyle}>Neues Kürzel</div>
            <div style={shortcutExpansionStyle}>Kürzel hinzufügen</div>
          </div>
          {filteredShortcuts.map((shortcut) => (
            <div
              key={shortcut.id}
              style={{
                ...shortcutItemStyle,
                backgroundColor: selectedShortcut?.id === shortcut.id ? '#e3f2fd' : 'transparent',
                borderLeft: selectedShortcut?.id === shortcut.id ? '3px solid #007aff' : 'none',
              }}
              onClick={() => selectShortcut(shortcut)}
            >
              <div style={shortcutTriggerStyle}>{shortcut.trigger}</div>
              <div style={shortcutExpansionStyle}>{shortcut.expansion}</div>
            </div>
          ))}
        </div>
      </div>

      {/* Main Content */}
      <div style={mainContentStyle}>
        <div style={formStyle}>
          <div>
            <h2 style={formTitleStyle}>
              {selectedShortcut ? 'Kürzel bearbeiten' : 'Neues Kürzel'}
            </h2>
            <p style={formSubtitleStyle}>
              {selectedShortcut ? 'Bearbeiten Sie das ausgewählte Kürzel.' : 'Erstellen Sie ein neues Textkürzel.'}
            </p>
          </div>

          <div style={inputGroupStyle}>
            <label style={labelStyle}>Kürzel</label>
            <input
              type="text"
              value={formTrigger}
              onChange={(e) => setFormTrigger(e.target.value)}
              placeholder="z.B. mfg"
              style={inputStyle}
            />
          </div>

          <div style={inputGroupStyle}>
            <label style={labelStyle}>Text</label>
            <textarea
              value={formExpansion}
              onChange={(e) => setFormExpansion(e.target.value)}
              placeholder="Der zu expandierende Text..."
              style={textareaStyle}
            />
          </div>

          <div style={buttonGroupStyle}>
            <button
              onClick={addShortcut}
              style={primaryButtonStyle}
              disabled={!formTrigger || !formExpansion}
            >
              {selectedShortcut ? 'Aktualisieren' : 'Hinzufügen'}
            </button>
            {selectedShortcut && (
              <button
                onClick={() => deleteShortcut(selectedShortcut.id)}
                style={deleteButtonStyle}
              >
                Löschen
              </button>
            )}
          </div>
        </div>
      </div>
    </div>
  );
}

export default App;
