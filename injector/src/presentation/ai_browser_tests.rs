#[cfg(test)]
mod tests {
    use crate::domain::configuration::{Configuration, Hotkeys};
    use crate::presentation::{AiBrowserState, AiTab};

    #[test]
    fn test_hotkeys_has_all_bindings() {
        let hotkeys = Hotkeys::default();
        assert!(!hotkeys.screenshot.is_empty());
        assert!(!hotkeys.hide_window.is_empty());
        assert!(!hotkeys.show_gui.is_empty());
        assert!(!hotkeys.silent_screenshot.is_empty());
    }

    #[test]
    fn test_hotkeys_silent_screenshot_default() {
        let hotkeys = Hotkeys::default();
        assert_eq!(hotkeys.silent_screenshot, "Ctrl+Shift+P");
    }

    #[test]
    fn test_configuration_includes_silent_screenshot() {
        let config = Configuration::default();
        assert_eq!(config.hotkeys.silent_screenshot, "Ctrl+Shift+P");
    }

    #[test]
    fn test_ai_browser_state_initialization() {
        let state = AiBrowserState::default();
        assert_eq!(state.active_tab, AiTab::ChatGpt);
        assert!(state.last_screenshot_path.is_none());
        assert!(!state.is_browser_open());
    }

    #[test]
    fn test_ai_browser_tab_switching_all_tabs() {
        let mut state = AiBrowserState::default();
        
        for &tab in AiTab::all() {
            state.active_tab = tab;
            assert_eq!(state.active_tab, tab);
        }
    }

    #[test]
    fn test_ai_browser_screenshot_path_lifecycle() {
        let mut state = AiBrowserState::default();
        
        assert!(state.last_screenshot_path.is_none());
        
        state.set_screenshot_path("/tmp/test1.png".to_string());
        assert_eq!(state.last_screenshot_path.as_deref(), Some("/tmp/test1.png"));
        
        state.set_screenshot_path("/tmp/test2.png".to_string());
        assert_eq!(state.last_screenshot_path.as_deref(), Some("/tmp/test2.png"));
        
        state.clear_screenshot_path();
        assert!(state.last_screenshot_path.is_none());
    }

    #[test]
    fn test_all_ai_tabs_have_valid_urls() {
        for &tab in AiTab::all() {
            let url = tab.url();
            assert!(url.starts_with("https://"));
            assert!(url.len() > 10);
        }
    }

    #[test]
    fn test_all_ai_tabs_have_labels() {
        for &tab in AiTab::all() {
            let label = tab.label();
            assert!(!label.is_empty());
            assert!(label.len() > 2);
        }
    }

    #[test]
    fn test_ai_tab_count() {
        assert_eq!(AiTab::all().len(), 4);
    }

    #[test]
    fn test_ai_tab_urls_are_unique() {
        let urls: Vec<&str> = AiTab::all().iter().map(|t| t.url()).collect();
        let mut unique_urls = urls.clone();
        unique_urls.sort();
        unique_urls.dedup();
        assert_eq!(urls.len(), unique_urls.len());
    }

    #[test]
    fn test_hotkeys_serialization_includes_silent_screenshot() {
        let hotkeys = Hotkeys::default();
        let serialized = toml::to_string(&hotkeys).unwrap();
        assert!(serialized.contains("silent_screenshot"));
        assert!(serialized.contains("Ctrl+Shift+P"));
    }

    #[test]
    fn test_hotkeys_deserialization_with_silent_screenshot() {
        let toml_str = r#"
            screenshot = "Ctrl+Shift+S"
            hide_window = "Ctrl+Shift+H"
            show_gui = "Ctrl+Shift+I"
            silent_screenshot = "Ctrl+Shift+P"
        "#;
        let hotkeys: Hotkeys = toml::from_str(toml_str).unwrap();
        assert_eq!(hotkeys.silent_screenshot, "Ctrl+Shift+P");
    }

    #[test]
    fn test_hotkeys_deserialization_without_silent_screenshot_uses_default() {
        let toml_str = r#"
            screenshot = "Ctrl+Shift+S"
            hide_window = "Ctrl+Shift+H"
            show_gui = "Ctrl+Shift+I"
        "#;
        let hotkeys: Hotkeys = toml::from_str(toml_str).unwrap();
        assert_eq!(hotkeys.silent_screenshot, "Ctrl+Shift+P");
    }
}


#[cfg(test)]
mod hotkey_integration_tests {
    use crate::domain::configuration::Configuration;

