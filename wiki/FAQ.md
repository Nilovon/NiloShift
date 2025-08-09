# Frequently Asked Questions (FAQ) ❓

Find quick answers to the most common questions about NiloShift, the ultimate Windows profile migration tool.

## 🚀 General Questions

### What is NiloShift?

**NiloShift** is a free, secure desktop application that helps you migrate Windows user profiles between computers. It backs up your files, browser data, and settings into an encrypted package that can be restored on any Windows machine.

### Is NiloShift really free?

**Yes!** NiloShift is completely free for **non-commercial use** under the MIT license. For businesses and commercial environments, please contact hello@nilovon.com for licensing.

### What data can NiloShift migrate?

NiloShift can backup and restore:
- 📁 **Desktop files** and shortcuts
- 📄 **Documents** folder contents
- 🖼️ **Pictures** and photo collections
- ⬇️ **Downloads** folder
- 🌐 **Browser profiles** (Chrome, Edge, Firefox) - bookmarks, passwords, history
- 📧 **Outlook signatures** and email templates

### How secure is my data?

**Very secure!** NiloShift uses:
- 🛡️ **AES-256-GCM encryption** - Military-grade security
- 🔐 **Password protection** - Only you can decrypt your data
- 🏠 **Local processing** - Data never leaves your computer
- 🔍 **Open source** - Code is publicly auditable

## 💻 System Requirements

### What Windows versions are supported?

- **Windows 10** (version 1903 or later) ✅
- **Windows 11** (all versions) ✅
- **Older Windows versions** ❌ Not supported

### How much disk space do I need?

- **NiloShift itself**: ~500 MB
- **For exports**: 2-3x the size of your data (temporarily)
- **Final package**: Compressed, usually 50-70% of original size

### Do I need administrator rights?

**Recommended but not always required:**
- ✅ **With admin**: Full access to all user profiles and system areas
- ⚠️ **Without admin**: Limited to current user profile only
- 👑 **Best practice**: Run as administrator for complete functionality

### Can I run NiloShift on a USB drive?

**Yes!** The portable version (`niloshift.exe`) can run from:
- USB flash drives
- External hard drives  
- Network locations
- Any writable location

## 📦 Export Questions

### How long does an export take?

**Depends on data size:**
- **Small profiles** (< 1 GB): 2-5 minutes
- **Medium profiles** (1-10 GB): 10-30 minutes  
- **Large profiles** (10+ GB): 30+ minutes

**Factors affecting speed:**
- Amount of browser data
- Pictures and large files
- Disk speed (SSD vs HDD)
- System performance

### Can I export from multiple user accounts?

**One at a time:** Each export targets a single Windows user profile. To backup multiple users:
1. Export first user profile
2. Create separate package for second user
3. Import each package to respective target users

### What if I forget my export password?

**Unfortunately, there's no password recovery.** The encryption is designed so that without the password, the data cannot be recovered. This ensures maximum security.

**Best practices:**
- Use a memorable but strong password
- Write it down securely
- Consider using a password manager

### Can I export while using my computer?

**Yes, but with considerations:**
- 📧 Close browsers for complete browser data export
- 💾 Ensure sufficient disk space
- ⚡ Performance may be slower during export
- 🔄 Don't modify files being exported

## 📥 Import Questions

### Can I import to a different Windows version?

**Generally yes:**
- Windows 10 → Windows 11 ✅
- Windows 11 → Windows 10 ✅  
- Older → Newer ✅ (usually works)
- Newer → Older ⚠️ (may have compatibility issues)

### What happens to existing data during import?

**NiloShift preserves existing data:**
- 📁 **Files**: Existing files renamed with timestamp
- 🌐 **Bookmarks**: Merged with existing bookmarks
- 🔐 **Passwords**: Updated if newer, kept if current
- ⚙️ **Settings**: Imported settings typically take precedence

### Can I import only specific items?

**Absolutely!** NiloShift offers **selective import:**
- ✅ Choose specific data types (files, bookmarks, etc.)
- ✅ Mix and match as needed
- ✅ Skip unwanted content
- ✅ Import in multiple sessions

### Can I import the same package multiple times?

**Yes!** You can:
- Import same package to multiple computers
- Re-import to same computer if needed
- Import different parts at different times
- Use for multiple user accounts

## 🌐 Browser-Specific Questions

### Which browsers are supported?

**Fully Supported:**
- 🔵 **Google Chrome** - Bookmarks, passwords, history, extensions
- 🔷 **Microsoft Edge** - Favorites, passwords, history, settings  
- 🦊 **Mozilla Firefox** - Bookmarks, passwords, history, add-ons

**Not Currently Supported:**
- Safari, Opera, Brave (may work partially)
- Specialized browsers
- Browser profiles in non-standard locations

### Do extensions get transferred?

**Extensions are marked for reinstallation:**
- ✅ Extension preferences and data backed up
- ⚠️ Extensions themselves need manual reinstallation
- ✅ Extension settings restored after reinstall
- 🔧 Some extensions may need reconfiguration

### Will my saved passwords work?

