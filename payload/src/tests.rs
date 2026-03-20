#[cfg(test)]
mod tests {
    use crate::{
        install_hook, SetHiddenProcessName, HIDE_PROCESS_NAME, SYSTEM_PROCESS_INFORMATION,
        UNICODE_STRING,
    };
    use std::thread;

    #[test]
    fn test_hide_process_name_storage() {
        let name = "notepad.exe";
        let name_wide: Vec<u16> = name.encode_utf16().chain(std::iter::once(0)).collect();

        let result = SetHiddenProcessName(name_wide.as_ptr());
        assert!(result, "Failed to set hidden process name");

        let stored_name = HIDE_PROCESS_NAME.read().unwrap();
        assert_eq!(*stored_name, name);
    }

    #[test]
    fn test_hide_process_name_case_insensitive() {
        let name = "NOTEPAD.EXE";
        let name_wide: Vec<u16> = name.encode_utf16().chain(std::iter::once(0)).collect();

        let result = SetHiddenProcessName(name_wide.as_ptr());
        assert!(result);

        let stored_name = HIDE_PROCESS_NAME.read().unwrap();
        assert_eq!(stored_name.to_lowercase(), name.to_lowercase());
    }

    #[test]
    fn test_hide_process_name_substring_matching() {
        let name = "anti";
        let name_wide: Vec<u16> = name.encode_utf16().chain(std::iter::once(0)).collect();

        let result = SetHiddenProcessName(name_wide.as_ptr());
        assert!(result);

        let stored_name = HIDE_PROCESS_NAME.read().unwrap();
        assert!(stored_name.contains("anti"));
    }

    #[test]
    fn test_hide_process_name_null_pointer() {
        let result = SetHiddenProcessName(std::ptr::null());
        assert!(!result, "Should return false for null pointer");
    }

    #[test]
    fn test_hide_process_name_empty_string() {
        let name = "";
        let name_wide: Vec<u16> = name.encode_utf16().chain(std::iter::once(0)).collect();

        let result = SetHiddenProcessName(name_wide.as_ptr());
        assert!(result);

        let stored_name = HIDE_PROCESS_NAME.read().unwrap();
        assert_eq!(*stored_name, "");
    }

    #[test]
    fn test_unicode_string_conversion() {
        let test_name = "test.exe";
        let wide: Vec<u16> = test_name.encode_utf16().collect();

        let unicode_str = UNICODE_STRING {
            length: (wide.len() * 2) as u16,
            maximum_length: (wide.len() * 2) as u16,
            buffer: wide.as_ptr() as *mut u16,
        };

        let name_len = (unicode_str.length / 2) as usize;
        let name_slice = unsafe { std::slice::from_raw_parts(unicode_str.buffer, name_len) };
        let result = String::from_utf16(name_slice).unwrap();

        assert_eq!(result, test_name);
    }

    #[test]
    fn test_process_info_linked_list_structure() {
        let mut proc1 = SYSTEM_PROCESS_INFORMATION {
            next_entry_offset: 100,
            number_of_threads: 1,
            working_set_private_size: 0,
            hard_fault_count: 0,
            number_of_threads_high_water_mark: 0,
            cycle_time: 0,
            create_time: 0,
            user_time: 0,
            kernel_time: 0,
            image_name: UNICODE_STRING {
                length: 0,
                maximum_length: 0,
                buffer: std::ptr::null_mut(),
            },
            base_priority: 0,
            unique_process_id: 1234,
            inherited_from_unique_process_id: 0,
        };

        assert_eq!(proc1.next_entry_offset, 100);
        assert_eq!(proc1.unique_process_id, 1234);
        assert_eq!(proc1.inherited_from_unique_process_id, 0);

        proc1.next_entry_offset = 0;
        assert_eq!(proc1.next_entry_offset, 0);
    }

    #[test]
    fn test_hook_installation_idempotent() {
        let result1 = install_hook();
        let result2 = install_hook();

        assert_eq!(result1, result2, "Hook installation should be idempotent");
    }

    #[test]
    fn test_multiple_process_names() {
        let names = vec!["notepad", "chrome", "firefox"];

        for name in names {
            let name_wide: Vec<u16> = name.encode_utf16().chain(std::iter::once(0)).collect();
            let result = SetHiddenProcessName(name_wide.as_ptr());
            assert!(result);

            let stored_name = HIDE_PROCESS_NAME.read().unwrap();
            assert_eq!(*stored_name, name);
        }
    }

    #[test]
    fn test_process_name_with_path() {
        let name = "C:\\Windows\\System32\\notepad.exe";
        let name_wide: Vec<u16> = name.encode_utf16().chain(std::iter::once(0)).collect();

        let result = SetHiddenProcessName(name_wide.as_ptr());
        assert!(result);

        let stored_name = HIDE_PROCESS_NAME.read().unwrap();
        assert_eq!(*stored_name, name);
    }

    #[test]
    fn test_process_name_special_characters() {
        let name = "test-app_v1.2.exe";
        let name_wide: Vec<u16> = name.encode_utf16().chain(std::iter::once(0)).collect();

        let result = SetHiddenProcessName(name_wide.as_ptr());
        assert!(result);

        let stored_name = HIDE_PROCESS_NAME.read().unwrap();
        assert_eq!(*stored_name, name);
    }

    #[test]
    fn test_concurrent_access() {
        let handles: Vec<_> = (0..10)
            .map(|i| {
                thread::spawn(move || {
                    let name = format!("process{}.exe", i);
                    let name_wide: Vec<u16> =
                        name.encode_utf16().chain(std::iter::once(0)).collect();
                    SetHiddenProcessName(name_wide.as_ptr())
                })
            })
            .collect();

        for handle in handles {
            assert!(handle.join().unwrap());
        }
    }
}
