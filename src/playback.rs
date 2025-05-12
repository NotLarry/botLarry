use std::process::{Command, Child, Stdio};
//use std::sync::{Arc, Mutex};
use std::sync::{Mutex};
use std::thread;
use std::time::Duration;
use rppal::gpio::{InputPin, Level};
use once_cell::sync::Lazy;

// Global handle to the dial tone process
static DIAL_TONE_PROCESS: Lazy<Mutex<Option<Child>>> = Lazy::new(|| Mutex::new(None));
/// Starts looping the dial tone in the background
pub fn start_dial_tone(audio_device: &str) {
    let child = Command::new("mpg123")
        .arg("--loop")
        .arg("-1")
        .arg("-a")
        .arg(audio_device)
        .arg("utility/dial_tone.mp3")
        .spawn()
        .expect("Failed to play dial tone");

    let mut proc_lock = DIAL_TONE_PROCESS.lock().unwrap();
    *proc_lock = Some(child);
}


/// Stops the dial tone if it is running
pub fn stop_dial_tone() {
    let mut proc_lock = DIAL_TONE_PROCESS.lock().unwrap();
    if let Some(child) = proc_lock.as_mut() {
        let _ = child.kill();
    }
    *proc_lock = None;
}

/// Plays the keypress beep once (blocking)
pub fn play_keypress_beep(device: &str) {
    let _ = Command::new("mpg123")
        .arg("-a")
        .arg(device)
        .arg("utility/keypress.mp3")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();
}

/// (Optional) Plays ringing.mp3 then a main file, both interruptible on hook
pub fn play_mp3_blocking_until_onhook(switch: &InputPin, main_path: &str) {
    let device = "hw:0,0";

    fn play_file_blocking(switch: &InputPin, path: &str, device: &str) -> bool {
        let mut child = Command::new("mpg123")
            .arg("-a")
            .arg(device)
            .arg(path)
            .spawn()
            .ok();

        while let Some(ref mut c) = child {
            if switch.read() == Level::High {
                let _ = c.kill();
                let _ = c.wait();
                println!("⏹️ On-hook detected. Playback interrupted.");
                return false;
            }

            match c.try_wait() {
                Ok(Some(_)) => return true,
                Ok(None) => thread::sleep(Duration::from_millis(100)),
                Err(_) => return false,
            }
        }

        true
    }

    if !play_file_blocking(switch, "utility/ringing.mp3", device) {
        return;
    }

    let _ = play_file_blocking(switch, main_path, device);
}

