import { useEffect, useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { getCurrentWindow, ProgressBarStatus } from '@tauri-apps/api/window';
import { isPermissionGranted, requestPermission, sendNotification } from '@tauri-apps/plugin-notification';

interface ExportOptions {
  desktop: boolean;
  documents: boolean;
  pictures: boolean;
  videos: boolean;
  music: boolean;
  downloads: boolean;
  chrome: boolean;
  edge: boolean;
  firefox: boolean;
  outlook_signatures: boolean;
}

function ExportPage() {
  const navigate = useNavigate();
  const [exportOptions, setExportOptions] = useState<ExportOptions>({
    desktop: true,
    documents: true,
    pictures: false,
    videos: false,
    music: false,
    downloads: false,
    chrome: true,
    edge: false,
    firefox: false,
    outlook_signatures: false,
  });
  const [password, setPassword] = useState('');
  const [confirmPassword, setConfirmPassword] = useState('');
  const [isLoading, setIsLoading] = useState(false);
  const [message, setMessage] = useState('');
  const [exportPath, setExportPath] = useState('');
  const [profiles, setProfiles] = useState<string[]>([]);
  const [selectedUser, setSelectedUser] = useState('');
  const [progress, setProgress] = useState<{percent:number, eta_ms:number, phase:string}>({percent:0, eta_ms:0, phase:'Idle'});
  const [browserPresence, setBrowserPresence] = useState<{chrome:boolean; edge:boolean; firefox:boolean}>({chrome:true, edge:true, firefox:true});

  useEffect(() => {
    const unlistenPromise = listen<any>('export-progress', async (e) => {
      const { percent, eta_ms, phase } = e.payload as any;
      setProgress({ percent, eta_ms, phase });
      // Windows taskbar progress (also works on macOS Dock)
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
        if (list.length > 0) {
          setSelectedUser(list[0]);
          try {
            const presence = await invoke<{chrome:boolean;edge:boolean;firefox:boolean}>('detect_browsers', { selectedUser: list[0] });
            setBrowserPresence(presence);
            setExportOptions(prev => ({
              ...prev,
              chrome: presence.chrome,
              edge: presence.edge && prev.edge,
              firefox: presence.firefox && prev.firefox,
            }));
          } catch {}
        }
      } catch (e) {
        setMessage(`Konnte Benutzerprofile nicht laden: ${e}`);
      }
    })();
  }, []);

  useEffect(() => {
    if (!selectedUser) return;
    (async () => {
      try {
        const presence = await invoke<{chrome:boolean;edge:boolean;firefox:boolean}>('detect_browsers', { selectedUser });
        setBrowserPresence(presence);
        setExportOptions(prev => ({
          ...prev,
          chrome: presence.chrome ? prev.chrome : false,
          edge: presence.edge ? prev.edge : false,
          firefox: presence.firefox ? prev.firefox : false,
        }));
      } catch {}
    })();
  }, [selectedUser]);

  const openSaveDialog = async () => {
    try {
      const path = await invoke<string>('select_export_path', { defaultFileName: 'NiloShift_Export.nilo' });
      setExportPath(path);
    } catch (e) {
      setMessage(`Kein Speicherort gewählt.`);
    }
  };

  const handleOptionChange = (option: keyof ExportOptions) => {
    setExportOptions(prev => ({
      ...prev,
      [option]: !prev[option]
    }));
  };

  const handleExport = async () => {
    if (!selectedUser) {
      setMessage('Bitte wähle ein Benutzerprofil aus.');
      return;
    }

    if (!password) {
      setMessage('Bitte gib ein Passwort ein.');
      return;
    }
    
    if (password !== confirmPassword) {
      setMessage('Passwörter stimmen nicht überein.');
      return;
    }

    const selectedOptions = Object.entries(exportOptions)
      .filter(([_, selected]) => selected)
      .map(([option, _]) => option);

    if (selectedOptions.length === 0) {
      setMessage('Bitte wähle mindestens eine Option aus.');
      return;
    }

    setIsLoading(true);
    setMessage('');

    let permissionGranted = await isPermissionGranted();

    if (!permissionGranted) {
      const permission = await requestPermission();
      permissionGranted = permission === 'granted';
    }

    if (permissionGranted) {
      sendNotification({
        title: "NiloShift",
        body: "Export erfolgreich gestartet!",
        icon: "/icon.png",
        group: "NiloShift",
        iconColor: "#059669",
      });
    }

    try {
      const result = await invoke('start_export_command', {
        options: exportOptions,
        password: password,
        exportPath: exportPath || undefined,
        selectedUser: selectedUser,
      });
      setMessage('Export erfolgreich gestartet!');
      console.log('Export result:', result);
    } catch (error) {
      setMessage(`Fehler beim Export: ${error}`);
      sendNotification({
        title: "NiloShift",
        body: "Export fehlgeschlagen!",
        icon: "/icon.png",
        group: "NiloShift",
        iconColor: "#dc2626",
      });
    } finally {
      setIsLoading(false);
      try {
        const win = getCurrentWindow();
        await win.setProgressBar({ status: ProgressBarStatus.None });
      } catch {}
    }
  };

  const OptionCard = ({ 
    option, 
    icon, 
    title, 
    description 
  }: { 
    option: keyof ExportOptions; 
    icon: string; 
    title: string; 
    description: string; 
  }) => (
    <label className={`linear-card p-4 cursor-pointer transition-all duration-150 ${
      exportOptions[option] ? 'border-[#5e6ad2] bg-[#5e6ad2]/5' : 'hover:border-[#484f58]'
    }`}>
      <input
        type="checkbox"
        checked={exportOptions[option]}
        onChange={() => handleOptionChange(option)}
        className="sr-only"
      />
      <div className="flex items-center gap-3">
        <div className="text-lg">{icon}</div>
        <div className="flex-1 min-w-0">
          <div className="text-sm font-medium linear-text-primary mb-1">{title}</div>
          <div className="linear-text-muted text-xs">{description}</div>
        </div>
        <div className={`w-4 h-4 border rounded flex items-center justify-center transition-all duration-150 ${
          exportOptions[option] 
            ? 'bg-[#5e6ad2] border-[#5e6ad2] text-white' 
            : 'border-[#484f58]'
        }`}>
          {exportOptions[option] && (
            <svg width="10" height="10" viewBox="0 0 16 16" fill="none">
              <path d="M13 4L6 11L3 8" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"/>
            </svg>
          )}
        </div>
      </div>
    </label>
  );

  return (
    <div className="min-h-screen linear-bg-primary">
      <div className="max-w-4xl mx-auto px-6 py-8">
        {/* Header */}
        <div className="mb-8">
          <button 
            className="inline-flex items-center gap-2 linear-text-muted hover:linear-text-secondary transition-colors duration-150 mb-6 text-sm"
            onClick={() => navigate('/')}
          >
            <svg width="14" height="14" viewBox="0 0 16 16" fill="none">
              <path d="M10 4L6 8L10 12" stroke="currentColor" strokeWidth="1.5" strokeLinecap="round" strokeLinejoin="round"/>
            </svg>
            Zurück
          </button>
          
          <div className="flex items-center gap-4 mb-6">
            <div className="w-12 h-12 bg-[#dc2626] rounded-lg flex items-center justify-center text-white">
              <svg width="20" height="20" viewBox="0 0 24 24" fill="none">
                <path d="M12 2L12 15M12 2L8 6M12 2L16 6" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"/>
                <path d="M22 17V19C22 20.1046 21.1046 21 20 21H4C2.89543 21 2 20.1046 2 19V17" stroke="currentColor" strokeWidth="2" strokeLinecap="round"/>
              </svg>
            </div>
            <div>
              <h1 className="text-2xl font-semibold linear-text-primary">Profil Exportieren</h1>
              <p className="linear-text-secondary text-sm">Wähle die Daten aus, die du exportieren möchtest</p>
            </div>
          </div>
        </div>

        {/* User select */}
        <div className="linear-card p-4 mb-8 space-y-3">
          <div>
            <label className="block text-sm font-medium linear-text-primary mb-2">Benutzerprofil</label>
            <div className="relative">
              <select
                className="linear-input w-full pr-10 appearance-none cursor-pointer hover:border-[#484f58] focus:border-[#5e6ad2]"
                value={selectedUser}
                onChange={(e) => setSelectedUser(e.target.value)}
              >
                {profiles.map((p) => (
                  <option key={p} value={p}>{p}</option>
                ))}
              </select>
              <svg className="pointer-events-none absolute right-3 top-1/2 -translate-y-1/2 text-[#7d8590]" width="16" height="16" viewBox="0 0 16 16" fill="none">
                <path d="M4 6L8 10L12 6" stroke="currentColor" strokeWidth="1.5" strokeLinecap="round" strokeLinejoin="round"/>
              </svg>
            </div>
            <p className="linear-text-muted text-xs mt-2">Quelle: C:\\Users\\{selectedUser}</p>
          </div>
          <div>
            <label className="block text-sm font-medium linear-text-primary mb-2">Ziel-Datei</label>
            <div className="flex items-center gap-2">
              <input className="linear-input flex-1" placeholder="z.B. D:\\Backups\\NiloShift_Export.nilo" value={exportPath} onChange={(e)=>setExportPath(e.target.value)} />
              <button className="linear-button-secondary" onClick={openSaveDialog}>Speicherort wählen</button>
            </div>
            <p className="linear-text-muted text-xs mt-2">Wenn leer, wird auf dem Desktop des ausgewählten Benutzers gespeichert.</p>
          </div>
        </div>

        {/* Export Options */}
        <div className="space-y-8">
          <div>
            <h2 className="text-lg font-medium linear-text-primary mb-4">Benutzerordner</h2>
            <div className="grid grid-cols-1 md:grid-cols-2 gap-3">
              <OptionCard
                option="desktop"
                icon="📁"
                title="Desktop"
                description="Desktop-Dateien und Verknüpfungen"
              />
              <OptionCard
                option="documents"
                icon="📄"
                title="Dokumente"
                description="Alle Dateien im Dokumentenordner"
              />
              <OptionCard
                option="pictures"
                icon="🖼️"
                title="Bilder"
                description="Fotos und Bilder"
              />
              <OptionCard
                option="downloads"
                icon="⬇️"
                title="Downloads"
                description="Heruntergeladene Dateien"
              />
            </div>
          </div>

          <div>
            <h2 className="text-lg font-medium linear-text-primary mb-4">Browser-Profile</h2>
            <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-3">
              {browserPresence.chrome && (
                <OptionCard
                  option="chrome"
                  icon="🌐"
                  title="Google Chrome"
                  description="Lesezeichen, Passwörter, Verlauf"
                />
              )}
              {browserPresence.edge && (
                <OptionCard
                  option="edge"
                  icon="🔷"
                  title="Microsoft Edge"
                  description="Lesezeichen, Passwörter, Verlauf"
                />
              )}
              {browserPresence.firefox && (
                <OptionCard
                  option="firefox"
                  icon="🦊"
                  title="Mozilla Firefox"
                  description="Lesezeichen, Passwörter, Verlauf"
                />
              )}
            </div>
          </div>

          <div>
            <h2 className="text-lg font-medium linear-text-primary mb-4">Microsoft Office</h2>
            <div className="grid grid-cols-1 md:grid-cols-2 gap-3">
              <OptionCard
                option="outlook_signatures"
                icon="📧"
                title="Outlook-Signaturen"
                description="E-Mail-Signaturen und Vorlagen"
              />
            </div>
          </div>

          {/* Security Section */}
          <div>
            <h2 className="text-lg font-medium linear-text-primary mb-4">Sicherheit</h2>
            <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div>
                <label htmlFor="password" className="block text-sm font-medium linear-text-primary mb-2">
                  Passwort
                </label>
                <input
                  id="password"
                  type="password"
                  value={password}
                  onChange={(e) => setPassword(e.target.value)}
                  placeholder="Sicheres Passwort eingeben"
                  className="linear-input w-full"
                />
              </div>
              <div>
                <label htmlFor="confirmPassword" className="block text-sm font-medium linear-text-primary mb-2">
                  Passwort bestätigen
                </label>
                <input
                  id="confirmPassword"
                  type="password"
                  value={confirmPassword}
                  onChange={(e) => setConfirmPassword(e.target.value)}
                  placeholder="Passwort wiederholen"
                  className="linear-input w-full"
                />
              </div>
            </div>
          </div>

          {/* Export Button */}
          <div className="flex flex-col items-start gap-4 pt-6 border-t linear-border">
            {/* Progress */}
            {isLoading && (
              <div className="w-full space-y-2">
                <div className="flex items-center justify-between text-xs linear-text-muted">
                  <span>Phase: {progress.phase}</span>
                  <span>ETA: {Math.max(0, Math.round(progress.eta_ms/1000))}s</span>
                </div>
                <div className="w-full bg-[#21262d] rounded-full h-2 overflow-hidden">
                  <div className="h-full bg-[#5e6ad2] rounded-full transition-all duration-200" style={{ width: `${Math.min(100, Math.max(0, progress.percent))}%` }} />
                </div>
              </div>
            )}
            {message && (
              <div className={`px-4 py-3 rounded-md text-sm max-w-md ${
                message.includes('Fehler') 
                  ? 'bg-red-500/10 text-red-400 border border-red-500/20' 
                  : 'bg-green-500/10 text-green-400 border border-green-500/20'
              }`}>
                {message}
              </div>
            )}
            
            <button
              className={`linear-button-primary flex items-center gap-2 ${
                isLoading ? 'opacity-50 cursor-not-allowed' : ''
              }`}
              onClick={handleExport}
              disabled={isLoading}
            >
              {isLoading ? (
                <>
                  <div className="w-4 h-4 border-2 border-white/30 border-t-white rounded-full animate-spin" />
                  Export läuft...
                </>
              ) : (
                <>
                  <svg width="16" height="16" viewBox="0 0 20 20" fill="none">
                    <path d="M10 2L10 13M10 2L6 6M10 2L14 6" stroke="currentColor" strokeWidth="1.5" strokeLinecap="round" strokeLinejoin="round"/>
                    <path d="M18 14V16C18 17.1046 17.1046 18 16 18H4C2.89543 18 2 17.1046 2 16V14" stroke="currentColor" strokeWidth="1.5" strokeLinecap="round"/>
                  </svg>
                  Export starten
                </>
              )}
            </button>
          </div>
        </div>
      </div>
    </div>
  );
}

export default ExportPage;