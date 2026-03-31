pub mod ai_browser;
pub mod app_state;
pub mod gui;
pub mod video;

#[cfg(test)]
mod ai_browser_tests;
#[cfg(test)]
mod app_state_tests;
#[cfg(test)]
mod clipboard_tests;
#[cfg(test)]
mod silent_screenshot_tests;

pub use ai_browser::{AiBrowserState, AiTab};
pub use app_state::AppState;
pub use gui::start;
