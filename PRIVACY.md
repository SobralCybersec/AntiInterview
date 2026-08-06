# Privacy Notice

Last updated: August 6, 2026

Anti-Interview runs locally. The project does not operate a backend, user
account service, analytics endpoint, advertising system, or telemetry
collector.

## Data handled locally

The application may handle:

- Window titles, process names, process IDs, window handles, and monitor frames
  while listing windows or producing a live preview.
- Hotkeys, UI preferences, hidden-window identifiers, and enabled hook options
  in `AntiInterview/config.toml` under the operating system's user
  configuration directory.
- Diagnostic events in `anti-interview.log` in the process working directory.
  Logs may include process names, window titles, process IDs, URLs, DLL or
  screenshot paths, and error details.
- Silent screenshots in the operating system temporary directory, named
  `anti_interview_<timestamp>.png`.
- Screenshot pixels in the system clipboard when automatic clipboard copying
  is enabled.

The application processes live preview frames in memory. Its bounded capture
queue drops excess frames rather than storing an unbounded history.

## Network access and third parties

The integrated or external browser can connect to ChatGPT, Antigravity, Gemini,
and Claude. A configured remote banner URL can also make a network request.
Those services receive data according to their own privacy policies, including
content entered into their pages, IP address, cookies, and WebView or browser
storage. Anti-Interview does not proxy those connections through a
project-operated server.

## Retention and deletion

- Configuration remains until the user deletes
  `AntiInterview/config.toml` or its containing user configuration directory.
- The log file uses append mode and remains until the user rotates or deletes
  it.
- Temporary screenshots remain until the user or operating system removes
  them. Clearing the displayed screenshot path does not delete the PNG.
- Clipboard data remains under operating-system clipboard rules and may be
  available to other local applications.
- Third-party browser data follows the selected provider and WebView/browser
  retention controls.

Before sharing logs or screenshots, inspect and redact personal data,
credentials, internal URLs, and window titles. To remove local application
data, close the application, delete the configuration and log files, remove
`anti_interview_*.png` from the temporary directory, and clear the clipboard.

## Permissions

Core UI and configuration data use the current user account. Windows process
hooks may require administrator privileges. Elevation increases access to local
process information; enable it only for the operation that requires it.

Privacy questions that contain no sensitive data may use the repository issue
tracker. Send security-sensitive reports through the private channel in
[SECURITY.md](SECURITY.md).
