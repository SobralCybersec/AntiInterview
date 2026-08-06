# Current task: animated egui interface redesign and responsiveness hardening

## Current goal

Ship a polished neutral-black egui interface with a persistent, usable sidebar,
purposeful motion, responsive input, and preserved window-management behavior.

## Files touched

- Tooling/contracts: `.clang-format`, `.gitattributes`, `.github/workflows/ci.yml`, `.gitignore`, `Cargo.lock`, `Cargo.toml`, `README.md`, `rust-toolchain.toml`
- Manifests: `injector/Cargo.toml`, `payload/Cargo.toml`
- Performance paths: `injector/src/application/window_service.rs`, `injector/src/domain/window.rs`, `injector/src/infrastructure/process_monitor.rs`, `injector/src/infrastructure/screen_capture.rs`, `injector/src/infrastructure/windows_window_repository.rs`, `injector/src/presentation/app_state.rs`, `injector/src/presentation/video/video_renderer.rs`
- Build/test fixes and formatting: scoped Rust sources under `injector/src` and `payload/`, plus C++ files under `hook/`
- Policy/docs: `SECURITY.md`, `PRIVACY.md`, `CONTRIBUTING.md`, `docs/ARCHITECTURE.md`, `docs/DEPENDENCIES.md`, `LICENSE`, `release/LICENSE`
- Checklist pass: `PROGRESS.md`, `README.md`, `config.example.toml`, `.github/workflows/ci.yml`
- Cleanup/security: `Cargo.lock`, `injector/Cargo.toml`, configuration schema test, sensitive-path logging, orphaned unused Rust files, and Bash-invoked LF-enforced Linux scripts
- Release automation: `.github/workflows/release.yml`, `hook/CMakeLists.txt`, `installer.iss`, `PROGRESS.md`
- UI redesign and responsiveness: `injector/src/presentation/gui.rs`,
  `injector/src/presentation/ai_browser.rs`, `PROGRESS.md`

## Decisions made

- Keep UI/webview stack migration separate; paired upgrades need Windows regression checks.
- Gate Windows-only payload code/dependency on Windows so Linux workspace checks can run.
- Pin `nightly-2026-08-04`; `dll-syringe 0.18.2` still uses nightly features.
- Add `profiling` Cargo profile with debug symbols; no profiler dependency added.
- Skip ESLint/Prettier: repository contains no JavaScript/TypeScript project or `package.json`.
- Use `[=]` for evaluated non-applicable items; do not add databases, APIs,
  metrics, Node tooling, or design patterns to satisfy a generic checklist.
- Preserve the MIT text; update both distributed copies to
  `2021-2026 radiantly and contributors`.
- Use GitHub private vulnerability reporting and RustSec lockfile auditing.
- Trigger releases only from existing `v*` tags whose version matches both the
  `anti-interview` Cargo package and Inno Setup metadata.
- Build Rust executables, payload DLL, and Detours hook DLLs from source; never
  upload the tracked legacy archive from `release/`.
- Publish portable ZIP, installer, and `SHA256SUMS.txt` through GitHub CLI using
  the workflow token and least-required `contents: write` permission.
- Keep the redesign in `gui.rs`; reuse egui 0.33 animation, painter, frame, font,
  and accessibility behavior without adding a dependency.
- Use a privacy control-room direction: deep ink surfaces, arctic teal signal,
  editorial split hero, layered control cards, and 160-280 ms motion.
- Preserve real checkboxes, hotkeys, tray behavior, capture throttling, and every
  existing call into `AppState`.
- Use neutral black/charcoal surfaces and white primary accents; green remains
  semantic success feedback, not the application theme.
- Start with a 252 px expanded sidebar, retain an 84 px collapsed state, and
  keep its toggle at the top so short windows cannot hide it.
- Replace AI Browser UI-thread sleeps with a two-stage repaint timer that keeps
  the existing 1.5 s refresh and 0.5 s hide sequence.
- RustProxyHub's Linux WebKit environment guards are Tauri-specific and do not
  apply to this eframe/winit app; its reusable non-blocking/coalescing pattern
  matches the frame-driven timer used here.

