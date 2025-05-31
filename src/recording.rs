// This assumes you already have functions to:
// - Detect when the number is not found
// - Read hook state
// - Collect keypad input

use chrono::Local;
use std::fs;
use std::path::Path;
use std::process::{Command, Child};
use std::thread;
use std::time::Duration;
use rppal::gpio::{InputPin, Level};

/// Called when a number is not found in the database
pub fn handle_unknown_number(switch: &InputPin, number: &str) {
    let area_code = &number[0..3];
    let timestamp = Local::now().format("%Y%m%d%H%M%S").to_string();
    let dir_path = format!("recordings/{}", area_code);
    let file_path = format!("{}/{}-{}.mp3", dir_path, number, timestamp);

    // Create the directory if needed
    fs::create_dir_all(&dir_path).expect("Failed to create recording directory");

    // Step 1: Ring three times (2s on, 4s off x3)
    for _ in 0..3 {
        let mut child = Command::new("sox")
            .args(["-n", "-t", "alsa", "hw:0,0", "synth", "2", "sin", "440", "sin", "480"])
            .spawn()
            .expect("Failed to play ring tone");
        thread::sleep(Duration::from_secs(2));
        let _ = child.kill();
        thread::sleep(Duration::from_secs(4));
    }

    // Step 2: Play a beep
    let _ = Command::new("sox")
        .args(["-n", "-t", "alsa", "hw:0,0", "synth", "0.2", "sin", "1000"])
        .spawn()
        .and_then(|mut c| c.wait());

    // Step 3: Start recording until hangup or `#`
    let mut arecord = Command::new("arecord")
        .args(["-D", "hw:1,0", "-f", "cd", &file_path])
        .spawn()
        .expect("Failed to start recording");

loop {
    if switch.read() == Level::High {
        println!("⏹️ On-hook detected. Stopping recording.");
        break;
    }

    thread::sleep(Duration::from_millis(200));
}


    let _ = arecord.kill();
}

