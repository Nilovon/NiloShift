import { useNavigate } from 'react-router-dom';

function HomePage() {
  const navigate = useNavigate();

  return (
    <div className="min-h-screen linear-bg-primary">
      <div className="max-w-5xl mx-auto px-6 py-8">
        {/* Header */}
        <div className="mb-12">
          <div className="flex items-center gap-3 mb-8">
            <div className="w-10 h-10 rounded-md overflow-hidden">
              <img src="/icon.png" alt="NiloShift" className="w-10 h-10" />
            </div>
            <div>
              <h1 className="text-2xl font-semibold linear-text-primary">NiloShift</h1>
              <p className="text-sm linear-text-muted">Einfache PC-Migration für IT-Profis</p>
            </div>
          </div>
          
          <div className="mb-8">
            <h2 className="text-3xl font-semibold linear-text-primary mb-3">
              Willkommen bei NiloShift
            </h2>
            <p className="text-lg linear-text-secondary leading-relaxed max-w-2xl">
              Migriere Benutzerdaten schnell und sicher von einem alten auf einen neuen PC. 
              Wähle eine Option unten, um zu beginnen.
            </p>
          </div>
        </div>

        {/* Action Cards */}
        <div className="grid grid-cols-1 lg:grid-cols-2 gap-4 mb-12">
          <div 
            className="linear-card p-6 cursor-pointer group"
            onClick={() => navigate('/export')}
          >
            <div className="flex items-start gap-4">
              <div className="w-12 h-12 bg-[#dc2626] rounded-lg flex items-center justify-center text-white flex-shrink-0">
                <svg width="20" height="20" viewBox="0 0 24 24" fill="none">
                  <path d="M12 2L12 15M12 2L8 6M12 2L16 6" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"/>
                  <path d="M22 17V19C22 20.1046 21.1046 21 20 21H4C2.89543 21 2 20.1046 2 19V17" stroke="currentColor" strokeWidth="2" strokeLinecap="round"/>
                </svg>
              </div>
              <div className="flex-1 min-w-0">
                <h3 className="text-lg font-medium linear-text-primary mb-2">Profil Exportieren</h3>
                <p className="linear-text-secondary text-sm leading-relaxed">
                  Sammle und verschlüssele Daten vom alten PC
                </p>
              </div>
              <div className="linear-text-muted group-hover:linear-text-secondary transition-colors duration-150">
                <svg width="16" height="16" viewBox="0 0 16 16" fill="none">
                  <path d="M6 4L10 8L6 12" stroke="currentColor" strokeWidth="1.5" strokeLinecap="round" strokeLinejoin="round"/>
                </svg>
              </div>
            </div>
          </div>

          <div 
            className="linear-card p-6 cursor-pointer group"
            onClick={() => navigate('/import')}
          >
            <div className="flex items-start gap-4">
              <div className="w-12 h-12 bg-[#059669] rounded-lg flex items-center justify-center text-white flex-shrink-0">
                <svg width="20" height="20" viewBox="0 0 24 24" fill="none">
                  <path d="M12 22L12 9M12 22L8 18M12 22L16 18" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"/>
                  <path d="M22 7V5C22 3.89543 21.1046 3 20 3H4C2.89543 3 2 3.89543 2 5V7" stroke="currentColor" strokeWidth="2" strokeLinecap="round"/>
                </svg>
              </div>
              <div className="flex-1 min-w-0">
                <h3 className="text-lg font-medium linear-text-primary mb-2">Profil Importieren</h3>
                <p className="linear-text-secondary text-sm leading-relaxed">
                  Stelle Daten auf dem neuen PC wieder her
                </p>
              </div>
              <div className="linear-text-muted group-hover:linear-text-secondary transition-colors duration-150">
                <svg width="16" height="16" viewBox="0 0 16 16" fill="none">
                  <path d="M6 4L10 8L6 12" stroke="currentColor" strokeWidth="1.5" strokeLinecap="round" strokeLinejoin="round"/>
                </svg>
              </div>
            </div>
          </div>
        </div>

        {/* Features Grid */}
        <div className="mb-12">
          <h3 className="text-xl font-medium linear-text-primary mb-6">Was kann NiloShift?</h3>
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
            <div className="linear-card p-5">
              <div className="w-10 h-10 bg-[#161b22] border linear-border rounded-lg flex items-center justify-center mb-4">
                <svg width="18" height="18" viewBox="0 0 20 20" fill="none" className="text-[#5e6ad2]">
                  <path d="M5 9V7C5 5.89543 5.89543 5 7 5H13C14.1046 5 15 5.89543 15 7V9M5 9C3.89543 9 3 9.89543 3 11V15C3 16.1046 3.89543 17 5 17H15C16.1046 17 17 16.1046 17 15V11C17 9.89543 16.1046 9 15 9M5 9H15" stroke="currentColor" strokeWidth="1.5"/>
                </svg>
              </div>
              <h4 className="text-sm font-medium linear-text-primary mb-2">Sicher & Verschlüsselt</h4>
              <p className="linear-text-muted text-xs leading-relaxed">AES-256 Verschlüsselung für maximale Sicherheit</p>
            </div>

            <div className="linear-card p-5">
              <div className="w-10 h-10 bg-[#161b22] border linear-border rounded-lg flex items-center justify-center mb-4">
                <svg width="18" height="18" viewBox="0 0 20 20" fill="none" className="text-[#5e6ad2]">
                  <path d="M13 7L7 13M7 7L13 13" stroke="currentColor" strokeWidth="1.5" strokeLinecap="round"/>
                  <circle cx="10" cy="10" r="8" stroke="currentColor" strokeWidth="1.5"/>
                </svg>
              </div>
              <h4 className="text-sm font-medium linear-text-primary mb-2">Schnell & Einfach</h4>
              <p className="linear-text-muted text-xs leading-relaxed">Intuitive Benutzeroberfläche für effiziente Migrationen</p>
            </div>

            <div className="linear-card p-5">
              <div className="w-10 h-10 bg-[#161b22] border linear-border rounded-lg flex items-center justify-center mb-4">
                <svg width="18" height="18" viewBox="0 0 20 20" fill="none" className="text-[#5e6ad2]">
                  <path d="M3 7V17C3 17.5523 3.44772 18 4 18H16C16.5523 18 17 17.5523 17 17V7M3 7L10 2L17 7M3 7H17" stroke="currentColor" strokeWidth="1.5"/>
                </svg>
              </div>
              <h4 className="text-sm font-medium linear-text-primary mb-2">Vollständige Profile</h4>
              <p className="linear-text-muted text-xs leading-relaxed">Desktop, Dokumente, Browser-Profile und mehr</p>
            </div>

            <div className="linear-card p-5">
              <div className="w-10 h-10 bg-[#161b22] border linear-border rounded-lg flex items-center justify-center mb-4">
                <svg width="18" height="18" viewBox="0 0 20 20" fill="none" className="text-[#5e6ad2]">
                  <path d="M10 2V6M10 14V18M4.22 4.22L6.34 6.34M13.66 13.66L15.78 15.78M2 10H6M14 10H18M4.22 15.78L6.34 13.66M13.66 6.34L15.78 4.22" stroke="currentColor" strokeWidth="1.5" strokeLinecap="round"/>
                </svg>
              </div>
              <h4 className="text-sm font-medium linear-text-primary mb-2">IT-Profi Tools</h4>
              <p className="linear-text-muted text-xs leading-relaxed">Entwickelt für Systemadministratoren</p>
            </div>
          </div>
        </div>

        {/* Footer */}
        <div className="pt-8 border-t linear-border">
          <p className="linear-text-muted text-xs">NiloShift v0.1.0 - Entwickelt für Windows-Migrationen</p>
        </div>
      </div>
    </div>
  );
}

export default HomePage;