## Verified checks

- [x] Exa dependency/API/performance research completed.
- [x] CodeGraph architecture and hot-path map completed.
- [x] Baseline `cargo fmt`, `check`, `test`, `clippy`, and release build captured; all blocked by committed merge markers.
- [x] CMake baseline captured; Linux configure stops at missing Windows Detours library.
- [x] `cargo fmt --all -- --check` passed.
- [x] `cargo check --workspace --all-targets` passed.
- [x] `cargo test --workspace` passed: 96 tests across 5 suites.
- [x] `cargo clippy --workspace --all-targets -- -D warnings` passed.
- [x] Windows GNU target check and clippy passed.
- [x] Release and profiling builds passed; profiling binary retains debug info.
- [x] C++ `clang-format --dry-run --Werror` passed.
- [x] Rollback restored original SHA-256 and original manifest parse failure on separate copy.
- [x] RustSec audit identified and lockfile updates fixed both actionable vulnerabilities.
- [x] Direct dependencies mapped and documented; unused `clap` and `libloading` removed.
- [x] Orphaned NT-hook prototype modules and tests removed; supported hook paths remain.
- [x] Security, privacy, architecture, dependency, contribution, configuration, and license documents added or updated.
- [x] Full Rust, C++ format, Linux self-test, cross-target, audit, docs, and rollback validation passed.
- [x] Official GitHub Release, GitHub CLI, runner-image, and vcpkg guidance reviewed.
- [x] Baseline confirmed no release workflow existed.
- [x] PyYAML parsing and release contract assertions passed: version gate,
  seven DLL sets, fresh asset paths, checksums, and create/upload branches.
- [x] CMake Release configuration passed with all seven hook targets present.
- [x] `cargo fmt --all -- --check` and `cargo test --workspace --locked`
  passed; 96 tests passed across five suites.
- [x] Transaction rollback passed on a separate copy; original hashes and local
  Debug-path installer behavior were restored while `MODIFIED_FILE` stayed changed.
- [x] Independent risk review found no release-automation issue; independent
  verification confirmed version, assets, CMake targets, and upload branches.
- [x] CodeGraph mapped the egui shell, render flow, AppState calls, and blast radius.
- [x] Exa research verified egui 0.33 animation, Frame, repaint, and performance APIs.
- [x] UI Pro Max design-system and motion/accessibility searches completed.
- [x] Baseline fmt/check/tests passed: 96 tests across five suites.
- [x] Baseline UI screenshot captured at 941x1030.
- [x] Implement and visually validate the animated egui redesign; user accepted
  the neutral-black result during the live smoke run.
- [x] Remove presentation-path `thread::sleep` calls; static check reports
  `no UI-thread sleeps`.
- [x] Modified fmt/check/tests/clippy passed; 96 tests passed across five suites.
- [x] Test UI transaction rollback on a separate copy; all three source hashes
  matched and the restored workspace passed 96 tests across five suites.
- [x] Complete independent UI risk and verification review.

## Remaining work

- [x] Produce dependency diagnosis and deferred upgrade plan.
- [ ] Migrate the eframe/winit/GTK dependency stack when compatible releases clear recorded future-incompatibility and RustSec warnings.
- [x] Track the current no-fix RustSec warning for `anyhow 1.0.102`; no vulnerable call to `downcast_mut` exists in this repository.
- [ ] Run native Windows Detours C++ build; Linux host lacks required Detours library.
- [ ] Push a matching version tag to exercise the Windows-hosted release workflow.
- [ ] Validate the redesigned UI on native Windows after local Linux visual smoke.

---

Here's the checklist I'd use as a **gold standard** for most modern Rust/TypeScript desktop or backend projects. It intentionally focuses on maintainability, correctness, performance, and simplicity—not on applying patterns for their own sake.

---

# Universal Engineering Checklist

> **Constraint:** Never break working behavior. Every implemented item is backed
> by repository evidence or validation. `[=]` records an evaluated,
> non-applicable item so the checklist does not create speculative systems.

## Legend

