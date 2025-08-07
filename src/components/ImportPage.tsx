import { useState, useEffect } from 'react';
import { useNavigate } from 'react-router-dom';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { getCurrentWindow, ProgressBarStatus } from '@tauri-apps/api/window';
import { isPermissionGranted, requestPermission, sendNotification } from '@tauri-apps/plugin-notification';

function ImportPage() {
  const navigate = useNavigate();
  const [selectedFile, setSelectedFile] = useState<string>('');
  const [password, setPassword] = useState('');
  const [isLoading, setIsLoading] = useState(false);
  const [message, setMessage] = useState('');
  const [, setImportProgress] = useState(0);
  const [profiles, setProfiles] = useState<string[]>([]);
  const [selectedUser, setSelectedUser] = useState('');
  const [progress, setProgress] = useState<{percent:number, eta_ms:number, phase:string}>({percent:0, eta_ms:0, phase:'Idle'});
  const [detected, setDetected] = useState<{desktop:boolean;documents:boolean;pictures:boolean;downloads:boolean;chrome:boolean;edge:boolean;firefox:boolean} | null>(null);

  useEffect(() => {
    const unlistenPromise = listen<any>('import-progress', async (e) => {
      const { percent, eta_ms, phase } = e.payload as any;
      setProgress({ percent, eta_ms, phase });
      try {
        const win = getCurrentWindow();
        await win.setProgressBar({ status: ProgressBarStatus.Normal, progress: Math.min(100, Math.max(0, Math.round(percent))) });
        if (percent >= 100) {
          await win.setProgressBar({ status: ProgressBarStatus.None });
        }
      } catch {}
    });
    return () => { unlistenPromise.then((off)=>off()); };
  }, []);

  useEffect(() => {
    (async () => {
      try {
        const list = await invoke<string[]>('list_windows_profiles');
        setProfiles(list);
        if (list.length > 0) setSelectedUser(list[0]);
      } catch (e) {
        setMessage(`Konnte Benutzerprofile nicht laden: ${e}`);
      }
    })();
  }, []);

  const handleFileSelect = async () => {
    try {
      const result = await invoke('select_import_file');
      if (result) {
        setSelectedFile(result as string);
        setMessage('');
        // Falls Passwort schon eingegeben: Inhalte automatisch erkennen
        try {
          if (password) {
            const info = await invoke<{desktop:boolean;documents:boolean;pictures:boolean;downloads:boolean;chrome:boolean;edge:boolean;firefox:boolean}>(
              'detect_package_contents',
              { packagePath: result as string, password }
            );
            setDetected(info);
          }
        } catch (e) {
          console.warn('Detect failed:', e);
        }
      }
    } catch (error) {
      setMessage(`Fehler beim Dateiwählen: ${error}`);
    }
  };

  const handleImport = async () => {
    if (!selectedFile) {
      setMessage('Bitte wähle eine .nilo-Datei aus.');
      return;
    }
    if (!selectedUser) {
      setMessage('Bitte wähle ein Ziel-Benutzerprofil aus.');
      return;
    }
    if (!password) {
      setMessage('Bitte gib das Passwort ein.');
      return;
    }

    setIsLoading(true);
    setMessage('');
    setImportProgress(0);

    try {
      const result = await invoke('start_import_command', {
        packagePath: selectedFile,
        password: password,
        selectedUser: selectedUser,
      });
      setProgress({percent:100, eta_ms:0, phase:'Fertig'});
    let permissionGranted = await isPermissionGranted();

    if (!permissionGranted) {
      const permission = await requestPermission();
      permissionGranted = permission === 'granted';
    }

    if (permissionGranted) {
      sendNotification({
        title: "NiloShift",
        body: "Import erfolgreich abgeschlossen!",
        icon: "/icon.png",
        group: "NiloShift",
        iconColor: "#059669",
      });
    }

      setMessage('Import erfolgreich abgeschlossen!');
      console.log('Import result:', result);
    } catch (error) {
      sendNotification({
        title: "NiloShift",
        body: "Import fehlgeschlagen!",
        icon: "/icon.png",
        group: "NiloShift",
        iconColor: "#dc2626",
      });
      setMessage(`Fehler beim Import: ${error}`);
      setProgress({percent:0, eta_ms:0, phase:'Idle'});
    } finally {
      setIsLoading(false);
      try {
        const win = getCurrentWindow();
        await win.setProgressBar({ status: ProgressBarStatus.None });
      } catch {}
    }
  };

  return (
    <div className="min-h-screen linear-bg-primary">
      <div className="max-w-4xl mx-auto px-6 py-8">
        {/* Header */}
        <div className="mb-8">
          <button className="inline-flex items-center gap-2 linear-text-muted hover:linear-text-secondary transition-colors duration-150 mb-6 text-sm" onClick={() => navigate('/')}> 
            <svg width="16" height="16" viewBox="0 0 16 16" fill="none">
              <path d="M10 4L6 8L10 12" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"/>
            </svg>
            Zurück
          </button>
          <div className="flex items-center gap-4 mb-6">
            <div className="w-12 h-12 bg-[#059669] rounded-lg flex items-center justify-center text-white">
              <svg width="20" height="20" viewBox="0 0 24 24" fill="none">
                <path d="M12 22L12 9M12 22L8 18M12 22L16 18" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"/>
                <path d="M22 7V5C22 3.89543 21.1046 3 20 3H4C2.89543 3 2 3.89543 2 5V7" stroke="currentColor" strokeWidth="2" strokeLinecap="round"/>
              </svg>
            </div>
            <div>
              <h1 className="text-2xl font-semibold linear-text-primary">Profil Importieren</h1>
              <p className="linear-text-secondary text-sm">Stelle deine Daten auf diesem PC wieder her</p>
            </div>
          </div>
        </div>

        {/* User select and file */}
        <div className="linear-card p-4 mb-8 space-y-3">
          <div>
            <label className="block text-sm font-medium linear-text-primary mb-2">Ziel-Benutzerprofil</label>
            <div className="relative">
              <select className="linear-input w-full pr-10 appearance-none cursor-pointer hover:border-[#484f58] focus:border-[#5e6ad2]" value={selectedUser} onChange={(e) => setSelectedUser(e.target.value)}>
                {profiles.map((p) => (
                  <option key={p} value={p}>{p}</option>
                ))}
              </select>
              <svg className="pointer-events-none absolute right-3 top-1/2 -translate-y-1/2 text-[#7d8590]" width="16" height="16" viewBox="0 0 16 16" fill="none">
                <path d="M4 6L8 10L12 6" stroke="currentColor" strokeWidth="1.5" strokeLinecap="round" strokeLinejoin="round"/>
              </svg>
            </div>
            <p className="linear-text-muted text-xs mt-2">Ziel: C:\\Users\\{selectedUser}</p>
          </div>
          <div>
            <label className="block text-sm font-medium linear-text-primary mb-2">NiloShift-Paket</label>
            <div className="flex items-center gap-2">
              <input className="linear-input flex-1" placeholder="Pfad zur .nilo-Datei" value={selectedFile} onChange={(e)=>setSelectedFile(e.target.value)} />
              <button className="linear-button-secondary" onClick={handleFileSelect}>Datei wählen</button>
            </div>
          </div>
        </div>

        {/* Password Section */}
        <div className="flex flex-col gap-6">
          <h2 className="text-2xl font-semibold text-white">Entschlüsselung</h2>
          <div className="max-w-md">
            <label htmlFor="import-password" className="text-sm font-medium text-white mb-2">Passwort</label>
            <input id="import-password" type="password" value={password} onChange={async (e) => {
              const val = e.target.value;
              setPassword(val);
              // Wenn Datei vorhanden: Inhalte erkennen
              try {
                if (selectedFile && val) {
                  const info = await invoke<{desktop:boolean;documents:boolean;pictures:boolean;downloads:boolean;chrome:boolean;edge:boolean;firefox:boolean}>(
                    'detect_package_contents',
                    { packagePath: selectedFile, password: val }
                  );
                  setDetected(info);
                }
              } catch {}
            }} placeholder="Passwort zum Entschlüsseln eingeben" className="linear-input w-full" disabled={isLoading} />
          </div>
        </div>

        {/* Import Progress */}
        {(isLoading || progress.percent > 0) && (
          <div className="flex flex-col gap-2 mt-6 w-full">
            <div className="flex items-center justify-between text-xs linear-text-muted">
              <span>Phase: {progress.phase}</span>
              <span>ETA: {Math.max(0, Math.round(progress.eta_ms/1000))}s</span>
            </div>
            <div className="w-full bg-[#21262d] rounded-full h-2 overflow-hidden">
              <div className="h-full bg-[#059669] rounded-full transition-all duration-200" style={{ width: `${Math.min(100, Math.max(0, progress.percent))}%` }} />
            </div>
          </div>
        )}

        {/* Import Button */}
        <div className="flex flex-col items-start gap-4 pt-6 border-t linear-border mt-6">
          {detected && (
            <div className="text-xs linear-text-muted">
              <span className="linear-text-primary">Erkannt:</span>
              <span className="ml-2">{[
                detected.desktop && 'Desktop',
                detected.documents && 'Dokumente',
                detected.pictures && 'Bilder',
                detected.downloads && 'Downloads',
                detected.chrome && 'Chrome',
                detected.edge && 'Edge',
                detected.firefox && 'Firefox',
              ].filter(Boolean).join(', ') || 'Keine Inhalte erkannt'}</span>
            </div>
          )}
          {message && (<div className={`px-4 py-3 rounded-md text-sm max-w-md ${message.includes('Fehler') ? 'bg-red-500/10 text-red-400 border border-red-500/20' : 'bg-green-500/10 text-green-400 border border-green-500/20'}`}>{message}</div>)}
          <button className={`linear-button-primary flex items-center gap-2 ${isLoading || !selectedFile || !password ? 'opacity-50 cursor-not-allowed' : ''}`} onClick={handleImport} disabled={isLoading || !selectedFile || !password}>
            {isLoading ? (<><div className="w-4 h-4 border-2 border-white/30 border-t-white rounded-full animate-spin" />Import läuft...</>) : (<><svg width="16" height="16" viewBox="0 0 20 20" fill="none"><path d="M10 18L10 5M10 18L6 14M10 18L14 14" stroke="currentColor" strokeWidth="1.5" strokeLinecap="round" strokeLinejoin="round"/><path d="M18 6V4C18 2.89543 17.1046 2 16 2H4C2.89543 2 2 2.89543 2 4V6" stroke="currentColor" strokeWidth="1.5" strokeLinecap="round"/></svg>Import starten</>)}
          </button>
        </div>
      </div>
    </div>
  );
}

export default ImportPage;