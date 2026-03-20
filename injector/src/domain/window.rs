use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WindowId(u32);

impl WindowId {
    pub fn new(id: u32) -> Self {
        Self(id)
    }

    pub fn value(&self) -> u32 {
        self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProcessId(u32);

impl ProcessId {
    pub fn new(id: u32) -> Self {
        Self(id)
    }

    pub fn value(&self) -> u32 {
        self.0
    }

}

impl fmt::Display for ProcessId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}


#[derive(Debug, Clone)]
pub struct Window {
    id: WindowId,
    title: String,
    process_id: ProcessId,
    process_name: String,
    hidden: bool,
}

impl Window {
    pub fn new(id: WindowId, title: String, process_id: ProcessId, process_name: String, hidden: bool) -> Self {
        Self {
            id,
            title,
            process_id,
            process_name,
            hidden,
        }
    }

    pub fn id(&self) -> &WindowId {
        &self.id
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn process_id(&self) -> &ProcessId {
        &self.process_id
    }

    pub fn process_name(&self) -> &str {
        &self.process_name
    }

    pub fn is_hidden(&self) -> bool {
        self.hidden
    }

    pub fn set_hidden(&mut self, hidden: bool) {
        self.hidden = hidden;
    }

    pub fn matches_filter(&self, filter: &str) -> bool {
        if filter.is_empty() {
            return true;
        }
        let filter_lower = filter.to_lowercase();
        self.title.to_lowercase().contains(&filter_lower) ||
        self.process_name.to_lowercase().contains(&filter_lower)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_window_id_creation() {
        let id = WindowId::new(12345);
        assert_eq!(id.value(), 12345);
    }

    #[test]
    fn test_process_id_creation() {
        let pid = ProcessId::new(9876);
        assert_eq!(pid.value(), 9876);
    }

    #[test]
    fn test_window_creation() {
        let window = Window::new(
            WindowId::new(1),
            "Test Window".to_string(),
            ProcessId::new(100),
            "test.exe".to_string(),
            false,
        );
        assert_eq!(window.id().value(), 1);
        assert_eq!(window.title(), "Test Window");
        assert_eq!(window.process_id().value(), 100);
        assert_eq!(window.process_name(), "test.exe");
        assert!(!window.is_hidden());
    }

    #[test]
    fn test_window_set_hidden() {
        let mut window = Window::new(
            WindowId::new(1),
            "Test".to_string(),
            ProcessId::new(100),
            "test.exe".to_string(),
            false,
        );
        assert!(!window.is_hidden());
        window.set_hidden(true);
        assert!(window.is_hidden());
    }

    #[test]
    fn test_window_matches_filter_empty() {
        let window = Window::new(
            WindowId::new(1),
            "Chrome Browser".to_string(),
            ProcessId::new(100),
            "chrome.exe".to_string(),
            false,
        );
        assert!(window.matches_filter(""));
    }

    #[test]
    fn test_window_matches_filter_case_insensitive() {
        let window = Window::new(
            WindowId::new(1),
            "Chrome Browser".to_string(),
            ProcessId::new(100),
            "chrome.exe".to_string(),
            false,
        );
        assert!(window.matches_filter("chrome"));
        assert!(window.matches_filter("CHROME"));
        assert!(window.matches_filter("browser"));
    }

    #[test]
    fn test_window_matches_filter_no_match() {
        let window = Window::new(
            WindowId::new(1),
            "Chrome Browser".to_string(),
            ProcessId::new(100),
            "chrome.exe".to_string(),
            false,
        );
        assert!(!window.matches_filter("firefox"));
    }

    #[test]
    fn test_window_id_equality() {
        let id1 = WindowId::new(123);
        let id2 = WindowId::new(123);
        let id3 = WindowId::new(456);
        assert_eq!(id1, id2);
        assert_ne!(id1, id3);
    }
}
