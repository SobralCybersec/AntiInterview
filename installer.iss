#define MyAppName "Anti-Interview"
#define MyAppVersion "1.0.0"
#define MyAppPublisher "Matheus & Pyetrah"
#define MyAppURL "https://github.com/yourusername/InvisWind"
#define MyAppExeName "anti-interview.exe"

[Setup]
AppId={{8F9A2B3C-4D5E-6F7A-8B9C-0D1E2F3A4B5C}
AppName={#MyAppName}
AppVersion={#MyAppVersion}
AppPublisher={#MyAppPublisher}
AppPublisherURL={#MyAppURL}
AppSupportURL={#MyAppURL}
AppUpdatesURL={#MyAppURL}
DefaultDirName={autopf}\{#MyAppName}
DefaultGroupName={#MyAppName}
AllowNoIcons=yes
LicenseFile=LICENSE
OutputDir=installer
OutputBaseFilename=anti-interview-setup-v{#MyAppVersion}
SetupIconFile=assets\icons\invicon.ico
Compression=lzma2/ultra64
SolidCompression=yes
WizardStyle=modern
PrivilegesRequired=admin
ArchitecturesAllowed=x64compatible
ArchitecturesInstallIn64BitMode=x64compatible
UninstallDisplayIcon={app}\{#MyAppExeName}

[Languages]
Name: "english"; MessagesFile: "compiler:Default.isl"
Name: "portuguese"; MessagesFile: "compiler:Languages\BrazilianPortuguese.isl"

[Tasks]
Name: "desktopicon"; Description: "{cm:CreateDesktopIcon}"; GroupDescription: "{cm:AdditionalIcons}"; Flags: unchecked
Name: "quicklaunchicon"; Description: "{cm:CreateQuickLaunchIcon}"; GroupDescription: "{cm:AdditionalIcons}"; Flags: unchecked; OnlyBelowVersion: 6.1; Check: not IsAdminInstallMode

[Files]
Source: "target\release\{#MyAppExeName}"; DestDir: "{app}"; Flags: ignoreversion
Source: "target\release\utils.dll"; DestDir: "{app}"; Flags: ignoreversion
Source: "hook\out\build\x64-Debug\hook_notepad.dll"; DestDir: "{app}"; Flags: ignoreversion
Source: "hook\out\build\x64-Debug\hook_firefox.dll"; DestDir: "{app}"; Flags: ignoreversion
Source: "hook\out\build\x64-Debug\hook_edge.dll"; DestDir: "{app}"; Flags: ignoreversion
Source: "hook\out\build\x64-Debug\hook_chrome.dll"; DestDir: "{app}"; Flags: ignoreversion
Source: "hook\out\build\x64-Debug\hook_vscode.dll"; DestDir: "{app}"; Flags: ignoreversion
Source: "hook\out\build\x64-Debug\hook_visualstudio.dll"; DestDir: "{app}"; Flags: ignoreversion
Source: "hook\out\build\x64-Debug\hook_antiinterview.dll"; DestDir: "{app}"; Flags: ignoreversion
Source: "README.md"; DestDir: "{app}"; Flags: ignoreversion
Source: "LICENSE"; DestDir: "{app}"; Flags: ignoreversion
Source: "RELEASE_NOTES.md"; DestDir: "{app}"; Flags: ignoreversion
Source: "RELEASE_NOTES_PT.md"; DestDir: "{app}"; Flags: ignoreversion
Source: "assets\*"; DestDir: "{app}\assets"; Flags: ignoreversion recursesubdirs createallsubdirs

[Icons]
Name: "{group}\{#MyAppName}"; Filename: "{app}\{#MyAppExeName}"
Name: "{group}\{cm:UninstallProgram,{#MyAppName}}"; Filename: "{uninstallexe}"
Name: "{autodesktop}\{#MyAppName}"; Filename: "{app}\{#MyAppExeName}"; Tasks: desktopicon
Name: "{userappdata}\Microsoft\Internet Explorer\Quick Launch\{#MyAppName}"; Filename: "{app}\{#MyAppExeName}"; Tasks: quicklaunchicon

[Run]
Filename: "{app}\{#MyAppExeName}"; Description: "{cm:LaunchProgram,{#StringChange(MyAppName, '&', '&&')}}"; Flags: nowait postinstall skipifsilent runascurrentuser

[Code]
function InitializeSetup(): Boolean;
begin
  Result := True;
  if not IsWin64 then
  begin
    MsgBox('Este aplicativo requer Windows 64-bit.', mbError, MB_OK);
    Result := False;
  end;
end;
