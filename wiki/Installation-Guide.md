# Installation Guide 📥

This guide will walk you through installing NiloShift on your Windows computer.

## 📋 System Requirements

Before installing, make sure your system meets these requirements:

- **💻 Operating System**: Windows 10 (1903) or later
- **🧠 Memory**: 4 GB RAM minimum, 8 GB recommended
- **💾 Storage**: 500 MB free space (plus space for your data)
- **👑 Permissions**: Administrator rights recommended for full functionality

## 📦 Download Options

NiloShift offers two download options:

### 🎯 **Recommended: MSI Installer**
- **File**: `NiloShift-1.0.0.msi`
- **Best for**: Most users who want easy installation
- **Features**: 
  - Automatic installation
  - Start menu shortcuts
  - Uninstaller included
  - Windows integration

### ⚡ **Alternative: Portable Executable**
- **File**: `niloshift.exe`
- **Best for**: Advanced users or portable use
- **Features**:
  - No installation required
  - Run from any location
  - Perfect for USB drives
  - No registry changes

## 🚀 Installation Steps

### Method 1: MSI Installer (Recommended)

1. **Download the MSI file** 📥
   - Go to [GitHub Releases](https://github.com/Nilovon/NiloShift/releases/latest)
   - Download `NiloShift-1.0.0.msi`

2. **Run the installer** 🔧
   - Double-click the downloaded MSI file
   - Click "Yes" when Windows asks for permission

3. **Follow the installation wizard** 📋
   - Choose installation directory (default: `C:\Program Files\NiloShift`)
   - Select additional options if desired
   - Click "Install"

4. **Complete installation** ✅
   - Wait for installation to finish
   - Click "Finish"
   - NiloShift is now installed!

### Method 2: Portable Executable

1. **Download the EXE file** 📥
   - Go to [GitHub Releases](https://github.com/Nilovon/NiloShift/releases/latest)
   - Download `niloshift.exe`

2. **Choose a location** 📁
   - Create a folder like `C:\Tools\NiloShift`
   - Or place it on a USB drive for portability

3. **Run NiloShift** ⚡
   - Double-click `niloshift.exe`
   - Right-click and "Run as administrator" for full features

## 🛡️ Security Considerations

### Windows Defender / Antivirus Warnings

Some antivirus software may flag NiloShift as suspicious. This is a **false positive** because:

- NiloShift accesses user profile data (which looks suspicious to antivirus)
- The executable is not widely recognized yet
- File encryption/decryption triggers security alerts

### To resolve antivirus issues:

1. **Add exception** 🛡️
   - Add NiloShift folder to antivirus exclusions
   - Whitelist `niloshift.exe` specifically

2. **Download from official source** ✅
   - Only download from [GitHub Releases](https://github.com/Nilovon/NiloShift/releases)
   - Verify file checksums if provided

3. **Run as Administrator** 👑
   - Right-click NiloShift and "Run as administrator"
   - This provides full access to user profile data

## 🔧 Post-Installation Setup

### First Launch

1. **Run NiloShift** 🚀
   - From Start Menu (MSI install) or double-click executable
   - Accept any Windows firewall prompts

2. **Check permissions** 👑
   - NiloShift will check for necessary permissions
   - Grant administrator access when prompted

3. **Verify functionality** ✅
   - Test by browsing to the Export page
   - Verify your user profiles are detected

### Optional: Create Desktop Shortcut

For portable installation:
1. Right-click `niloshift.exe`
2. Select "Create shortcut"
3. Move shortcut to Desktop
4. Rename to "NiloShift"

## 🔄 Updating NiloShift

### Automatic Updates
Currently, NiloShift doesn't have automatic updates. Check our [releases page](https://github.com/Nilovon/NiloShift/releases) periodically.

### Manual Update Process
1. Download the latest version
2. For MSI: Run new installer (will update existing installation)
3. For portable: Replace the old executable with the new one

## 🗑️ Uninstalling NiloShift

### MSI Installation
1. Go to **Settings > Apps**
2. Find "NiloShift" in the list
3. Click "Uninstall"
4. Follow the uninstaller prompts

### Portable Installation
1. Simply delete the `niloshift.exe` file
2. Remove any shortcuts you created
3. No registry cleanup needed

## ⚠️ Troubleshooting Installation

### Common Issues

**"This app can't run on your PC"**
- Ensure you're running Windows 10 (1903) or later
- Download the correct architecture (x64)

**"Windows protected your PC"**
- Click "More info"
- Click "Run anyway"
- This happens because the app isn't digitally signed yet

**Installation fails**
- Run installer as Administrator
- Temporarily disable antivirus
- Ensure sufficient disk space

**MSI installer error**
- Try the portable version instead
- Run Windows Update and try again
- Check Windows Installer service is running

### Getting Help

If you encounter issues:
1. Check our [Troubleshooting Guide](Troubleshooting)
2. [Search existing issues](https://github.com/Nilovon/NiloShift/issues)
3. [Create a new issue](https://github.com/Nilovon/NiloShift/issues/new)
4. Email us at hello@nilovon.com

## ✅ Next Steps

Once installed, you're ready to:
- [Quick Start Tutorial](Quick-Start-Tutorial) - Learn the basics
- [Export Guide](Export-Guide) - Create your first backup
- [Import Guide](Import-Guide) - Restore data on a new machine

---

**Installation complete? Let's get started!** 🚀

*Next: [Quick Start Tutorial](Quick-Start-Tutorial)*
