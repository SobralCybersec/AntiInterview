@echo off
echo ========================================
echo Building Anti-Interview Release Package
echo ========================================
echo.

echo [1/3] Building Rust release...
cargo build --release
if %errorlevel% neq 0 (
    echo ERROR: Rust build failed
    exit /b 1
)
echo.

echo [2/3] Creating portable ZIP package...
if not exist "release" mkdir release
powershell -Command "Compress-Archive -Path 'target\release\anti-interview.exe','target\release\utils.dll','hook\out\build\x64-Debug\hook_notepad.dll','hook\out\build\x64-Debug\hook_firefox.dll','hook\out\build\x64-Debug\hook_edge.dll','hook\out\build\x64-Debug\hook_chrome.dll','hook\out\build\x64-Debug\hook_vscode.dll','hook\out\build\x64-Debug\hook_visualstudio.dll','hook\out\build\x64-Debug\hook_antiinterview.dll','README.md','LICENSE','RELEASE_NOTES.md','RELEASE_NOTES_PT.md','assets' -DestinationPath 'release\anti-interview-v1.0.0-portable.zip' -Force"
echo.

echo [3/3] Building installer with Inno Setup...
"C:\Program Files (x86)\Inno Setup 6\ISCC.exe" installer.iss
if %errorlevel% neq 0 (
    echo WARNING: Inno Setup not found or build failed
    echo Installer creation skipped
) else (
    echo Installer created successfully
)
echo.

echo ========================================
echo Build Complete!
echo ========================================
echo.
echo Portable ZIP: release\anti-interview-v1.0.0-portable.zip
echo Installer: installer\anti-interview-setup-v1.0.0.exe
echo.
pause