* `[x]` Implemented and verified
* `[=]` Evaluated; no change warranted
* `[~]` In progress
* `[ ]` Not started
* `✅` Already satisfied

## Audit

Date: 2026-08-06

Evidence: `Cargo.toml`, `Cargo.lock`, crate manifests, CodeGraph call paths,
RustSec audit, compiler/clippy/tests, CI, `SECURITY.md`, `PRIVACY.md`,
`CONTRIBUTING.md`, `docs/ARCHITECTURE.md`, and
`docs/DEPENDENCIES.md`.

# 1. Dependency Policy

* [x] Every dependency has a documented purpose. See `docs/DEPENDENCIES.md`.
* [x] Prefer `std` before external crates. Direct dependencies were reviewed.
* [x] Remove duplicate functionality. No overlapping direct dependency remained.
* [x] Remove unused dependencies. Removed `clap` and `libloading`.
* [x] Keep optional features behind feature flags. Platform dependencies use target `cfg`; codec features are limited.
* [x] Avoid dependency bloat. Removed unused direct crates and disabled unnecessary image defaults.
* [x] Review dependencies periodically. RustSec runs locally and in CI; review procedure is documented.
* [x] Keep lockfiles committed. `Cargo.lock` is tracked by policy.

# 2. Code Style

* [x] `cargo fmt` / formatter enforced in CI.
* [x] `clippy -D warnings` (or language equivalent).
* [=] Eliminate risky `unwrap()`/`expect()` outside acceptable locations. Remaining uses are tests, startup fail-fast paths, or poisoned-lock invariants.
* [x] Prefer immutable variables. Compiler and clippy review passed.
* [x] Minimize mutable state. Mutation stays in UI state, capture buffers, and synchronized workers.
* [x] Clear naming. Code and review gates passed.
* [x] Remove dead imports. Clippy passes with warnings denied.
* [=] Keep functions small. Large presentation functions remain cohesive; splitting them now adds regression risk without behavior gain.
* [x] Comments explain *why*, not *what*. Platform and safety comments document constraints.
* [x] Consistent formatting. Rustfmt and clang-format pass.
* [=] Avoid magic numbers. Win32 constants, UI dimensions, and documented timing thresholds remain local and domain-specific.
* [x] Prefer explicit types at boundaries. Repository traits, commands, configuration, and FFI signatures are typed.

# 3. Error Handling

* [=] Typed domain errors. This local application uses contextual `anyhow`; it has no public error contract that benefits from a new hierarchy.
* [x] Avoid opaque "string errors". Boundary failures carry operation context and diagnostics.
* [x] Propagate errors properly. Fallible repository, configuration, capture, and FFI operations return `Result`.
* [x] Recover where appropriate. Optional tray, monitor, icon, and clipboard failures degrade without corrupting state.
* [=] User-facing errors are friendly. Existing desktop behavior logs technical failures; adding a notification system is outside this pass.
* [x] Internal logs contain useful diagnostics. `tracing` records operations and contextual errors.

# 4. Architecture

## Modules

* [x] Single Responsibility. Domain, application, infrastructure, and presentation own distinct work.
* [x] High cohesion. Platform implementations live with their repositories and adapters.
* [x] Low coupling. Application traits isolate UI and platform code.
* [x] Feature-oriented organization where beneficial. Capture, browser, tray, video, hooks, and configuration have focused modules.
* [=] Remove god modules. `gui.rs` is large but presentation-only; a mechanical split would not reduce runtime coupling.
* [x] Avoid cyclic dependencies. Cargo and module compilation pass.

## Composition

* [x] Prefer composition. `AppState` composes services and adapters.
* [x] Inject dependencies. Repository traits enter `WindowService` and `AppState`.
* [x] Keep composition root centralized. `injector/src/main.rs` wires the app.
* [x] Avoid unnecessary globals. Globals remain only where DLL ABI/state constraints require them.

# 5. Design Patterns

Only where justified.

## NewType

* [x] IDs. `WindowId` and `ProcessId` wrap raw values.
* [x] Domain values. Window and configuration types own behavior.
* [x] Prevent parameter mix-ups. Distinct ID types protect service boundaries.

