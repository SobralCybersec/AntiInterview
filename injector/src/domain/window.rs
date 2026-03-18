use serde::{Deserialize, Serialize};

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

#[derive(Debug, Clone)]
pub struct Window {
    id: WindowId,
    title: String,
    process_id: ProcessId,
    hidden: bool,
}

impl Window {
    pub fn new(id: WindowId, title: String, process_id: ProcessId, hidden: bool) -> Self {
        Self {
            id,
            title,
            process_id,
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
        self.title.to_lowercase().contains(&filter.to_lowercase())
    }
}
