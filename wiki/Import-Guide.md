# Import Guide 📥

Learn how to restore your Windows profile data from a NiloShift backup package on your new or target computer.

## 🎯 What is Importing?

Importing takes an encrypted `.nilo` package (created by the export process) and restores the selected data to a Windows user profile on your target computer.

## 🚀 Quick Import Process

1. **Launch NiloShift** and click "Import Profile" 📥
2. **Select .nilo file** - Choose your backup package 📁
3. **Enter password** - Decrypt the package 🔐
4. **Choose what to restore** - Select specific content ✅
5. **Pick target user** - Where to restore data 👤
6. **Start import** - Wait for completion ⏱️

## 📋 Step-by-Step Import Guide

### Step 1: Launch Import Mode 🚀

1. Open NiloShift on your target computer
2. Click the **"Import Profile"** button on the home screen
3. You'll see the import configuration page

### Step 2: Select Target User Profile 👤

1. **Choose destination profile** from the dropdown
   - Shows all Windows user accounts on target computer
   - Select where you want to restore data
   - Path shows as `C:\Users\[Username]`

2. **Verify write access**
   - Green checkmark = accessible
   - Red warning = permission issues (run as admin)

### Step 3: Choose NiloShift Package 📁

1. **Select .nilo file**
   - Click "Choose File" button
   - Browse to your backup location
   - Select the `.nilo` file created during export

2. **File verification**
   - NiloShift checks file integrity
   - Displays basic package information
   - Shows when package was created

### Step 4: Enter Decryption Password 🔐

1. **Enter export password**
   - Type the password you used during export
   - Password is case-sensitive
   - Must match exactly

2. **Load package contents** 🔍
   - Click "Load Contents" to analyze package
   - Or "Skip Detection" if having issues
   - NiloShift scans for available data types

### Step 5: Select Content to Restore ✅

After successful decryption, choose what to import:

#### 🗂️ **Available User Folders**
- **📁 Desktop** - Desktop files and shortcuts
- **📄 Documents** - Document files and folders
- **🖼️ Pictures** - Photos and images
- **⬇️ Downloads** - Downloaded files

#### 🌐 **Available Browser Data**
- **🔵 Google Chrome** - Bookmarks, passwords, history
- **🔷 Microsoft Edge** - Favorites, passwords, history
- **🦊 Mozilla Firefox** - Bookmarks, passwords, history

#### 📧 **Available Office Data**
- **📧 Outlook Signatures** - Email signatures and templates

### Step 6: Configure Import Options ⚙️

1. **Select specific items** ✅
   - Check boxes for desired content
   - Uncheck items you don't want
   - "Select All" / "Select None" for convenience

2. **Review detected content** 📋
   - Green checkmarks show available content
   - Grayed items not found in package
   - Summary shows what will be imported

### Step 7: Start Import Process ⚡

1. **Review import settings** 📋
   - Verify target user profile
   - Check selected content types
   - Ensure sufficient disk space

2. **Click "Start Import"** 🚀
   - Process begins immediately
   - Real-time progress display
   - ETA calculation shows time remaining

### Step 8: Monitor Progress 📊

During import, you'll see:

- **📊 Progress bar** - Visual completion percentage
- **⏱️ Time remaining** - Estimated time to completion
- **📝 Current phase** - What's happening now:
  - "Decrypting" - Unlocking package data
  - "Extracting" - Unpacking compressed files
  - "Restoring" - Copying files to target locations
  - "Finalizing" - Setting permissions and cleanup

## 🎯 Content Restoration Details

### User Folders 📁

**Desktop Restoration**
- Files placed in target user's Desktop
- Shortcuts maintain their targets (if accessible)
- Folder structure preserved
- Existing files with same names backed up

**Documents Restoration**
- Files copied to Documents folder
- Nested folder structures preserved
- File permissions set appropriately
- Duplicate handling available

**Pictures Restoration**
- Images copied to Pictures folder
- Organized folder structure maintained
- EXIF data and metadata preserved
- Integration with Windows Photos app

**Downloads Restoration**
- Files placed in Downloads folder
- Original download metadata preserved where possible
- Folder organization maintained

### Browser Data Restoration 🌐

**Google Chrome**
- Bookmarks merged with existing
- Passwords imported to Chrome password manager
- History integrated with current browsing
- Extensions marked for reinstallation
- Settings applied to active profile

**Microsoft Edge**
- Favorites and collections merged
- Password vault updated
- Browsing history integrated
- Extensions and settings restored
- Sync settings applied

**Mozilla Firefox**
- Bookmarks merged with current profile
- Saved logins imported securely
- History integrated
- Add-ons marked for reinstallation
- Preferences and settings applied

### Microsoft Office Data 📧

**Outlook Signatures**
- Signature files copied to Outlook folder
- HTML and text versions restored
- Images and formatting preserved
- Available in Outlook signature selector
- Default signature settings applied

