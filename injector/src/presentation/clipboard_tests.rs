#[cfg(test)]
mod tests {
    #[test]
    fn test_arboard_image_data_structure() {
        use arboard::ImageData;

        let width = 100;
        let height = 100;
        let bytes: Vec<u8> = vec![255; width * height * 4];

        let img_data = ImageData {
            width,
            height,
            bytes: bytes.into(),
        };

        assert_eq!(img_data.width, width);
        assert_eq!(img_data.height, height);
        assert_eq!(img_data.bytes.len(), width * height * 4);
    }

    #[test]
    fn test_rgba_pixel_format() {
        let width = 2;
        let height = 2;
        let bytes: Vec<u8> = vec![
            255, 0, 0, 255, // Red pixel
            0, 255, 0, 255, // Green pixel
            0, 0, 255, 255, // Blue pixel
            255, 255, 255, 255, // White pixel
        ];

        assert_eq!(bytes.len(), width * height * 4);
        assert_eq!(bytes[0], 255); // Red channel
        assert_eq!(bytes[1], 0); // Green channel
        assert_eq!(bytes[2], 0); // Blue channel
        assert_eq!(bytes[3], 255); // Alpha channel
    }

    #[test]
    fn test_auto_paste_screenshot_config_default() {
        let config = crate::domain::Configuration::default();
        assert!(!config.ui.auto_paste_screenshot);
    }

    #[test]
    fn test_auto_hide_ai_browser_config_default() {
        let config = crate::domain::Configuration::default();
        assert!(config.ui.auto_hide_ai_browser);
    }

    #[test]
    fn test_config_serialization_with_new_fields() {
        let mut config = crate::domain::Configuration::default();
        config.ui.auto_paste_screenshot = true;
        config.ui.auto_hide_ai_browser = false;

        let serialized = toml::to_string(&config).unwrap();
        assert!(serialized.contains("auto_paste_screenshot"));
        assert!(serialized.contains("auto_hide_ai_browser"));
    }

    #[test]
    fn test_config_deserialization_with_new_fields() {
        let toml_str = r#"
            [hotkeys]
            screenshot = "Ctrl+S"
            hide_window = "Ctrl+H"
            show_gui = "Ctrl+I"
            
            [ui]
            dark_theme = true
            show_preview = false
            hide_from_taskbar = false
            window_size = [320.0, 540.0]
            auto_paste_screenshot = true
            auto_hide_ai_browser = false
        "#;

        let config: crate::domain::Configuration = toml::from_str(toml_str).unwrap();
        assert!(config.ui.auto_paste_screenshot);
        assert!(!config.ui.auto_hide_ai_browser);
    }

    #[test]
    fn test_config_backward_compatibility() {
        let config = crate::domain::Configuration::default();
        assert!(!config.ui.auto_paste_screenshot);
        assert!(config.ui.auto_hide_ai_browser);
    }

    #[test]
    fn test_image_data_cow_owned() {
        use arboard::ImageData;

        let bytes: Vec<u8> = vec![255; 400];
        let img_data = ImageData {
            width: 10,
            height: 10,
            bytes: bytes.into(),
        };

        assert_eq!(img_data.bytes.len(), 400);
    }

    #[test]
    fn test_clipboard_image_dimensions() {
        use arboard::ImageData;

        let width = 1920;
        let height = 1080;
        let bytes: Vec<u8> = vec![0; width * height * 4];

        let img_data = ImageData {
            width,
            height,
            bytes: bytes.into(),
        };

        assert_eq!(img_data.width, 1920);
        assert_eq!(img_data.height, 1080);
    }
}
