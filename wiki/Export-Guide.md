# Export Guide 📤

Learn how to create secure backups of your Windows profile data with NiloShift's export feature.

## 🎯 What is Exporting?

Exporting creates an encrypted `.nilo` package containing your selected profile data. This package can be transferred to another computer and imported to restore your data.

## 🚀 Quick Export Process

1. **Launch NiloShift** and click "Export Profile" 🏗️
2. **Select your data** - Choose what to backup ✅
3. **Set security** - Enter a strong password 🔐
4. **Choose destination** - Where to save the `.nilo` file 📁
5. **Start export** - Wait for completion ⏱️

## 📋 Step-by-Step Export Guide

### Step 1: Launch Export Mode 🚀

1. Open NiloShift
2. Click the **"Export Profile"** button on the home screen
3. You'll see the export configuration page

### Step 2: Select User Profile 👤

1. **Choose source profile** from the dropdown
   - Shows all Windows user accounts on your computer
   - Select the profile you want to backup
   - Path shows as `C:\Users\[Username]`

2. **Verify profile access**
   - Green checkmark = accessible
   - Red warning = permission issues (run as admin)

### Step 3: Choose Data to Export 📦

NiloShift lets you selectively choose what to backup:

#### 🗂️ **User Folders**
- **📁 Desktop** - Shortcuts, files, and folders on your desktop
- **📄 Documents** - All files in your Documents folder
- **🖼️ Pictures** - Photos and images from Pictures folder
- **⬇️ Downloads** - Downloaded files and content

#### 🌐 **Browser Profiles**
- **🔵 Google Chrome** - Bookmarks, passwords, history, extensions
- **🔷 Microsoft Edge** - Bookmarks, passwords, history, settings
- **🦊 Mozilla Firefox** - Bookmarks, passwords, history, add-ons

#### 📧 **Microsoft Office**
- **📧 Outlook Signatures** - Email signatures and templates

### Step 4: Configure Security 🔒

1. **Set export password** 🔐
   - Choose a strong, memorable password
   - This encrypts your entire backup
   - **Important**: You'll need this password to import later!

2. **Confirm password** ✅
   - Re-enter to ensure accuracy
   - Passwords must match to proceed

#### 💡 **Password Tips**
- Use at least 12 characters
- Include uppercase, lowercase, numbers, symbols
- Avoid common passwords or personal info
- Consider using a passphrase like "Purple-Elephant-Dancing-2024!"

### Step 5: Choose Export Location 📁

1. **Set destination path**
   - Click "Choose Export Location" to browse
   - Or type path manually
   - Default: Desktop of selected user

2. **Recommended locations**
   - External drive for backup safety
   - Network location for easy transfer
   - Cloud-synced folder (Dropbox, OneDrive, etc.)

### Step 6: Start Export Process ⚡

1. **Review settings** 📋
   - Verify selected data types
   - Check export location
   - Ensure password is set

2. **Click "Start Export"** 🚀
   - Process begins immediately
   - Real-time progress display
   - ETA calculation shows time remaining

### Step 7: Monitor Progress 📊

During export, you'll see:

- **📊 Progress bar** - Visual completion percentage
- **⏱️ Time remaining** - Estimated time to completion
- **📝 Current phase** - What's happening now:
  - "Preparing" - Analyzing selected data
  - "Collecting" - Copying files to temporary location
  - "Compressing" - Creating ZIP archive
  - "Encrypting" - Applying AES-256 encryption
  - "Finalizing" - Cleaning up and completing

## 🎯 Export Options Explained

### User Folders 📁

**Desktop**
- All files and folders on desktop
- Desktop shortcuts and links
- Custom desktop icons

**Documents**
- Personal documents and files
- Nested folder structures preserved
- File permissions maintained

**Pictures**
- Photos from camera imports
- Screenshots and saved images
- Organized folder structures

**Downloads**
- Browser downloaded files
- Torrent downloads
- Software installers

### Browser Data 🌐

**Google Chrome**
- Bookmarks and bookmark folders
- Saved passwords (encrypted)
- Browsing history
- Extensions and their data
- Settings and preferences
- Auto-fill data

**Microsoft Edge**
- Favorites and collections
- Saved passwords
- Browsing history
- Extensions
- Settings and sync data

**Mozilla Firefox**
- Bookmarks and folders
- Saved logins
- Browsing history
- Add-ons and themes
- Settings and preferences

### Microsoft Office 📧

**Outlook Signatures**
- Email signature templates
- HTML and plain text versions
- Embedded images and formatting
- Default signature settings

## ⚡ Performance Tips

### For Faster Exports

1. **Close running applications** 📴
   - Exit browsers and email clients
   - Close file-heavy applications
   - Free up system resources

2. **Use SSD for temporary files** 💾
   - NiloShift uses system temp folder
   - SSD dramatically improves speed
   - Ensure sufficient free space

3. **Select only needed data** 🎯
   - Don't export unnecessary files
   - Pictures and downloads can be large
   - Browser data is usually quick

### For Large Exports

1. **Monitor disk space** 💾
   - Export can be 2-3x original size during process
   - Final `.nilo` file is compressed
   - Ensure adequate free space

2. **Stable power supply** 🔌
   - Use UPS for desktop computers
   - Ensure laptop is plugged in
   - Don't let system sleep during export

## 🛡️ Security Considerations

### Encryption Details 🔒

- **AES-256-GCM encryption** - Military-grade security
- **Unique salt per export** - Even same data creates different files
- **Password-derived key** - Your password generates the encryption key
- **Authenticated encryption** - Detects any tampering

### Safe Storage 📦

1. **Multiple copies** 
   - Keep backup on external drive
   - Store copy in different location
   - Consider cloud storage with strong password

2. **Password security**
   - Never share export password
   - Use password manager if needed
   - Consider secure password hint

## ⚠️ Common Export Issues

### Large File Warnings

**Very large exports (>5GB)**
- Consider selective export
- May take several hours
- Ensure stable power supply

**Browser data too large**
- Clear browser cache first
- Remove unnecessary history
- Consider exporting profiles separately

### Permission Issues

**"Access Denied" errors**
- Run NiloShift as Administrator
- Check user account permissions
- Ensure source profile is accessible

### Storage Problems

**Insufficient disk space**
- Free up temporary space
- Choose different export location
- Remove unnecessary files first

## ✅ Export Completion

When export finishes successfully:

1. **Notification appears** 🔔
   - "Export completed successfully!"
   - Shows final file location

2. **Verify the file** ✅
   - Check `.nilo` file exists
   - Note file size (should be substantial)
   - Keep file location secure

3. **Test password** 🔐
   - Try importing on same machine first
   - Ensures password works correctly
   - Verify data integrity

## 📋 Next Steps

After successful export:

- **🚚 Transfer file** to target computer
- **🔒 Store backup** safely
- **📖 Read [Import Guide](Import-Guide)** for restoration
- **🧪 Test import** on non-critical system first

## 🆘 Troubleshooting

For export problems:
- Check [Troubleshooting Guide](Troubleshooting)
- Review [Known Issues](Known-Issues)
- Contact support at hello@nilovon.com

---

**Export successful? Ready to import!** 🎉

*Next: [Import Guide](Import-Guide)*
