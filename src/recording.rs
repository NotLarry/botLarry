use chrono::Local;
use std::fs;
use std::path::Path;
use std::process::{Command, Child};
use std::thread;
use std::time::Duration;
use rppal::gpio::{Gpio, InputPin, OutputPin, Level};

/// Constants for keypad scanning
const ROW_PINS: [u8; 4] = [16, 25, 24, 23];
const COL_PINS: [u8; 3] = [22, 27, 17];
const KEYPAD: [[char; 3]; 4] = [
    ['1', '2', '3'],
    ['4', '5', '6'],
    ['7', '8', '9'],
    ['*', '0', '#'],
];

/// Attempt to get a key press (non-blocking)
fn get_key(rows: &Vec<InputPin>, cols: &mut Vec<OutputPin>) -> Option<char> {
    for (col_idx, col) in cols.iter_mut().enumerate() {
        col.set_low();
        for (row_idx, row) in rows.iter().enumerate() {
            if row.read() == Level::Low {
                thread::sleep(Duration::from_millis(50)); // debounce
                if row.read() == Level::Low {
                    col.set_high();
                    return Some(KEYPAD[row_idx][col_idx]);
                }
            }
        }
        col.set_high();
    }
    None
}

/// Handle an unknown number call flow
/// Returns true if recording was saved successfully
pub fn handle_unknown_number(
    rows: &Vec<InputPin>,
    cols: &mut Vec<OutputPin>,
    switch: &InputPin,
    number: &str,
) -> bool {
    let area_code = &number[0..3];
    let dir_path = format!("/botLarry/recordings/{}", area_code);
    let file_path = format!("{}/{}.mp3", dir_path, number);

    fs::create_dir_all(&dir_path).expect("Failed to create recording directory");

    for _ in 0..1 {
        let mut child = Command::new("sox")
            .args(["-n", "-t", "alsa", "hw:0,0", "synth", "2", "sin", "440", "sin", "480"])
            .spawn()
            .expect("Failed to play ring tone");
        thread::sleep(Duration::from_secs(2));
        let _ = child.kill();
        thread::sleep(Duration::from_secs(4));
    }

    let _ = Command::new("sox")
        .args(["-n", "-t", "alsa", "hw:0,0", "synth", "0.2", "sin", "1000"])
        .spawn()
        .and_then(|mut c| c.wait());

    let mut arecord = Command::new("arecord")
        .args(["-D", "hw:0,0", "-f", "cd", &file_path])
        .spawn()
        .expect("Failed to start recording");

    println!("File path: {}", file_path);
    println!("🎙️  Recording... Press '#' to stop and save. Hang up to cancel.");

    loop {
        if switch.read() == Level::High {
            println!("📞 Hangup detected — discarding recording.");
            let _ = arecord.kill();
            let _ = fs::remove_file(&file_path);
            return false;
        }

        if let Some(key) = get_key(rows, cols) {
            if key == '#' {
                println!("✅ '#' received — stopping and saving recording.");
                break;
            }
        }

        thread::sleep(Duration::from_millis(100));
    }

    let _ = arecord.kill();
    true
}


