# Anti-Interview - User Guide

## Table of Contents
1. [Introduction](#introduction)
2. [Installation](#installation)
3. [Getting Started](#getting-started)
4. [Features](#features)
5. [Settings](#settings)
6. [Troubleshooting](#troubleshooting)

---

## Introduction

Anti-Interview allows you to hide specific windows from screen sharing applications while continuing to use them normally. Perfect for online interviews, meetings, or presentations.

**What it does:**
- Hides windows from Zoom, Teams, Discord, OBS, etc.
- Optionally hides processes from Task Manager
- Works with multiple monitors
- Remembers your settings

**What it doesn't do:**
- Does NOT hide your physical screen
- Does NOT work with physical cameras
- Does NOT hide from remote desktop connections

---

## Installation

### Method 1: Installer (Recommended)

1. Download `anti-interview-setup-v1.0.0.exe`
2. Right-click → **Run as administrator**
3. Follow installation wizard
4. Launch from Start Menu

### Method 2: Portable

1. Download `anti-interview-v1.0.0-portable.zip`
2. Extract to a folder (e.g., `C:\Tools\AntiInterview`)
3. Right-click `anti-interview.exe` → **Run as administrator**

**Why administrator?**
- Required for hiding from Task Manager
- Required for some protected windows
- Screen capture hiding works without admin

---

## Getting Started

### Basic Usage (5 steps)

1. **Launch the application**
   - Run as administrator for full features

2. **Navigate to Windows section**
   - Click the Windows icon in the side menu

3. **Select windows to hide**
   - Check the boxes next to windows you want to hide
   - Use the filter to search for specific windows

4. **Start your screen sharing**
   - Open Zoom/Teams/Discord
   - Start sharing your screen

5. **Verify**
   - Hidden windows won't appear in the shared screen
   - You can still see and use them normally

### Quick Hide Browsers/IDEs

1. Go to **Settings**
2. Scroll to **"Ocultar Aplicações Comuns"**
3. Check boxes for:
   - 🦊 Firefox
   - 🌐 Microsoft Edge
   - 🎨 Google Chrome
   - 💻 Visual Studio Code
   - 🔧 Visual Studio

All windows from these applications will be hidden instantly.

---

## Features

### 1. Window Management

**Hide Individual Windows:**
- Go to **Windows** section
- Check boxes to hide specific windows
- Uncheck to show them again

**Filter Windows:**
- Type in the filter box to search
- Searches both window titles and process names

**Refresh List:**
- Click **"Atualizar Lista"** to refresh window list
- Automatically refreshes when app gains focus

### 2. Screen Capture Preview

**Enable Preview:**
1. Go to **Settings**
2. Check **"Mostrar preview da área de trabalho"**
3. Preview appears in Windows section

**Multi-Monitor:**
- Click **"Tela 1"**, **"Tela 2"**, etc. to switch monitors
- Preview shows what others see on that monitor

### 3. Task Manager Hiding

**Enable:**
1. Go to **Settings** > **Comportamento de Janelas**
2. Check **"Ocultar do Task Manager"**
3. Hidden windows will also hide from Task Manager

**Advanced (NT Hook):**
1. Go to **Settings** > **Avançado**
2. Check **"Ativar NT Hook"**
3. Hides ALL processes (including child processes without windows)

**Note:** NT Hook requires `hook.dll` in the same folder.

### 4. System Tray

**Minimize to Tray:**
1. Go to **Settings** > **Comportamento**
2. Enable **"Minimize to tray"** (if available)
3. Click X to minimize to tray
4. Right-click tray icon to restore

### 5. Themes

**Switch Theme:**
1. Go to **Settings**
2. Toggle **"Tema Escuro"**
3. Changes between dark and light theme

---

## Settings

### Interface

| Setting | Description |
|---------|-------------|
| **Tema Escuro** | Toggle dark/light theme |
| **Ocultar de Alt+Tab** | Remove hidden windows from task switcher |
| **Mostrar preview** | Show real-time screen capture preview |

### Window Behavior

| Setting | Description |
|---------|-------------|
| **Ignorar Mouse** | Hidden windows become click-through |
| **Ocultar do Task Manager** | Hide processes from Task Manager |

### Quick Hide

| Application | Description |
|-------------|-------------|
| 🦊 **Firefox** | Hide all Firefox windows |
| 🌐 **Microsoft Edge** | Hide all Edge windows |
| 🎨 **Google Chrome** | Hide all Chrome windows |
| 💻 **Visual Studio Code** | Hide all VS Code windows |
| 🔧 **Visual Studio** | Hide all Visual Studio windows |

### Advanced

| Setting | Description |
|---------|-------------|
| **Ativar NT Hook** | Enable NT API hooking for complete Task Manager hiding |

**⚠ Requires hook.dll**

### Keyboard Shortcuts

| Shortcut | Action |
|----------|--------|
| `Ctrl+Shift+S` | Take screenshot |
| `Ctrl+Shift+H` | Hide selected window |
| `Ctrl+Shift+I` | Show/Hide application |

---

## Troubleshooting

### Windows not hiding from screen share

**Solution:**
1. Refresh window list
2. Verify window is checked
3. Restart screen sharing
4. Restart application

### "Access Denied" errors

**Solution:**
- Run as administrator
- Some system processes are protected and cannot be hidden

### Task Manager hiding not working

**Solution:**
1. Enable **"Ocultar do Task Manager"** in Settings
2. For complete hiding, enable **"Ativar NT Hook"**
3. Ensure `hook.dll` is present
4. Run as administrator

### NT Hook not available

**Solution:**
1. Check if `hook.dll` exists in application folder
2. Build hook.dll using `build_hook_simple.bat`
3. Copy `hook.dll` to application folder

### Application crashes

**Solution:**
1. Delete config file: `C:\Users\{username}\AppData\Roaming\AntiInterview\config.toml`
2. Restart application
3. Check `anti-interview.log` for errors

### High CPU usage

**Solution:**
1. Disable **"Mostrar preview"** in Settings
2. Close unnecessary windows
3. Reduce number of hidden windows

---

## Best Practices

### For Interviews

1. **Test before the interview:**
   - Hide windows you need
   - Start screen sharing
   - Verify they're hidden

2. **Keep it simple:**
   - Only hide what you need
   - Don't hide too many windows

3. **Have a backup plan:**
   - Know how to quickly unhide windows
   - Keep important windows accessible

### For Presentations

1. **Use preview:**
   - Enable preview to see what audience sees
   - Check before presenting

2. **Multi-monitor:**
   - Hide windows on secondary monitor
   - Present from primary monitor

3. **Quick hide:**
   - Use browser/IDE checkboxes for quick hiding
   - Prepare before presentation starts

---

## FAQ

**Q: Does this work with Zoom/Teams/Discord?**
A: Yes, works with any application using Windows screen capture APIs.

**Q: Can others see my hidden windows?**
A: No, hidden windows are invisible to screen capture.

**Q: Can I still use hidden windows?**
A: Yes, you can see and use them normally on your screen.

**Q: Does this require internet?**
A: No, works completely offline.

**Q: Is this safe?**
A: Yes, uses standard Windows APIs. Open source code available.

**Q: Why administrator privileges?**
A: Required for Task Manager hiding and some protected windows. Screen capture hiding works without admin.

**Q: Does this work on Mac/Linux?**
A: No, Windows only (uses Windows-specific APIs).

---

## Support

**Issues or questions?**
- Check troubleshooting section
- Review logs in `anti-interview.log`
- Contact support: [Email/GitHub]

**Want to contribute?**
- GitHub: [Repository URL]
- Report bugs
- Suggest features

---

## Credits

Developed by: **Matheus & Pyetrah**

Built with: Rust, Windows API, egui

Inspired by: InvisWind by Radiantly

---

**Version:** 1.0.0  
**Last Updated:** 2025-03-20