    #[test]
    fn test_all_hotkeys_are_unique() {
        let config = Configuration::default();
        let hotkeys = vec![
            &config.hotkeys.screenshot,
            &config.hotkeys.hide_window,
            &config.hotkeys.show_gui,
            &config.hotkeys.silent_screenshot,
            &config.hotkeys.refresh_windows,
            &config.hotkeys.toggle_preview,
            &config.hotkeys.open_ai_browser,
        ];
        
        for i in 0..hotkeys.len() {
            for j in (i + 1)..hotkeys.len() {
                assert_ne!(
                    hotkeys[i], hotkeys[j],
                    "Hotkey conflict detected: {} == {}",
                    hotkeys[i], hotkeys[j]
                );
            }
        }
    }

    #[test]
    fn test_hotkeys_follow_naming_convention() {
        let config = Configuration::default();
        let hotkeys = vec![
            &config.hotkeys.screenshot,
            &config.hotkeys.hide_window,
            &config.hotkeys.show_gui,
            &config.hotkeys.silent_screenshot,
            &config.hotkeys.refresh_windows,
            &config.hotkeys.toggle_preview,
            &config.hotkeys.open_ai_browser,
        ];
        
        for hotkey in hotkeys {
            assert!(
                hotkey.contains("Ctrl") || hotkey.contains("Alt") || hotkey.contains("Shift"),
                "Hotkey '{}' should contain a modifier key",
                hotkey
            );
        }
    }

    #[test]
    fn test_config_roundtrip_preserves_all_hotkeys() {
        let original = Configuration::default();
        let serialized = toml::to_string(&original).unwrap();
        let deserialized: Configuration = toml::from_str(&serialized).unwrap();
        
        assert_eq!(original.hotkeys.screenshot, deserialized.hotkeys.screenshot);
        assert_eq!(original.hotkeys.hide_window, deserialized.hotkeys.hide_window);
        assert_eq!(original.hotkeys.show_gui, deserialized.hotkeys.show_gui);
        assert_eq!(original.hotkeys.silent_screenshot, deserialized.hotkeys.silent_screenshot);
        assert_eq!(original.hotkeys.refresh_windows, deserialized.hotkeys.refresh_windows);
        assert_eq!(original.hotkeys.toggle_preview, deserialized.hotkeys.toggle_preview);
        assert_eq!(original.hotkeys.open_ai_browser, deserialized.hotkeys.open_ai_browser);
    }

    #[test]
    fn test_ai_browser_window_state() {
        use crate::presentation::AiBrowserState;
        let state = AiBrowserState::default();
        assert!(!state.is_browser_open());
    }
}


#[cfg(test)]
mod wry_browser_tests {
    use crate::presentation::{AiBrowserState, AiTab};

    #[test]
    fn test_browser_state_default_not_open() {
        let state = AiBrowserState::default();
        assert!(!state.is_browser_open(), "Browser should not be open by default");
    }

    #[test]
    fn test_browser_state_default_tab_is_chatgpt() {
        let state = AiBrowserState::default();
        assert_eq!(state.active_tab, AiTab::ChatGpt, "Default tab should be ChatGPT");
    }

    #[test]
    fn test_browser_state_no_screenshot_path_initially() {
        let state = AiBrowserState::default();
        assert!(state.last_screenshot_path.is_none(), "No screenshot path should exist initially");
    }

    #[test]
    fn test_tab_selection_changes_active_tab() {
        let mut state = AiBrowserState::default();
        
        state.select_tab(AiTab::Claude);
        assert_eq!(state.active_tab, AiTab::Claude);
        
        state.select_tab(AiTab::Gemini);
        assert_eq!(state.active_tab, AiTab::Gemini);
        
        state.select_tab(AiTab::Antigravity);
        assert_eq!(state.active_tab, AiTab::Antigravity);
        
        state.select_tab(AiTab::ChatGpt);
        assert_eq!(state.active_tab, AiTab::ChatGpt);
    }

    #[test]
    fn test_all_tabs_have_https_urls() {
        for &tab in AiTab::all() {
            let url = tab.url();
            assert!(url.starts_with("https://"), "Tab {} should have HTTPS URL", tab.label());
        }
    }

