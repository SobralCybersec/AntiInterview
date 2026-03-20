# Anti-Interview v1.0.0 - Release Notes

**Release Date:** 2025-03-20

## Overview

Anti-Interview is a Windows application that hides windows from screen capture and sharing applications (Zoom, Teams, Discord, OBS) and optionally from Task Manager.

## Features

### Core Functionality
- **Window Hiding**: Hide specific windows from screen capture APIs
- **Screen Capture Preview**: Real-time preview of what others see
- **Multi-Monitor Support**: Works with multiple displays
- **System Tray Integration**: Minimize to tray
- **Persistent Configuration**: Settings saved across sessions

### Advanced Features
- **NT API Hook**: Hide processes from Task Manager (including child processes)
- **Browser Quick Hide**: One-click hiding for Firefox, Edge, Chrome
- **IDE Quick Hide**: One-click hiding for VS Code, Visual Studio
- **Task Manager Hiding**: Hide processes from Task Manager process list
- **Alt+Tab Hiding**: Remove windows from task switcher

### UI Features
- **Animated Side Menu**: Expandable navigation menu
- **Dark/Light Theme**: Toggle between themes
- **Animated Banner**: GIF banner support
- **Window Filtering**: Search and filter windows
- **Process Name Display**: Shows actual process names

## Installation

### Installer (Recommended)
1. Download `anti-interview-setup-v1.0.0.exe`
2. Run installer as administrator
3. Follow installation wizard
4. Launch from Start Menu or Desktop

### Portable Version
1. Download `anti-interview-v1.0.0-portable.zip`
2. Extract to any folder
3. Run `anti-interview.exe` as administrator (for full features)

## System Requirements

- **OS**: Windows 10/11 (64-bit)
- **RAM**: 100 MB minimum
- **Disk**: 50 MB free space
- **Privileges**: Administrator recommended for full features

## Quick Start

1. Launch Anti-Interview
2. Go to **Windows** section
3. Select windows to hide
4. Start screen sharing
5. Hidden windows won't appear in shared screen

## Advanced Setup (NT Hook)

For complete Task Manager hiding:

1. Go to **Settings** > **Avançado**
2. Enable **"Ativar NT Hook"**
3. Enable **"Ocultar do Task Manager"**
4. Hide browser/IDE windows
5. All child processes will be hidden from Task Manager

## Keyboard Shortcuts

| Shortcut | Action |
|----------|--------|
| `Ctrl+Shift+S` | Take screenshot |
| `Ctrl+Shift+H` | Hide selected window |
| `Ctrl+Shift+I` | Show/Hide GUI |

## Known Limitations

- Task Manager window itself requires admin privileges to hide
- Some protected system processes cannot be hidden
- NT Hook requires `hook.dll` to be present

## Compatibility

**Tested with:**
- Zoom
- Microsoft Teams
- Discord
- OBS Studio
- Google Meet
- Skype

**Browsers:**
- Google Chrome
- Microsoft Edge
- Mozilla Firefox

**IDEs:**
- Visual Studio Code
- Visual Studio 2019/2022

## Technical Details

- **Language**: Rust
- **UI Framework**: egui
- **Architecture**: Domain-Driven Design (DDD)
- **Screen Capture**: Windows API (SetWindowDisplayAffinity)
- **Process Hiding**: NT API Hooking (NtQuerySystemInformation)

## Files Included

```
anti-interview.exe    - Main application
utils.dll             - Payload DLL for window manipulation
hook.dll              - NT API hook DLL
README.md             - Documentation
LICENSE               - License information
assets/               - Icons, fonts, banner
```

## Configuration

Configuration stored at:
```
C:\Users\{username}\AppData\Roaming\AntiInterview\config.toml
```

## Troubleshooting

### "Access Denied" errors
- Run as administrator
- Some system processes are protected

### NT Hook not working
- Ensure `hook.dll` is in same directory as executable
- Enable NT Hook in Settings > Avançado

### Windows not hiding
- Check if window is in the list
- Refresh window list
- Restart application

## Support

For issues, questions, or contributions:
- GitHub: [Repository URL]
- Email: [Contact Email]

## Credits

**Developed by:** Matheus & Pyetrah

**Technologies:**
- Rust Programming Language
- Windows API
- egui Framework
- dll-syringe

**References:**
- [InvisWind by Radiantly](https://github.com/radiantly/Invisiwind)
- [Windows SetWindowDisplayAffinity](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowdisplayaffinity)

## License

See LICENSE file for details.

## Changelog

### v1.0.0 (2025-03-20)
- Initial release
- Window hiding from screen capture
- NT API hook for Task Manager hiding
- Browser/IDE quick hide
- Multi-monitor support
- System tray integration
- Dark/Light theme
- Animated UI
- Persistent configuration
