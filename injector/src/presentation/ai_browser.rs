use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AiTab {
    ChatGpt,
    Antigravity,
    Gemini,
    Claude,
}

impl AiTab {
    pub fn url(&self) -> &'static str {
        match self {
            AiTab::ChatGpt => "https://chatgpt.com",
            AiTab::Antigravity => "https://antigravity.ai",
            AiTab::Gemini => "https://gemini.google.com",
            AiTab::Claude => "https://claude.ai",
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            AiTab::ChatGpt => "ChatGPT",
            AiTab::Antigravity => "Antigravity",
            AiTab::Gemini => "Gemini",
            AiTab::Claude => "Claude",
        }
    }

    pub fn all() -> &'static [AiTab] {
        &[AiTab::ChatGpt, AiTab::Antigravity, AiTab::Gemini, AiTab::Claude]
    }
}

pub struct AiBrowserState {
    pub active_tab: AiTab,
    pub last_screenshot_path: Option<String>,
    browser_open: Arc<Mutex<bool>>,
    on_browser_spawn: Option<Box<dyn Fn() + Send + Sync>>,
}

impl std::fmt::Debug for AiBrowserState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AiBrowserState")
            .field("active_tab", &self.active_tab)
            .field("last_screenshot_path", &self.last_screenshot_path)
            .field("browser_open", &self.browser_open)
            .finish()
    }
}

impl Default for AiBrowserState {
    fn default() -> Self {
        Self {
            active_tab: AiTab::ChatGpt,
            last_screenshot_path: None,
            browser_open: Arc::new(Mutex::new(false)),
            on_browser_spawn: None,
        }
    }
}

impl AiBrowserState {
    pub fn set_on_browser_spawn<F>(&mut self, callback: F)
    where
        F: Fn() + Send + Sync + 'static,
    {
        self.on_browser_spawn = Some(Box::new(callback));
    }

    pub fn select_tab(&mut self, tab: AiTab) {
        self.active_tab = tab;
    }

    pub fn set_screenshot_path(&mut self, path: String) {
        self.last_screenshot_path = Some(path);
    }

    pub fn clear_screenshot_path(&mut self) {
        self.last_screenshot_path = None;
    }

    pub fn open_browser_window(&mut self) {
        let url = self.active_tab.url().to_string();
        let title = format!("AI Browser - {} - Anti-Interview", self.active_tab.label());
        
        tracing::info!("=== OPENING AI BROWSER WINDOW ===");
        tracing::info!("Target URL: {}", url);
        tracing::info!("Active Tab: {:?}", self.active_tab);
        tracing::info!("Window Title: {}", title);
        
        let exe_path = std::env::current_exe()
            .ok()
            .and_then(|p| p.parent().map(|p| p.to_path_buf()))
            .unwrap_or_else(|| std::path::PathBuf::from("."));
        
        let browser_exe = exe_path.join("ai_browser_window.exe");
        
        tracing::info!("Browser executable: {:?}", browser_exe);
        
        match std::process::Command::new(&browser_exe)
            .arg(&url)
            .arg(&title)
            .spawn()
        {
            Ok(mut child) => {
                let browser_open = Arc::clone(&self.browser_open);
                *browser_open.lock().unwrap() = true;
                tracing::info!("Browser window spawned successfully with PID: {}", child.id());
                
                std::thread::spawn(move || {
                    match child.wait() {
                        Ok(status) => {
                            tracing::info!("Browser window closed with status: {:?}", status);
                            *browser_open.lock().unwrap() = false;
                        }
                        Err(e) => {
                            tracing::error!("Failed to wait for browser process: {:?}", e);
                            *browser_open.lock().unwrap() = false;
                        }
                    }
                });
                
                std::thread::sleep(std::time::Duration::from_millis(500));
                
                if let Some(on_spawn) = &self.on_browser_spawn {
                    on_spawn();
                }
            }
            Err(e) => tracing::error!("Failed to spawn browser window: {:?}", e),
        }
    }

    pub fn is_browser_open(&self) -> bool {
        *self.browser_open.lock().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ai_tab_urls() {
        assert_eq!(AiTab::ChatGpt.url(), "https://chatgpt.com");
        assert_eq!(AiTab::Antigravity.url(), "https://antigravity.ai");
        assert_eq!(AiTab::Gemini.url(), "https://gemini.google.com");
        assert_eq!(AiTab::Claude.url(), "https://claude.ai");
    }

    #[test]
    fn test_ai_tab_labels() {
        assert_eq!(AiTab::ChatGpt.label(), "ChatGPT");
        assert_eq!(AiTab::Antigravity.label(), "Antigravity");
        assert_eq!(AiTab::Gemini.label(), "Gemini");
        assert_eq!(AiTab::Claude.label(), "Claude");
    }

    #[test]
    fn test_ai_tab_all_count() {
        assert_eq!(AiTab::all().len(), 4);
    }

    #[test]
    fn test_ai_browser_state_default_tab() {
        let state = AiBrowserState::default();
        assert_eq!(state.active_tab, AiTab::ChatGpt);
        assert!(state.last_screenshot_path.is_none());
        assert!(!state.is_browser_open());
    }

    #[test]
    fn test_ai_browser_state_tab_switching() {
        let mut state = AiBrowserState::default();
        state.active_tab = AiTab::Claude;
        assert_eq!(state.active_tab, AiTab::Claude);
        state.active_tab = AiTab::Gemini;
        assert_eq!(state.active_tab, AiTab::Gemini);
    }

    #[test]
    fn test_ai_browser_state_screenshot_path() {
        let mut state = AiBrowserState::default();
        state.set_screenshot_path("/tmp/screenshot.png".to_string());
        assert_eq!(state.last_screenshot_path.as_deref(), Some("/tmp/screenshot.png"));
        state.clear_screenshot_path();
        assert!(state.last_screenshot_path.is_none());
    }
}