    #[test]
    fn test_all_tabs_have_valid_domains() {
        let chatgpt_url = AiTab::ChatGpt.url();
        assert!(chatgpt_url.contains("chatgpt.com"));
        
        let antigravity_url = AiTab::Antigravity.url();
        assert!(antigravity_url.contains("antigravity.ai"));
        
        let gemini_url = AiTab::Gemini.url();
        assert!(gemini_url.contains("gemini.google.com"));
        
        let claude_url = AiTab::Claude.url();
        assert!(claude_url.contains("claude.ai"));
    }

    #[test]
    fn test_tab_labels_are_human_readable() {
        assert_eq!(AiTab::ChatGpt.label(), "ChatGPT");
        assert_eq!(AiTab::Antigravity.label(), "Antigravity");
        assert_eq!(AiTab::Gemini.label(), "Gemini");
        assert_eq!(AiTab::Claude.label(), "Claude");
    }

    #[test]
    fn test_tab_count_is_four() {
        assert_eq!(AiTab::all().len(), 4, "Should have exactly 4 AI tabs");
    }

    #[test]
    fn test_screenshot_path_can_be_set_and_cleared() {
        let mut state = AiBrowserState::default();
        
        let test_path = "C:\\temp\\screenshot_12345.png";
        state.set_screenshot_path(test_path.to_string());
        
        assert!(state.last_screenshot_path.is_some());
        assert_eq!(state.last_screenshot_path.as_deref(), Some(test_path));
        
        state.clear_screenshot_path();
        assert!(state.last_screenshot_path.is_none());
    }

    #[test]
    fn test_screenshot_path_can_be_overwritten() {
        let mut state = AiBrowserState::default();
        
        state.set_screenshot_path("path1.png".to_string());
        assert_eq!(state.last_screenshot_path.as_deref(), Some("path1.png"));
        
        state.set_screenshot_path("path2.png".to_string());
        assert_eq!(state.last_screenshot_path.as_deref(), Some("path2.png"));
        
        state.set_screenshot_path("path3.png".to_string());
        assert_eq!(state.last_screenshot_path.as_deref(), Some("path3.png"));
    }

    #[test]
    fn test_browser_state_is_thread_safe() {
        use std::sync::Arc;
        use std::thread;
        
        let state = Arc::new(AiBrowserState::default());
        let state_clone = Arc::clone(&state);
        
        let handle = thread::spawn(move || {
            assert!(!state_clone.is_browser_open());
        });
        
        handle.join().unwrap();
        assert!(!state.is_browser_open());
    }

    #[test]
    fn test_tab_enum_equality() {
        assert_eq!(AiTab::ChatGpt, AiTab::ChatGpt);
        assert_ne!(AiTab::ChatGpt, AiTab::Claude);
        assert_ne!(AiTab::Gemini, AiTab::Antigravity);
    }

    #[test]
    fn test_tab_enum_copy_trait() {
        let tab1 = AiTab::ChatGpt;
        let tab2 = tab1;
        assert_eq!(tab1, tab2);
    }

    #[test]
    fn test_all_tabs_iterator() {
        let tabs: Vec<AiTab> = AiTab::all().iter().copied().collect();
        assert_eq!(tabs.len(), 4);
        assert!(tabs.contains(&AiTab::ChatGpt));
        assert!(tabs.contains(&AiTab::Antigravity));
        assert!(tabs.contains(&AiTab::Gemini));
        assert!(tabs.contains(&AiTab::Claude));
    }

    #[test]
    fn test_browser_state_debug_format() {
        let state = AiBrowserState::default();
        let debug_str = format!("{:?}", state);
        assert!(debug_str.contains("AiBrowserState"));
        assert!(debug_str.contains("active_tab"));
    }

    #[test]
    fn test_tab_enum_debug_format() {
        let tab = AiTab::ChatGpt;
        let debug_str = format!("{:?}", tab);
        assert!(debug_str.contains("ChatGpt"));
    }

    #[test]
    fn test_screenshot_path_with_special_characters() {
        let mut state = AiBrowserState::default();
        
        let path_with_spaces = "C:\\Users\\Test User\\screenshot 123.png";
        state.set_screenshot_path(path_with_spaces.to_string());
        assert_eq!(state.last_screenshot_path.as_deref(), Some(path_with_spaces));
        
        let path_with_unicode = "C:\\temp\\screenshot_测试.png";
        state.set_screenshot_path(path_with_unicode.to_string());
        assert_eq!(state.last_screenshot_path.as_deref(), Some(path_with_unicode));
    }

    #[test]
    fn test_multiple_tab_switches() {
        let mut state = AiBrowserState::default();
        
        for _ in 0..10 {
            for &tab in AiTab::all() {
                state.select_tab(tab);
                assert_eq!(state.active_tab, tab);
            }
        }
    }

