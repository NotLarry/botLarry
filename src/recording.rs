use std::fs;
use std::process::Command;
use std::thread;
use std::time::Duration;
use rppal::gpio::{InputPin, OutputPin, Level};
use log::info;


/// Constants for keypad scanning
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

/// Handle an unknown number call flow using WAV + MP3 conversion.
/// Uses pre-initialized GPIO (no reinit).
pub fn handle_unknown_number(
    rows: &Vec<InputPin>,
    cols: &mut Vec<OutputPin>,
    switch: &InputPin,
    number: &str,
) -> bool {
    let area_code = &number[0..3];
    let dir_path = format!("/botLarry/recordings/{}", area_code);
    let wav_path = format!("{}/{}.wav", dir_path, number);
    let mp3_path = format!("{}/{}.mp3", dir_path, number);

    fs::create_dir_all(&dir_path).expect("Failed to create recording directory");

    // Ring tone
    let mut child = Command::new("sox")
        .args(["-n", "-t", "alsa", "hw:0,0", "synth", "2", "sin", "440", "sin", "480"])
        .spawn()
        .expect("Failed to play ring tone");
    thread::sleep(Duration::from_secs(2));
    let _ = child.kill();
    thread::sleep(Duration::from_secs(4));

    // Play unassigned message
    let _ = Command::new("mpg123")
        .args(["-a", "hw:0,0", "/botLarry/utility/unassigned.mp3"])
        .status()
        .expect("Failed to play unassigned.mp3");

    // Beep
    let _ = Command::new("sox")
        .args(["-n", "-t", "alsa", "hw:0,0", "synth", "0.2", "sin", "1000"])
        .spawn()
        .and_then(|mut c| c.wait());


    // Start recording to WAV
    let mut arecord = Command::new("arecord")
        .args(["-D", "hw:0,0", "-f", "cd", &wav_path])
        .spawn()
        .expect("Failed to start recording");

    info!("File path: {}", mp3_path);
    info!("🎙️  Recording... Press '#' to stop and save. Hang up to cancel.");

    // Wait loop
    loop {
        if switch.read() == Level::High {
            info!("📞 Hangup detected — discarding recording.");
            let _ = arecord.kill();
            let _ = fs::remove_file(&wav_path);
            return false;
        }

        if let Some(key) = get_key(rows, cols) {
            if key == '#' {
                info!("✅ '#' received — stopping and saving recording.");
                break;
            }
        }

        thread::sleep(Duration::from_millis(100));
    }

    let _ = arecord.kill();

    // Convert to MP3
    let _ = Command::new("lame")
        .args([&wav_path, &mp3_path])
        .status()
        .expect("Failed to encode MP3");

    let _ = fs::remove_file(&wav_path);
    true
}

