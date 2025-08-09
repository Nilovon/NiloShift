# Troubleshooting Guide 🔧

Having issues with NiloShift? This comprehensive troubleshooting guide will help you resolve common problems and get back to migrating your data safely.

## 🚨 Emergency Quick Fixes

### Can't start NiloShift?
1. **Right-click** → **"Run as administrator"** 👑
2. **Temporarily disable antivirus** 🛡️
3. **Check Windows version** (Windows 10 1903+ required) 💻

### Import/Export stuck?
1. **Check available disk space** 💾
2. **Close other applications** 📴
3. **Restart NiloShift** and try again 🔄

### "Access denied" errors?
1. **Run as administrator** 👑
2. **Check user permissions** 👤
3. **Disable file/folder compression** 📦

## 📋 Common Issues & Solutions

### 🚀 Installation & Startup Issues

#### **"This app can't run on your PC"**

**Cause**: Wrong architecture or old Windows version

**Solutions**:
- ✅ Ensure you're running Windows 10 (1903) or later
- ✅ Download x64 version for 64-bit Windows
- ✅ Check Windows Update for latest updates
- ✅ Try portable version instead of installer

#### **"Windows protected your PC" (SmartScreen)**

**Cause**: Unsigned executable triggers Windows Defender

**Solutions**:
- ✅ Click "More info" → "Run anyway"
- ✅ Add NiloShift to Windows Defender exclusions
- ✅ Download only from official GitHub releases
- ✅ Verify file integrity if concerned

#### **Antivirus blocking NiloShift**

**Cause**: False positive due to file system access

**Solutions**:
- ✅ Add NiloShift folder to antivirus exclusions
- ✅ Whitelist `niloshift.exe` specifically
- ✅ Temporarily disable real-time protection
- ✅ Submit false positive report to antivirus vendor

#### **MSI installer fails**

**Cause**: Windows Installer issues or insufficient permissions

**Solutions**:
- ✅ Run installer as Administrator
- ✅ Restart Windows Installer service
- ✅ Clear Windows Installer cache
- ✅ Use portable version as alternative
- ✅ Check Event Viewer for detailed error

### 🔐 Authentication & Permissions

#### **"Access Denied" during export/import**

**Cause**: Insufficient permissions to access user profiles

**Solutions**:
- ✅ **Run NiloShift as Administrator** (most common fix)
- ✅ Check target user account permissions
- ✅ Ensure you have rights to source/destination folders
- ✅ Disable file compression on target folders
- ✅ Check Windows UAC settings

#### **Can't access other user profiles**

**Cause**: Windows security restrictions

**Solutions**:
- ✅ Log in as target user and run export
- ✅ Use Administrator account with full privileges
- ✅ Enable "Replace owner on subcontainers" in security settings
- ✅ Grant "Log on as a service" rights if needed

#### **"Profile not found" errors**

**Cause**: User profile corruption or special account type

**Solutions**:
- ✅ Check if user profile loads correctly
- ✅ Verify account is local (not Microsoft account sync issue)
- ✅ Rebuild user profile if corrupted
- ✅ Use `lusrmgr.msc` to check account status

### 📤 Export Problems

#### **Export hangs or freezes**

**Cause**: Large files, insufficient memory, or disk issues

**Solutions**:
- ✅ **Check available disk space** (need 2-3x data size temporarily)
- ✅ Close memory-intensive applications
- ✅ Restart NiloShift and try again
- ✅ Export smaller data sets separately
- ✅ Move temp folder to faster drive (SSD)

#### **"Insufficient disk space" during export**

**Cause**: Not enough space for temporary files

**Solutions**:
- ✅ Free up disk space (need 2-3x final export size)
- ✅ Change temp directory to drive with more space
- ✅ Export to external drive directly
- ✅ Selective export of smaller data sets
- ✅ Clean browser cache before exporting

#### **Very slow export performance**

**Cause**: Large browser data, fragmented disk, or antivirus scanning

**Solutions**:
- ✅ **Clear browser cache/history** before export
- ✅ Add NiloShift temp folder to antivirus exclusions
- ✅ Use SSD for temporary files
- ✅ Close browsers during export
- ✅ Export during low system activity periods

#### **Browser data not found**

**Cause**: Non-standard browser installation or profile location

**Solutions**:
- ✅ Run browsers once to create default profiles
- ✅ Check if browsers are installed for current user vs. system-wide
- ✅ Verify browser profile paths are standard
- ✅ Export from each browser's data directory manually

### 📥 Import Problems

#### **"Wrong password" errors**

**Cause**: Incorrect password, corrupted package, or encoding issues

**Solutions**:
- ✅ **Double-check password case sensitivity**
- ✅ Ensure no extra spaces before/after password
- ✅ Verify Caps Lock and Num Lock states
- ✅ Try "Skip Detection" option
- ✅ Re-create export if package seems corrupted

#### **Import hangs at "Decrypting"**

**Cause**: Large package, slow disk, or corrupted encryption

**Solutions**:
- ✅ **Be patient** - large packages take time
- ✅ Check Task Manager for disk activity
- ✅ Ensure stable power supply
- ✅ Close other applications using disk
- ✅ Try importing to faster drive (SSD)

#### **Package detection fails**

**Cause**: Corrupted package or detection algorithm issues

**Solutions**:
- ✅ **Use "Skip Detection" button**
- ✅ Verify package file integrity
- ✅ Re-download package if transferred over network
- ✅ Check antivirus isn't quarantining parts
- ✅ Try different import location