    #[test]
    fn test_browser_state_default_values() {
        let state = AiBrowserState::default();
        
        assert_eq!(state.active_tab, AiTab::ChatGpt);
        assert!(state.last_screenshot_path.is_none());
        assert!(!state.is_browser_open());
    }

    #[test]
    fn test_tab_url_immutability() {
        let url1 = AiTab::ChatGpt.url();
        let url2 = AiTab::ChatGpt.url();
        assert_eq!(url1, url2);
        assert_eq!(url1.as_ptr(), url2.as_ptr(), "URLs should be static strings");
    }

    #[test]
    fn test_tab_label_immutability() {
        let label1 = AiTab::ChatGpt.label();
        let label2 = AiTab::ChatGpt.label();
        assert_eq!(label1, label2);
        assert_eq!(label1.as_ptr(), label2.as_ptr(), "Labels should be static strings");
    }

    #[test]
    fn test_all_tabs_returns_static_slice() {
        let tabs1 = AiTab::all();
        let tabs2 = AiTab::all();
        assert_eq!(tabs1.len(), tabs2.len());
        assert_eq!(tabs1.as_ptr(), tabs2.as_ptr(), "Should return same static slice");
    }

    #[test]
    fn test_screenshot_path_empty_string() {
        let mut state = AiBrowserState::default();
        
        state.set_screenshot_path(String::new());
        assert!(state.last_screenshot_path.is_some());
        assert_eq!(state.last_screenshot_path.as_deref(), Some(""));
    }

    #[test]
    fn test_screenshot_path_very_long() {
        let mut state = AiBrowserState::default();
        
        let long_path = "C:\\".to_string() + &"very_long_directory_name\\".repeat(50) + "screenshot.png";
        state.set_screenshot_path(long_path.clone());
        assert_eq!(state.last_screenshot_path.as_deref(), Some(long_path.as_str()));
    }

    #[test]
    fn test_browser_state_can_be_cloned_partially() {
        let state = AiBrowserState::default();
        let tab_copy = state.active_tab;
        let path_clone = state.last_screenshot_path.clone();
        
        assert_eq!(tab_copy, AiTab::ChatGpt);
        assert_eq!(path_clone, None);
    }

    #[test]
    fn test_tab_pattern_matching() {
        let tab = AiTab::ChatGpt;
        
        let result = match tab {
            AiTab::ChatGpt => "matched",
            _ => "not matched",
        };
        
        assert_eq!(result, "matched");
    }

    #[test]
    fn test_all_tabs_can_be_matched() {
        for &tab in AiTab::all() {
            let matched = match tab {
                AiTab::ChatGpt | AiTab::Antigravity | AiTab::Gemini | AiTab::Claude => true,
            };
            assert!(matched, "Tab {:?} should be matchable", tab);
        }
    }
}

#[cfg(test)]
mod wry_integration_tests {
    use crate::presentation::{AiBrowserState, AiTab};
    use std::sync::{Arc, Mutex};
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_concurrent_browser_state_access() {
        let state = Arc::new(Mutex::new(AiBrowserState::default()));
        let mut handles = vec![];

        for i in 0..4 {
            let state_clone = Arc::clone(&state);
            let handle = thread::spawn(move || {
                let mut s = state_clone.lock().unwrap();
                let tab = AiTab::all()[i];
                s.select_tab(tab);
                assert_eq!(s.active_tab, tab);
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }
    }

    #[test]
    fn test_screenshot_path_concurrent_updates() {
        let state = Arc::new(Mutex::new(AiBrowserState::default()));
        let mut handles = vec![];

        for i in 0..10 {
            let state_clone = Arc::clone(&state);
            let handle = thread::spawn(move || {
                let mut s = state_clone.lock().unwrap();
                s.set_screenshot_path(format!("path_{}.png", i));
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        let final_state = state.lock().unwrap();
        assert!(final_state.last_screenshot_path.is_some());
    }

    #[test]
    fn test_browser_state_is_send() {
        fn assert_send<T: Send>() {}
        assert_send::<AiBrowserState>();
    }

    #[test]
    fn test_ai_tab_is_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<AiTab>();
    }

    #[test]
    fn test_browser_open_state_persistence() {
        let state = AiBrowserState::default();
        assert!(!state.is_browser_open());
        
        thread::sleep(Duration::from_millis(10));
        assert!(!state.is_browser_open());
    }
}