## Builder

* [=] Complex configuration. Serde defaults and direct structs are simpler.
* [=] Many optional parameters. Current constructors stay readable.

## Factory

* [=] Multiple runtime implementations. The composition root selects repositories with platform `cfg`.

## RAII

* [x] Resource cleanup. Rust ownership and native wrapper drops own handles.
* [=] Temporary files. User-requested screenshots intentionally persist; retention is documented in `PRIVACY.md`.
* [x] Locks. Mutex guards release through RAII.
* [x] Transactions. Microsoft Detours changes use its transaction API.

## TypeState

* [=] Compile-time state safety where runtime isn't required. No multi-stage builder or protocol warrants TypeState.

Evaluate each pattern:

* [=] Builder. Not needed.
* [=] Factory. Platform composition is sufficient.
* [x] Strategy. `WindowRepository` supplies platform strategies.
* [x] Observer. Typed channels deliver app, capture, and tray events.
* [x] State. `AppState` and `AiBrowserState` own explicit runtime state.
* [=] TypeState. Not needed.
* [=] Visitor. Not needed.
* [x] Adapter. Infrastructure adapts Win32, Hyprland, filesystem, clipboard, and WebView APIs.

# 6. Database

* [=] Migrations. No database.
* [=] Prepared statements. No database.
* [=] Transactions. No database.
* [=] Full Text Search where useful. No database or search index.
* [=] Indexes. No database.
* [=] No N+1 queries. No database.
* [=] Query arguments grouped. No database.
* [=] Connection management. No database.
* [=] Data validation. Database validation is not applicable; config and FFI boundaries are validated.
* [=] Backups/migration strategy. No database; serde defaults preserve config compatibility.

# 7. API / IPC

* [x] Typed DTOs. Rust commands, domain types, configuration, and FFI signatures are typed.
* [=] Versioned contracts if public. No public network API; payload ABI changes require coordinated builds.
* [x] Stable payloads. Commands and exported Win32 signatures use explicit types.
* [=] No `any`. Rust/C++ codebase has no TypeScript `any`.
* [x] Boundary validation. Config parsing, FFI pointers/sizes, subprocess status, and platform results are checked.
* [x] Serialization consistency. Serde/TOML owns configuration round trips and defaults.

# 8. Real-Time

Evaluate:

* [=] SSE. No client/server stream.
* [=] WebSockets. No project backend.
* [x] Polling. Process monitoring uses bounded-frequency local polling.
* [x] Background workers. Capture, commands, process monitoring, and browser wait run off the UI thread.
* [x] Event bus. Typed channels provide the required local event flow.

The project uses the smallest local mechanism for each workflow.

# 9. Concurrency

* [x] Minimize shared mutable state. Shared state is confined behind `Arc<Mutex<_>>`.
* [x] Prefer ownership. Channels transfer commands and frames.
* [=] Avoid blocking async runtimes. No async runtime exists; blocking work uses native threads.
* [x] Cancellation support. Stop commands, flags, and channel disconnects terminate work.
* [x] Graceful shutdown. Capture handles stop on replacement/drop and `ProcessMonitor::drop` clears its run flag.
* [x] Thread safety. Rust compiler checks plus concurrency tests pass.

# 10. Observability

## Logging

* [x] Structured logs. `tracing` with module targets and levels is configured.
* [x] Capability targets. Tracing callsites retain module targets.
* [=] Correlation IDs. No concurrent request/transaction boundary needs them.
* [=] Request IDs. No server requests.

## Tracing

* [=] Spans. Current local operations are short and event logs suffice.
* [x] Performance timing. Profiling profile retains optimized code and debug symbols.
* [=] Context propagation. No distributed or async request context.

## Metrics

* [=] Errors. Local logs cover errors; no metrics backend exists.
* [=] Latency. Profiling is workload-driven; no service SLO exists.
* [=] Throughput. No server workload.
* [=] Queue sizes. Frame queue is statically bounded at two.

# 11. Security

