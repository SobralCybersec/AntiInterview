#[cfg(test)]
mod tests {
    use super::super::nt_hook::*;

    #[test]
    fn test_is_hook_available() {
        let available = is_hook_available();
        println!("Hook DLL available: {}", available);
    }

    #[test]
    fn test_hook_load_when_available() {
        if !is_hook_available() {
            println!("Skipping test: hook.dll not available");
            return;
        }

        let result = NtHook::load();
        assert!(result.is_ok(), "Failed to load hook DLL: {:?}", result.err());
    }

    #[test]
    fn test_set_hidden_process() {
        if !is_hook_available() {
            println!("Skipping test: hook.dll not available");
            return;
        }

        let hook = NtHook::load().expect("Failed to load hook");
        let result = hook.set_hidden_process("chrome.exe");
        assert!(result.is_ok(), "Failed to set hidden process: {:?}", result.err());
    }

    #[test]
    fn test_clear_hidden_process() {
        if !is_hook_available() {
            println!("Skipping test: hook.dll not available");
            return;
        }

        let hook = NtHook::load().expect("Failed to load hook");
        hook.set_hidden_process("test.exe").ok();
        let result = hook.clear_hidden_process();
        assert!(result.is_ok(), "Failed to clear hidden process: {:?}", result.err());
    }

    #[test]
    fn test_multiple_process_names() {
        if !is_hook_available() {
            println!("Skipping test: hook.dll not available");
            return;
        }

        let hook = NtHook::load().expect("Failed to load hook");
        
        let processes = vec!["chrome.exe", "firefox.exe", "msedge.exe"];
        
        for process in processes {
            let result = hook.set_hidden_process(process);
            assert!(result.is_ok(), "Failed to set {}: {:?}", process, result.err());
        }
    }

    #[test]
    fn test_hook_load_failure_when_missing() {
        if is_hook_available() {
            println!("Skipping test: hook.dll is available");
            return;
        }

        let result = NtHook::load();
        assert!(result.is_err(), "Expected error when hook.dll is missing");
    }
}