## ⚙️ Import Behavior Options

### Conflict Resolution 🔄

When importing data that already exists:

1. **File conflicts**
   - Existing files renamed with timestamp
   - New files imported alongside
   - No data loss occurs

2. **Browser conflicts**
   - Bookmarks merged (duplicates possible)
   - Passwords updated if newer
   - History entries added

3. **Settings conflicts**
   - Imported settings take precedence
   - Original settings backed up when possible
   - Manual review may be needed

### Selective Import ✅

**Granular control**
- Choose specific data types
- Mix and match as needed
- Skip unwanted content
- Import in multiple sessions

**Smart detection**
- Only shows available content
- Grays out missing data types
- Estimates import time and space
- Warns of potential conflicts

## 🛡️ Security During Import

### Decryption Process 🔒

- **Secure password handling** - Password never stored
- **Memory protection** - Decrypted data cleared from RAM
- **Temporary file cleanup** - No unencrypted traces left
- **Integrity verification** - Detects corrupted packages

### Permission Management 👑

- **Elevated privileges** - May require administrator rights
- **User context** - Files imported with correct ownership
- **Security descriptors** - Windows permissions maintained
- **Audit trail** - Import actions logged for review

## ⚡ Performance Optimization

### For Faster Imports

1. **Close target applications** 📴
   - Exit browsers during browser data import
   - Close Outlook during signature import
   - Free system resources

2. **Sufficient disk space** 💾
   - Ensure target drive has adequate space
   - Import process needs temporary space
   - Final files may be smaller than estimate

3. **Stable system** 🔌
   - Don't run during system updates
   - Ensure stable power supply
   - Avoid heavy system loads

### Large Import Optimization

1. **Selective importing** 🎯
   - Import in smaller batches
   - Start with critical data first
   - Resume with additional content later

2. **Storage considerations** 💾
   - Use fastest available drive
   - Consider SSD for target location
   - Monitor available space throughout

## ⚠️ Common Import Issues

### Password Problems 🔐

**"Wrong password" errors**
- Verify password case sensitivity
- Check for extra spaces
- Ensure caps lock state
- Try "Skip Detection" if password is correct

**Package corruption**
- Verify .nilo file integrity
- Re-download if transferred over network
- Check for antivirus interference

### Permission Issues 👑

**"Access denied" errors**
- Run NiloShift as Administrator
- Check target user permissions
- Verify disk space availability
- Ensure antivirus isn't blocking

### Browser Import Issues 🌐

**Browser not detected**
- Install target browsers first
- Run browsers once to create profiles
- Close browsers before importing

**Bookmark conflicts**
- Review merged bookmarks after import
- Remove duplicates manually if desired
- Reorganize bookmark folders as needed

## ✅ Import Completion

When import finishes successfully:

1. **Success notification** 🔔
   - "Import completed successfully!"
   - Summary of imported content

2. **Verify restored data** ✅
   - Check target folders for files
   - Open browsers to verify bookmarks
   - Test Outlook signatures

3. **Clean up** 🧹
   - Temporary files automatically removed
   - Original .nilo package preserved
   - Import log saved for review

## 📋 Post-Import Steps

After successful import:

### Verify Applications 🔍

1. **Browser verification**
   - Open each browser
   - Check bookmarks are present
   - Verify saved passwords work
   - Test extensions functionality

2. **Email verification**
   - Open Outlook
   - Check signature availability
   - Test signature formatting
   - Verify default signature setting

3. **File verification**
   - Browse restored folders
   - Open sample files
   - Check folder permissions
   - Verify shortcuts work

### Application Updates 🔄

1. **Browser extensions**
   - Reinstall extensions as needed
   - Some extensions may need reconfiguration
   - Update extension settings

2. **File associations**
   - Check file type associations
   - Update default programs if needed
   - Configure new file handlers

## 🧪 Testing Your Import

### Recommended testing:

1. **Create test profile** 👤
   - Use non-critical user account first
   - Verify import process works
   - Check for any issues

2. **Partial test import** 📦
   - Import only non-critical data first
   - Verify browser bookmarks work
   - Test file accessibility

3. **Full production import** 🚀
   - Once testing successful
   - Import to production profile
   - Verify all functionality

## 🆘 Troubleshooting

For import problems:
- Check [Troubleshooting Guide](Troubleshooting)
- Review [Known Issues](Known-Issues)
- Contact support at hello@nilovon.com

## 📖 Related Guides

- [Export Guide](Export-Guide) - Creating backup packages
- [Selective Migration](Selective-Migration) - Advanced options
- [Security Features](Security-Features) - Understanding protection

---

**Import successful? Welcome to your restored environment!** 🎉

*Next: [Post-Migration Tips](Post-Migration-Tips)*