* [x] Secrets never committed. Scoped secret-pattern scan found no credential material.
* [=] Keychain/Keyring where appropriate. The app stores no credentials.
* [x] Validate input. Config, subprocess, FFI, window, and image boundaries validate inputs/results.
* [x] Escape output. External commands receive separate arguments; UI renders text without shell interpolation.
* [x] Principle of least privilege. Elevated hook operations are optional and documented.
* [x] Secure defaults. Hooks, clipboard copy, preview, and Task Manager hiding default off.
* [x] Avoid leaking sensitive data. Sensitive paths/details use debug logging; disclosure and local-data handling are documented.
* [=] Hash passwords. No passwords.
* [=] Encrypt sensitive storage if needed. Config stores preferences, not secrets; screenshots/logs follow local OS access controls.

# 12. Performance

* [x] Measure before optimizing. Allocation/repaint/channel hot paths were traced before changes.
* [=] Benchmark hot paths. Desktop capture lacks a deterministic benchmark fixture; use the profiling workload documented for the app.
* [x] Profile. `profile.profiling` builds optimized binaries with debug symbols.
* [x] Cache only when justified. Icons/textures and reusable capture storage avoid repeat decode/allocation.
* [x] Lazy initialization. Icons, capture, tray, browser, and configuration resources initialize on demand or by feature.
* [x] Avoid unnecessary allocations. Capture buffers and window filtering reuse existing storage.

# 13. Frontend

* [x] Typed state. Rust structs and enums own UI state.
* [=] Typed IPC. No web frontend IPC; local commands are typed.
* [=] No `any`. No JavaScript/TypeScript.
* [x] Remove dead components. Compiler/clippy plus orphan cleanup found no compiled dead component.
* [=] Remove dead hooks. No React-style hooks.
* [x] Feature organization where useful. Browser, video, state, and GUI concerns have modules.
* [x] Keep components presentational. `gui.rs` delegates behavior to `AppState` and services.
* [x] Business logic in hooks/stores. Rust state/services own business behavior; framework hooks do not apply.
* [=] Accessibility. Existing labels, tooltips, keyboard shortcuts, and native egui semantics remain; no redesign requested.
* [=] Responsive layouts. Desktop layout behavior is preserved; no mobile/web viewport exists.
* [x] Loading states. Browser and capture readiness are represented.
* [=] Error states. Technical failures use structured logs; adding modal UX is outside this pass.
* [x] Empty states. Missing icons, monitors, windows, screenshots, and capture frames degrade explicitly.

# 14. JavaScript / Node

* [=] Remove dead scripts. No JavaScript/Node project.
* [=] Shared helpers. No JavaScript/Node project.
* [=] Consistent protocol. No JavaScript/Node project.
* [=] Typed payloads. No JavaScript/Node project.
* [=] Consistent errors. No JavaScript/Node project.

# 15. Testing

## Unit

* [x] Domain logic. Window/configuration tests pass.
* [x] Helpers. Video, browser, clipboard, screenshot, and hook helpers have scoped tests.
* [x] Parsers. TOML/configuration and platform data parsing paths are covered where deterministic.

## Integration

* [=] Database. No database.
* [=] API. No project backend API.
* [x] IPC. Payload ABI and app command integration tests cover internal boundaries.

## End-to-End

* [=] Main workflows. Windows capture/hook UI workflows require a native interactive smoke test; release instructions cover it.
* [=] Browser automation. Third-party AI pages are outside project control.
* [=] UI interactions. No stable headless egui harness is present; state and behavior are unit tested.

## Regression

* [x] Every bug fixed gets a regression test. This pass added schema coverage and retained prior regression tests.

# 16. CI/CD

* [x] Build.
* [x] Test.
* [x] Lint.
* [x] Format check.
* [x] Type check.
* [x] Release build.
* [x] Cache dependencies.
* [x] Multiple platforms if applicable. Ubuntu and Windows run in the matrix.

# 17. Documentation

