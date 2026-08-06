# Security Policy

## Supported versions

Security fixes target the current 1.2.x release line and the default branch.
Older release artifacts may not receive fixes.

## Reporting a vulnerability

Report vulnerabilities through GitHub private vulnerability reporting:

<https://github.com/SobralCybersec/AntiInterview/security/advisories/new>

Do not include exploit details, credentials, screenshots, logs, or personal
data in a public issue. Include the affected version and operating system,
reproduction steps, impact, and the smallest redacted diagnostic sample that
shows the problem. If private reporting is unavailable, contact the repository
maintainers through GitHub before sharing technical details.

Maintainers will acknowledge reports when reviewed, reproduce the issue,
assess affected versions, and coordinate a fix and disclosure. Response times
depend on maintainer availability.

## Security model

Anti-Interview is a local desktop application. It has no project-operated
backend, account system, password store, or telemetry service.

The main trust boundaries are:

- Windows DLL injection and process hooks, which modify other local processes.
- Experimental Windows cursor hiding, which temporarily replaces system cursors
  and can affect other applications until restored.
- Windows screen capture and clipboard integration.
- The Linux Hyprland helper, which changes managed window rules.
- Embedded or external web content opened for third-party AI providers.
- Local configuration, logs, temporary screenshots, DLLs, and release files.

Only enable process hooks when needed. Features that inject DLLs or manipulate
system processes may require administrator privileges. Run elevated for those
operations only, keep the installation directory writable only by trusted
users, and use DLLs built from this repository or an authenticated release.

External AI providers and remote banner URLs operate under their own security
policies. Do not enter secrets into third-party pages unless you trust that
provider and its local WebView storage.

## Sensitive local data

`anti-interview.log`, the local configuration, clipboard contents, and
`anti_interview_<timestamp>.png` temporary screenshots can expose window
titles, process names, paths, URLs, or screen content. Restrict access to the
user profile and working directory, redact diagnostics before sharing, and
remove files when no longer needed. See [PRIVACY.md](PRIVACY.md).

## Dependency and build checks

The repository commits `Cargo.lock`, pins its Rust toolchain, runs compiler,
test, clippy, format, and release gates in CI, and audits the lockfile with
RustSec. Contributors should run the checks in
[CONTRIBUTING.md](CONTRIBUTING.md) before submitting a change.
