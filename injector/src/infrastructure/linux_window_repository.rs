//! Linux/Hyprland implementation of `WindowRepository`.
//!
//! The GUI's hide toggle flows exactly as on Windows —
//! `AppState::toggle_window_visibility` → `WindowService::toggle_visibility` →
//! `WindowRepository::set_visibility`. Here that call drives Hyprland's
//! `no_screen_share` window rule through the tested `anti-interview-wayland.sh`
//! engine (see `linux/anti-interview-wayland.sh`), which rewrites a managed Lua
//! include and `hyprctl reload`s. Hiding is by window *class* (that's the
//! granularity Hyprland's rule works at), matching the Windows "hide the app"
//! behavior.

use crate::application::WindowRepository;
use crate::domain::{ProcessId, Window, WindowId};
use anyhow::{anyhow, Context, Result};
use serde_json::Value;
use std::process::Command;

pub struct LinuxWindowRepository {
    /// Path/name of the engine script (env `ANTI_INTERVIEW_WAYLAND_SH`, else PATH).
    script: String,
}

impl LinuxWindowRepository {
    pub fn new() -> Self {
        let script = std::env::var("ANTI_INTERVIEW_WAYLAND_SH")
            .unwrap_or_else(|_| "anti-interview-wayland.sh".to_string());
        Self { script }
    }

    fn hyprctl_clients(&self) -> Result<Vec<Value>> {
        let out = Command::new("hyprctl")
            .args(["clients", "-j"])
            .output()
            .context("running `hyprctl clients -j` (is Hyprland running?)")?;
        if !out.status.success() {
            return Err(anyhow!("hyprctl clients failed: {}", String::from_utf8_lossy(&out.stderr)));
        }
        let v: Value = serde_json::from_slice(&out.stdout).context("parsing hyprctl JSON")?;
        Ok(v.as_array().cloned().unwrap_or_default())
    }

    /// Classes currently hidden, per the engine's managed file.
    fn hidden_classes(&self) -> Vec<String> {
        Command::new(&self.script)
            .arg("classes")
            .output()
            .ok()
            .filter(|o| o.status.success())
            .map(|o| {
                String::from_utf8_lossy(&o.stdout)
                    .lines()
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect()
            })
            .unwrap_or_default()
    }

    fn class_for_pid(&self, pid: u32) -> Option<String> {
        self.hyprctl_clients().ok()?.into_iter().find_map(|c| {
            (c.get("pid").and_then(Value::as_u64) == Some(pid as u64))
                .then(|| c.get("class").and_then(Value::as_str).map(String::from))
                .flatten()
        })
    }

    fn run_engine(&self, args: &[&str]) -> Result<()> {
        let out = Command::new(&self.script)
            .args(args)
            .output()
            .with_context(|| format!("running `{} {}`", self.script, args.join(" ")))?;
        if !out.status.success() {
            return Err(anyhow!(
                "engine `{}` failed: {}",
                args.join(" "),
                String::from_utf8_lossy(&out.stderr)
            ));
        }
        Ok(())
    }
}

impl Default for LinuxWindowRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl WindowRepository for LinuxWindowRepository {
    fn find_all(&self) -> Result<Vec<Window>> {
        let hidden = self.hidden_classes();
        let clients = self.hyprctl_clients()?;
        let mut windows = Vec::with_capacity(clients.len());
        for c in clients {
            let class = c.get("class").and_then(Value::as_str).unwrap_or("").to_string();
            if class.is_empty() {
                continue;
            }
            let title = c.get("title").and_then(Value::as_str).unwrap_or("").to_string();
            let pid = c.get("pid").and_then(Value::as_u64).unwrap_or(0) as u32;
            // Hyprland addresses are 64-bit hex; fold to u32 for WindowId identity.
            let id = c
                .get("address")
                .and_then(Value::as_str)
                .and_then(|a| u64::from_str_radix(a.trim_start_matches("0x"), 16).ok())
                .map(|a| a as u32)
                .unwrap_or(pid);
            let is_hidden = hidden.iter().any(|h| h == &class);
            windows.push(Window::new(
                WindowId::new(id),
                title,
                ProcessId::new(pid),
                class, // process_name carries the Hyprland class on Linux
                is_hidden,
            ));
        }
        Ok(windows)
    }

    fn get_icon(&self, _window_id: &WindowId) -> Option<(usize, usize, Vec<u8>)> {
        // Hyprland exposes no per-window icon buffer; the GUI renders a placeholder.
        None
    }

    fn set_visibility(
        &self,
        process_id: &ProcessId,
        _window_id: &WindowId,
        hidden: bool,
        _hide_from_taskbar: Option<bool>,
    ) -> Result<()> {
        let class = self
            .class_for_pid(process_id.value())
            .ok_or_else(|| anyhow!("no Hyprland window found for pid {}", process_id.value()))?;
        self.run_engine(&[if hidden { "on" } else { "off" }, &class])
    }

    fn set_process_stealth(&self, _process_id: &ProcessId) -> Result<()> {
        // Task-manager hiding on Linux is a separate (LD_PRELOAD) concern, not wired here.
        Err(anyhow!("process stealth is not supported on Linux yet"))
    }

    fn inject_hook_dll(&self, _dll_name: &str) -> Result<()> {
        Err(anyhow!("DLL hook injection is Windows-only"))
    }
}