* [x] README.
* [x] Setup guide. See `CONTRIBUTING.md` and release user guide.
* [x] Architecture overview. See `docs/ARCHITECTURE.md`.
* [x] Module responsibilities. See `docs/ARCHITECTURE.md`.
* [x] Event documentation. Typed command and worker flows are documented.
* [x] Configuration. `config.example.toml` matches the schema and has a regression test.
* [x] Feature flags. Platform capability gates and Cargo features are documented.
* [x] Contribution guide. See `CONTRIBUTING.md`.

# 18. Principles Review

* [x] DRY. Hook selection and shared behavior have one owner.
* [x] KISS. Local channels, files, and platform adapters remain direct.
* [x] YAGNI. Database, backend, metrics, and speculative patterns were not added.
* [x] SOLID. Repository contracts and composition isolate platform changes.
* [x] Law of Demeter. UI commands services instead of reaching through platform objects.
* [x] Fail Fast. Startup and contract failures stop with context.
* [x] Separation of Concerns. Four application layers and platform adapters remain distinct.
* [x] Principle of Least Astonishment. Defaults are conservative and documented.

# 19. Cleanup

* [x] Remove dead code. Removed orphaned NT-hook prototypes/tests and unused direct dependencies.
* [=] Remove unused assets. Tracked release/visual assets remain because native packaging and visual smoke were not part of this pass.
* [x] Remove obsolete TODOs. The orphaned placeholder carrying the stale TODO was removed.
* [x] Remove obsolete comments. Touched documentation and code comments match current behavior.
* [=] Remove obsolete feature flags. No obsolete custom Cargo feature was found.
* [x] Remove duplicate code. Hook-list construction and filtered-window handling use shared owners.
* [x] Remove unused configuration. Removed the unsupported example `renderer` key and synchronized all schema fields.

# 20. Release Verification

## Manual

* [=] Smoke test. No release requested; native Windows smoke remains a release gate.
* [=] Main workflows. Automated state tests pass; interactive Windows verification belongs to release execution.
* [=] Error handling. Compiler/tests exercise deterministic errors; platform UI failures require native smoke.
* [=] Settings. Schema/default/serialization tests pass; interactive settings smoke belongs to release execution.
* [=] Imports/exports. Windows ABI compilation is cross-checked; native DLL loading requires Windows.

## Builds

* [x] Debug. Workspace tests/checks build debug targets.
* [x] Release. Release build passes.
* [x] Cross-platform builds. Linux and Windows GNU checks pass; CI owns native Windows MSVC.

## Final

* [=] CI green. Local CI-equivalent commands pass; remote workflow result requires a pushed commit.
* [=] No warnings. Project warnings are denied; accepted transitive future-incompatibility and RustSec warnings are recorded above pending compatible upstream releases.
* [=] Version bumped. No release or behavior version change requested.
* [=] Changelog updated. This engineering pass is recorded here; no release changelog entry is warranted.
* [=] Tag release. No release requested.

# 21. Post-Release

* [=] Monitor logs. No release or hosted service in this pass.
* [=] Monitor crashes. No project crash-collection service; users report through GitHub.
* [=] Monitor performance. Profiling is performed against native workloads when needed.
* [=] Collect user feedback. Repository issue channels remain the feedback path.
* [x] Schedule dependency review. CI RustSec audit and the documented review procedure provide recurrence.

## Engineering Philosophy

Before implementing:

* [x] Does this solve a real problem? It closes policy, audit, documentation, dependency, and configuration gaps.
* [x] Is it simpler than the alternative? Documentation and existing gates replace speculative systems.
* [x] Is it measurable? Audit, compiler, test, format, build, and checklist commands return explicit results.
* [x] Can it be tested? Schema, code, build, security, and rollback checks cover the change.
* [x] Will it reduce future maintenance? Policies and dependency ownership remove rediscovery.
* [x] Does it preserve current behavior? Runtime changes remove only unused code/dependencies and reduce sensitive info-level logging.
* [x] Is there a standard-library solution? Existing std and Cargo mechanisms remain preferred.
* [x] Is the complexity justified? New files document existing contracts; no new runtime layer was added.
* [x] Can a new contributor understand it in a few minutes? README links to focused architecture, dependency, privacy, security, and contribution documents.
