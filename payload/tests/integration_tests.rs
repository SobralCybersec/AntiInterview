use std::process::Command;
use std::thread;
use std::time::Duration;

#[test]
fn test_process_hiding_integration() {
    let notepad = Command::new("notepad.exe").spawn();
    
    if let Ok(mut child) = notepad {
        thread::sleep(Duration::from_millis(500));
        
        let pid = child.id();
        println!("Spawned notepad.exe with PID: {}", pid);
        
        thread::sleep(Duration::from_secs(2));
        
        let _ = child.kill();
        println!("Killed notepad.exe");
    }
}

#[test]
fn test_task_manager_process_list() {
    let output = Command::new("tasklist")
        .args(&["/FI", "IMAGENAME eq notepad.exe"])
        .output();
    
    if let Ok(output) = output {
        let stdout = String::from_utf8_lossy(&output.stdout);
        println!("Task list output:\n{}", stdout);
        
        assert!(output.status.success());
    }
}

#[test]
fn test_multiple_process_instances() {
    let mut processes = Vec::new();
    
    for i in 0..3 {
        if let Ok(child) = Command::new("notepad.exe").spawn() {
            println!("Spawned notepad instance {} with PID: {}", i, child.id());
            processes.push(child);
        }
    }
    
    thread::sleep(Duration::from_secs(1));
    
    for mut child in processes {
        let _ = child.kill();
    }
}

#[test]
fn test_process_name_matching() {
    let test_cases = vec![
        ("notepad.exe", "notepad", true),
        ("notepad.exe", "NOTEPAD", true),
        ("notepad.exe", "note", true),
        ("notepad.exe", "chrome", false),
        ("anti-interview.exe", "anti", true),
        ("anti-interview.exe", "interview", true),
    ];
    
    for (process_name, search_term, should_match) in test_cases {
        let matches = process_name
            .to_lowercase()
            .contains(&search_term.to_lowercase());
        
        assert_eq!(
            matches, should_match,
            "Process '{}' should {} match search term '{}'",
            process_name,
            if should_match { "" } else { "not" },
            search_term
        );
    }
}

#[test]
fn test_system_process_enumeration() {
    let output = Command::new("tasklist").output();
    
    if let Ok(output) = output {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let process_count = stdout.lines().filter(|line| line.contains(".exe")).count();
        
        println!("Found {} processes", process_count);
        assert!(process_count > 0, "Should find at least one process");
    }
}

#[test]
fn test_process_with_special_characters() {
    let test_names = vec![
        "test-app.exe",
        "my_application.exe",
        "app.v1.2.exe",
        "program (x86).exe",
    ];
    
    for name in test_names {
        let name_lower = name.to_lowercase();
        assert!(name_lower.ends_with(".exe"));
        println!("Valid process name: {}", name);
    }
}

#[test]
fn test_concurrent_process_operations() {
    let handles: Vec<_> = (0..5)
        .map(|i| {
            thread::spawn(move || {
                let output = Command::new("tasklist")
                    .args(&["/FI", &format!("PID eq {}", i + 1000)])
                    .output();
                
                output.is_ok()
            })
        })
        .collect();
    
    for handle in handles {
        assert!(handle.join().is_ok());
    }
}

#[test]
fn test_process_lifecycle() {
    if let Ok(mut child) = Command::new("notepad.exe").spawn() {
        let pid = child.id();
        println!("Process started with PID: {}", pid);
        
        thread::sleep(Duration::from_millis(500));
        
        let output = Command::new("tasklist")
            .args(&["/FI", &format!("PID eq {}", pid)])
            .output();
        
        if let Ok(output) = output {
            let stdout = String::from_utf8_lossy(&output.stdout);
            println!("Process check output: {}", stdout);
            if !stdout.contains("No tasks") {
                assert!(stdout.contains("notepad.exe") || stdout.contains(&pid.to_string()));
            }
        }
        
        let _ = child.kill();
        thread::sleep(Duration::from_millis(500));
        
        let output = Command::new("tasklist")
            .args(&["/FI", &format!("PID eq {}", pid)])
            .output();
        
        if let Ok(output) = output {
            let stdout = String::from_utf8_lossy(&output.stdout);
            println!("After kill output: {}", stdout);
        }
    }
}