**Yes, in most cases:**
- ✅ Passwords imported to browser's password manager
- ✅ Automatic form filling should work
- ✅ Sync with browser accounts (Chrome, Edge) maintained
- ⚠️ Some sites may require re-authentication for security

## 🔒 Security & Privacy

### Where is my data during migration?

**Your data stays local:**
1. **Export**: Encrypted on your source computer
2. **Transfer**: You control how package is moved
3. **Import**: Decrypted on your target computer
4. **Never**: Uploaded to any servers or cloud services

### Can NiloShift decrypt my data without my password?

**No!** Even Nilovon cannot decrypt your data without your password. The encryption is designed so that:
- 🔐 Only your password can decrypt the data
- 🛡️ No "master key" or backdoor exists
- 🔒 Password is never transmitted or stored
- 🏠 All encryption/decryption happens locally

### What happens if my computer crashes during migration?

**NiloShift is designed to be safe:**
- 📦 **Export**: Incomplete packages are invalid and won't import
- 📥 **Import**: Partial imports can be resumed or restarted
- 🗑️ **Cleanup**: Temporary files are automatically removed
- 💾 **Original data**: Never modified during export

### Can I verify package integrity?

**Yes!** NiloShift includes integrity checking:
- ✅ Automatic verification during import
- ✅ Detects corrupted or tampered packages
- ✅ Warns if package integrity is compromised
- ✅ Built-in error detection and reporting

## 🔧 Technical Questions

### What file format does NiloShift use?

**`.nilo` files are:**
- 📦 Compressed ZIP archives (for efficiency)
- 🔐 AES-256-GCM encrypted (for security)  
- 🏷️ Custom header with metadata
- ✅ Integrity checksums for verification

### Can I open .nilo files with other tools?

**No, by design:**
- 🔒 Files are encrypted and require NiloShift to decrypt
- 🛡️ This ensures security and data protection
- 🔧 Only NiloShift knows the proper decryption process
- ⚠️ Attempting to open with other tools may corrupt the package

### Does NiloShift work offline?

**Completely offline:**
- ✅ No internet connection required
- ✅ All processing happens locally
- ✅ Perfect for air-gapped systems
- ✅ Works on isolated networks

### Can I automate NiloShift?

**Currently manual only:**
- 🖱️ GUI-based application requiring user interaction
- 🔐 Password entry requires manual input for security
- 📋 Selective choices need user decisions
- 🔮 Command-line interface planned for future versions

## 💼 Business & Licensing

### Can I use NiloShift in my business?

**Commercial license required:**
- 🆓 **Free**: Personal, educational, non-profit use
- 💼 **Commercial**: Business use requires paid license
- 📧 **Contact**: hello@nilovon.com for business licensing
- 🏢 **Volume**: Discounts available for multiple licenses

### Can I redistribute NiloShift?

**Under specific conditions:**
- ✅ **Non-commercial redistribution**: Allowed under MIT license
- ✅ **Must include**: Copyright notice and license text
- ❌ **Commercial redistribution**: Requires permission
- 🔧 **Modifications**: Must be clearly marked as modified

### Is support included?

**Community support is free:**
- 📖 Comprehensive documentation and wiki
- 🐛 GitHub issues for bug reports
- 💬 Community discussions and help
- 📧 **Paid support**: Available for commercial licenses

## 🔮 Future Features

### What's planned for future versions?

**Roadmap includes:**
- 🔄 **Incremental backups** - Only backup changed files
- ☁️ **Cloud integration** - Optional cloud storage support
- 🖥️ **Multi-monitor settings** - Display configuration migration
- 🎮 **Game saves** - Steam, Epic Games, etc.
- 📱 **Mobile companion** - Remote management app
- 🤖 **Automation** - Command-line interface and scripting

### How often is NiloShift updated?

**Regular maintenance:**
- 🐛 **Bug fixes**: As needed
- 🔒 **Security updates**: High priority
- ✨ **New features**: Based on user feedback
- 📢 **Notifications**: Through GitHub releases

### Can I request features?

**Absolutely!**
- 💡 [Submit feature requests](https://github.com/Nilovon/NiloShift/issues) on GitHub
- 🗳️ Vote on existing feature requests
- 💬 Discuss ideas in GitHub Discussions
- 🔧 Contribute code if you're a developer

## 🆘 Still Have Questions?

### Getting More Help

1. **📖 Documentation**: Check our comprehensive [wiki](Home)
2. **🔍 Search**: Look through [existing issues](https://github.com/Nilovon/NiloShift/issues)
3. **🐛 Report**: [Create new issue](https://github.com/Nilovon/NiloShift/issues/new) for bugs
4. **📧 Email**: Contact hello@nilovon.com for direct support

### Quick Links

- [Download NiloShift](https://github.com/Nilovon/NiloShift/releases)
- [Installation Guide](Installation-Guide)
- [Quick Start Tutorial](Quick-Start-Tutorial)
- [Troubleshooting Guide](Troubleshooting)

---

**Question not answered here?** We'd love to help! 🤝

*Contact us and we'll add your question to this FAQ.* ✨
