use std::process::Command;
use std::thread;
use std::time::Duration;

fn main() {
    println!("=== Process Hiding Test ===\n");
    
    // Start notepad as test process
    println!("1. Starting notepad.exe...");
    let mut notepad = Command::new("notepad.exe")
        .spawn()
        .expect("Failed to start notepad");
    
    let pid = notepad.id();
    println!("   Notepad PID: {}\n", pid);
    
    thread::sleep(Duration::from_secs(2));
    
    // Check if visible in tasklist
    println!("2. Checking if notepad is visible in tasklist...");
    let output = Command::new("tasklist")
        .args(&["/FI", &format!("PID eq {}", pid)])
        .output()
        .expect("Failed to run tasklist");
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    if stdout.contains("notepad.exe") {
        println!("   ✓ Notepad is visible in tasklist\n");
    } else {
        println!("   ✗ Notepad NOT visible in tasklist\n");
    }
    
    println!("3. Instructions:");
    println!("   - Open Task Manager");
    println!("   - Verify notepad.exe is visible");
    println!("   - Run anti-interview.exe as administrator");
    println!("   - Enable 'Ocultar do Task Manager' in settings");
    println!("   - Hide the notepad window");
    println!("   - Check Task Manager again");
    println!("   - Notepad should be HIDDEN\n");
    
    println!("Press Enter to kill notepad and exit...");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    
    println!("\n4. Killing notepad...");
    notepad.kill().expect("Failed to kill notepad");
    println!("   ✓ Notepad killed\n");
    
    println!("Test complete!");
}
