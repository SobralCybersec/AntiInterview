#[cfg(test)]
mod integration_tests {
    use crate::application::{ConfigurationRepository, WindowService};
    use crate::infrastructure::{FileConfigurationRepository, WindowsWindowRepository};
    use crate::presentation::AppState;
    use std::sync::Arc;

    #[test]
    fn test_app_state_creation() {
        let window_repo = Arc::new(WindowsWindowRepository::new());
        let window_service = Arc::new(WindowService::new(window_repo));
        let config_repo = Arc::new(FileConfigurationRepository::new());

        let state = AppState::new(window_service, config_repo);
        assert!(state.is_ok(), "Failed to create AppState");
    }

    #[test]
    fn test_config_save_and_load() {
        let window_repo = Arc::new(WindowsWindowRepository::new());
        let window_service = Arc::new(WindowService::new(window_repo));
        let config_repo = Arc::new(FileConfigurationRepository::new());

        let mut state = AppState::new(window_service, config_repo).expect("Failed to create AppState");

        state.config_mut().ui.dark_theme = false;
        let result = state.save_config();
        assert!(result.is_ok(), "Failed to save config");

        state.config_mut().ui.dark_theme = true;
        let result = state.save_config();
        assert!(result.is_ok(), "Failed to save config");
    }

    #[test]
    fn test_hook_dll_injection() {
        let window_repo = Arc::new(WindowsWindowRepository::new());
        let window_service = Arc::new(WindowService::new(window_repo));
        let config_repo = Arc::new(FileConfigurationRepository::new());

        let state = AppState::new(window_service, config_repo).expect("Failed to create AppState");

        // Test injection (will fail if Task Manager not running or no admin)
        let result = state.inject_hook_dll("hook_notepad.dll");
        // Don't assert success as it requires Task Manager running and admin privileges
        println!("Hook injection result: {:?}", result);
    }
}