#### **Partial import success**

**Cause**: File conflicts, permission issues, or disk space

**Solutions**:
- ✅ **Check import log** for specific errors
- ✅ Verify sufficient disk space for remaining import
- ✅ Resolve file permission conflicts
- ✅ Continue import with remaining unprocessed items
- ✅ Import failed items separately

### 🌐 Browser-Specific Issues

#### **Chrome bookmarks not importing**

**Cause**: Chrome profile locked or non-standard location

**Solutions**:
- ✅ **Close Chrome completely** before import
- ✅ Check Chrome profile directory exists
- ✅ Import to new Chrome profile first
- ✅ Manually import bookmarks using Chrome's import feature
- ✅ Verify Chrome version compatibility

#### **Edge password import fails**

**Cause**: Windows Credential Manager conflicts

**Solutions**:
- ✅ Close Edge and all Edge processes
- ✅ Clear Edge cache before import
- ✅ Import to new Edge profile
- ✅ Use Edge's built-in import wizard as fallback
- ✅ Check Windows Credential Manager permissions

#### **Firefox profile corruption**

**Cause**: Profile conflict with existing Firefox data

**Solutions**:
- ✅ **Close Firefox completely** (check Task Manager)
- ✅ Create new Firefox profile for import
- ✅ Use Firefox Profile Manager
- ✅ Import to clean Firefox installation
- ✅ Restore from Firefox's bookmark backup

### 📧 Outlook Signature Issues

#### **Signatures not appearing in Outlook**

**Cause**: Outlook not recognizing signature files

**Solutions**:
- ✅ **Restart Outlook** after import
- ✅ Check Outlook signature folder path
- ✅ Verify signature files are in correct format
- ✅ Manually set default signature in Outlook
- ✅ Recreate signatures if formatting is lost

#### **Signature formatting broken**

**Cause**: HTML/image references broken during import

**Solutions**:
- ✅ Check embedded images are present
- ✅ Verify HTML formatting preserved
- ✅ Recreate complex signatures manually
- ✅ Use plain text versions as fallback
- ✅ Test signatures in new email

### 💾 Performance & System Issues

#### **High memory usage during migration**

**Cause**: Large data sets being processed in memory

**Solutions**:
- ✅ **Close unnecessary applications**
- ✅ Increase virtual memory if needed
- ✅ Process smaller data sets
- ✅ Use 64-bit Windows (if available)
- ✅ Restart system before large migrations

#### **System becomes unresponsive**

**Cause**: Disk I/O saturation or memory exhaustion

**Solutions**:
- ✅ **Lower process priority** in Task Manager
- ✅ Pause migration during high system activity
- ✅ Use faster storage (SSD) for temp files
- ✅ Ensure adequate cooling for prolonged operations
- ✅ Schedule migrations during low-activity periods

#### **Network transfer corruption**

**Cause**: Package corruption during file transfer

**Solutions**:
- ✅ **Use file integrity verification** (checksums)
- ✅ Transfer using reliable methods (USB, reliable network)
- ✅ Avoid WiFi for large package transfers
- ✅ Use error-checking transfer protocols
- ✅ Verify package size after transfer

## 🔍 Advanced Diagnostics

### Enable Debug Logging

1. Run NiloShift from command line with debug flag:
   ```cmd
   niloshift.exe --debug
   ```

2. Check log files in:
   ```
   %APPDATA%\NiloShift\logs\
   ```

3. Include relevant log excerpts when reporting issues

### Check System Health

1. **Disk Health**:
   - Run `chkdsk C: /f` for disk errors
   - Check SMART status for disk health
   - Verify adequate free space

2. **Memory Test**:
   - Run Windows Memory Diagnostic
   - Check for memory errors in Event Viewer
   - Monitor memory usage during migration

3. **Permissions Audit**:
   - Use `icacls` to check folder permissions
   - Verify effective permissions with `whoami /priv`
   - Check security event logs for access denials

### Event Viewer Analysis

Check these Event Viewer locations for NiloShift-related errors:

- **Windows Logs > Application**
- **Windows Logs > System** 
- **Windows Logs > Security**
- **Applications and Services Logs**

## 🆘 Getting Help

### Before Contacting Support

Please gather this information:

1. **System Information** 💻
   - Windows version and build
   - Available RAM and disk space
   - Antivirus software used
   - Administrator privileges status

2. **Error Details** 🐛
   - Exact error message
   - Steps leading to the error
   - Screenshots if helpful
   - Log file excerpts

3. **Package Information** 📦
   - Export source system details
   - Package size and creation date
   - Data types included
   - Transfer method used

### Support Channels

1. **GitHub Issues** 🐛
   - [Search existing issues](https://github.com/Nilovon/NiloShift/issues)
   - [Create new issue](https://github.com/Nilovon/NiloShift/issues/new)
   - Include all relevant information

2. **Email Support** 📧
   - Send details to: hello@nilovon.com
   - Include system information and error details
   - Attach log files if available

3. **Community Help** 💬
   - Check GitHub Discussions
   - Search for similar problems
   - Share solutions with others

## 📚 Related Documentation

- [Known Issues](Known-Issues) - Current limitations
- [System Requirements](System-Requirements) - Compatibility info
- [Security Features](Security-Features) - Understanding protection
- [Performance Tips](Performance-Tips) - Optimization guide

---

**Still having trouble?** Don't hesitate to reach out! 🤝

*We're here to help you succeed with your data migration.* ✨
