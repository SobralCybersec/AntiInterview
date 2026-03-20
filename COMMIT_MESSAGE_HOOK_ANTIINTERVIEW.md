Feat: Add hook_antiinterview.dll to hide anti-interview.exe from Task Manager

- Created hook_antiinterview.cpp with hardcoded HIDE_PROCNAME L"anti-interview.exe"
- Updated CMakeLists.txt to build 7th specialized hook DLL
- Added hook_antiinterview field to UiSettings configuration
- Added checkbox in Settings > Injetar Hooks no Task Manager section
- Updated ProcessMonitor to auto-inject hook_antiinterview.dll
- Updated installer.iss to include hook_antiinterview.dll
- Created build_release_package.bat for automated release builds
- Created create_zip.bat for quick portable ZIP creation

Files Modified:
- hook/hook_antiinterview.cpp (new)
- hook/CMakeLists.txt
- injector/src/domain/configuration.rs
- injector/src/presentation/gui.rs
- injector/src/presentation/app_state.rs
- installer.iss
- build_release_package.bat (new)
- create_zip.bat (new)

Total Hook DLLs: 7
- hook_notepad.dll
- hook_firefox.dll
- hook_edge.dll
- hook_chrome.dll
- hook_vscode.dll
- hook_visualstudio.dll
- hook_antiinterview.dll
