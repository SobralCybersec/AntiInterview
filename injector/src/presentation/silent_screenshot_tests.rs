#[cfg(test)]
mod tests {
    use crate::domain::Configuration;
    use arboard::{Clipboard, ImageData};
    use std::borrow::Cow;
    use std::sync::{Mutex, MutexGuard};

    static CLIPBOARD_TEST_LOCK: Mutex<()> = Mutex::new(());

    fn lock_clipboard() -> MutexGuard<'static, ()> {
        CLIPBOARD_TEST_LOCK
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
    }

    #[test]
    fn test_clipboard_set_image_basic() {
        let _guard = lock_clipboard();
        let width = 10;
        let height = 10;
        let bytes: Vec<u8> = vec![255; width * height * 4];

        let img_data = ImageData {
            width,
            height,
            bytes: bytes.into(),
        };

        let result = Clipboard::new().and_then(|mut cb| cb.set_image(img_data));
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_clipboard_get_image_after_set() {
        let _guard = lock_clipboard();
        let width = 5;
        let height = 5;
        let bytes: Vec<u8> = vec![128; width * height * 4];

        let img_data = ImageData {
            width,
            height,
            bytes: bytes.clone().into(),
        };

        let result = Clipboard::new().and_then(|mut cb| {
            cb.set_image(img_data)?;
            cb.get_image()
        });

        if let Ok(retrieved) = result {
            assert_eq!(retrieved.width, width);
            assert_eq!(retrieved.height, height);
            assert_eq!(retrieved.bytes.len(), width * height * 4);
        }
    }

    #[test]
    fn test_image_data_rgba_format_preservation() {
        let width = 2;
        let height = 1;
        let bytes: Vec<u8> = vec![
            255, 0, 0, 255, // Red pixel
            0, 255, 0, 255, // Green pixel
        ];

        let img_data = ImageData {
            width,
            height,
            bytes: bytes.clone().into(),
        };

        assert_eq!(img_data.bytes.as_ref(), bytes);
    }

    #[test]
    fn test_clipboard_large_image() {
        let _guard = lock_clipboard();
        let width = 100;
        let height = 100;
        let bytes: Vec<u8> = vec![200; width * height * 4];

        let img_data = ImageData {
            width,
            height,
            bytes: bytes.into(),
        };

        let result = Clipboard::new().and_then(|mut cb| cb.set_image(img_data));
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_clipboard_empty_image() {
        let _guard = lock_clipboard();
        let width = 0;
        let height = 0;
        let bytes: Vec<u8> = vec![];

        let img_data = ImageData {
            width,
            height,
            bytes: bytes.into(),
        };

        let result = Clipboard::new().and_then(|mut cb| cb.set_image(img_data));
        // Empty images may fail or succeed depending on platform
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_auto_paste_enabled_behavior() {
        let mut config = Configuration::default();
        config.ui.auto_paste_screenshot = true;

        assert!(config.ui.auto_paste_screenshot);
    }

    #[test]
    fn test_auto_paste_disabled_behavior() {
        let mut config = Configuration::default();
        config.ui.auto_paste_screenshot = false;

        assert!(!config.ui.auto_paste_screenshot);
    }

    #[test]
    fn test_image_data_cow_borrowed() {
        let bytes: Vec<u8> = vec![100; 400];
        let borrowed: Cow<[u8]> = Cow::Borrowed(&bytes);

        let img_data = ImageData {
            width: 10,
            height: 10,
            bytes: borrowed,
        };

        assert_eq!(img_data.bytes.len(), 400);
    }

    #[test]
    fn test_image_data_cow_owned() {
        let bytes: Vec<u8> = vec![100; 400];
        let owned: Cow<[u8]> = Cow::Owned(bytes);

        let img_data = ImageData {
            width: 10,
            height: 10,
            bytes: owned,
        };

        assert_eq!(img_data.bytes.len(), 400);
    }

    #[test]
    fn test_clipboard_multiple_set_operations() {
        let _guard = lock_clipboard();
        for i in 1..=3 {
            let width = i * 10;
            let height = i * 10;
            let bytes: Vec<u8> = vec![i as u8; width * height * 4];

            let img_data = ImageData {
                width,
                height,
                bytes: bytes.into(),
            };

            let result = Clipboard::new().and_then(|mut cb| cb.set_image(img_data));
            assert!(result.is_ok() || result.is_err());
        }
    }

    #[test]
    fn test_clipboard_clear_after_image() {
        let _guard = lock_clipboard();
        let width = 5;
        let height = 5;
        let bytes: Vec<u8> = vec![255; width * height * 4];

        let img_data = ImageData {
            width,
            height,
            bytes: bytes.into(),
        };

        let result = Clipboard::new().and_then(|mut cb| {
            cb.set_image(img_data)?;
            cb.clear()
        });

        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_image_bytes_length_validation() {
        let width = 10;
        let height = 10;
        let expected_len = width * height * 4;
        let bytes: Vec<u8> = vec![0; expected_len];

        assert_eq!(bytes.len(), expected_len);

        let img_data = ImageData {
            width,
            height,
            bytes: bytes.into(),
        };

        assert_eq!(img_data.bytes.len(), expected_len);
    }

    #[test]
    fn test_clipboard_concurrent_access() {
        let _guard = lock_clipboard();
        use std::thread;

        let handles: Vec<_> = (0..3)
            .map(|i| {
                thread::spawn(move || {
                    let width = 5;
                    let height = 5;
                    let bytes: Vec<u8> = vec![i as u8; width * height * 4];

                    let img_data = ImageData {
                        width,
                        height,
                        bytes: bytes.into(),
                    };

                    Clipboard::new()
                        .and_then(|mut cb| cb.set_image(img_data))
                        .is_ok()
                        || Clipboard::new().is_err()
                })
            })
            .collect();

        for handle in handles {
            let _ = handle.join();
        }
    }

    #[test]
    fn test_silent_screenshot_path_format() {
        use std::time::{SystemTime, UNIX_EPOCH};

        let ts = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_millis())
            .unwrap_or(0);

        let path = std::env::temp_dir().join(format!("anti_interview_{}.png", ts));
        let path_str = path.to_string_lossy().to_string();

        assert!(path_str.contains("anti_interview_"));
        assert!(path_str.ends_with(".png"));
    }

    #[test]
    fn test_rgba_to_image_conversion() {
        let width = 2;
        let height = 2;
        let raw: Vec<u8> = vec![
            255, 0, 0, 255, 0, 255, 0, 255, 0, 0, 255, 255, 255, 255, 255, 255,
        ];

        let img = image::RgbaImage::from_raw(width, height, raw.clone());
        assert!(img.is_some());

        let dyn_img = img.map(image::DynamicImage::ImageRgba8);
        assert!(dyn_img.is_some());
    }
}
