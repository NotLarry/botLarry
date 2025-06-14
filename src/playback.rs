use std::process::{Command, Child, Stdio};
use std::sync::Mutex;
use std::thread;
use std::time::Duration;
use rppal::gpio::{InputPin, Level};
use once_cell::sync::Lazy;

use std::sync::atomic::{AtomicUsize, Ordering};
use rppal::gpio::{Trigger};
use log::info;


// Volume levels: 0 = Low, 1 = Medium, 2 = High
static VOLUME_LEVEL: AtomicUsize = AtomicUsize::new(2); // Start at High
const VOLUME_VALUES: [u8; 3] = [20, 50, 100]; // Percentages

pub fn setup_volume_button(gpio: &rppal::gpio::Gpio) {
    let mut button_pin = gpio.get(6).unwrap().into_input_pullup();

    button_pin
        .set_async_interrupt(
            Trigger::FallingEdge,
            Some(Duration::from_millis(50)),  // debounce time
            move |_| {
                let current = VOLUME_LEVEL.load(Ordering::Relaxed);
                let next = (current + 1) % VOLUME_VALUES.len();
                VOLUME_LEVEL.store(next, Ordering::Relaxed);

                let percent = VOLUME_VALUES[next];
                let _ = Command::new("amixer")
                    .args(["-c", "0", "sset", "PCM", &format!("{}%", percent)])
                    .status();

                info!("🔊 Volume set to {}%", percent);
            },
        )

        .expect("Failed to set interrupt on volume button");
}

// Shared process handle for any background tone (dial or ring)
static DIAL_TONE_PROCESS: Lazy<Mutex<Option<Child>>> = Lazy::new(|| Mutex::new(None));

/// Starts a digital dial tone in the background (350 Hz + 440 Hz, continuous)
pub fn start_dial_tone(audio_device: &str) {
    let child = Command::new("sox")
        .args([
            "-n",
            "-t", "alsa", audio_device,
            "synth", "0",  // infinite duration
            "sin", "350",
            "sin", "440",
            "channels", "2",
        ])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("Failed to start digital dial tone");

    let mut proc_lock = DIAL_TONE_PROCESS.lock().unwrap();
    *proc_lock = Some(child);
}

/// Starts a digitally synthesized US-style telephone ring (440+480 Hz, 2s on, 4s off)
pub fn start_ringing_tone(audio_device: &str) {
    let child = Command::new("sox")
        .args([
            "-n",
            "-t", "alsa", audio_device,
            "synth", "2.0", "sin", "440", "sin", "480",  // 2 seconds tone
            "pad", "0", "4.0",                            // 4 seconds silence
            "repeat", "9999",                             // loop
            "channels", "2",
        ])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("Failed to start ringing tone");

    let mut proc_lock = DIAL_TONE_PROCESS.lock().unwrap();
    *proc_lock = Some(child);
}

/// Stops the background tone (dial or ring)
pub fn stop_dial_tone() {
    let mut proc_lock = DIAL_TONE_PROCESS.lock().unwrap();
    if let Some(child) = proc_lock.as_mut() {
        let _ = child.kill();
    }
    *proc_lock = None;
}

/// Plays a digital ring tone, then a main file, both interruptible on hook
pub fn play_digital_ring_then_mp3(switch: &InputPin, main_path: &str) {
    let device = "hw:0,0";

    // Start digital ringing
    start_ringing_tone(device);

    // Wait while off-hook or until ring timeout (~12 seconds max ring time)
    let mut waited = 0;
    while switch.read() == Level::Low && waited < 120 {
        thread::sleep(Duration::from_millis(100));
        waited += 1;
    }

    // Stop ringing
    stop_dial_tone();

    // If the call was picked up (still off-hook), play the main file
    if switch.read() == Level::Low {
        let _ = play_file_blocking(switch, main_path, device);
    }
}

/// Helper: play MP3 file interruptible on hook
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
            info!("⏹️ On-hook detected. Playback interrupted.");
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

