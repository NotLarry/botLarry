use std::process::{Command, Stdio};
use std::sync::mpsc::{Sender, Receiver};
use std::sync::{Mutex, OnceLock};
use std::thread;
//use once_cell::sync::Lazy;
use log::{info, error};
use crate::audio::SOX_GAIN_DB;

static SENDER: OnceLock<Mutex<Sender<char>>> = OnceLock::new();
//static DIAL_TONE_PROCESS: Lazy<Mutex<Option<Child>>> = Lazy::new(|| Mutex::new(None));

//pub fn start_dial_tone(audio_device: &str) {
//    let mut lock = DIAL_TONE_PROCESS.lock().unwrap();
//
//    if lock.is_some() {
//        return; // Already playing
//    }
//
//    let child = Command::new("sox")
//        .args([
//            "-n",
//            "-t", "alsa",
//            audio_device,
//            "synth", "-",  // Continuous
//            "sin", "350",
//            "sin", "440",
//        ])
//        .spawn()
//        .expect("Failed to start dial tone");
//
//    *lock = Some(child);
//}

//pub fn stop_dial_tone() {
//    let mut lock = DIAL_TONE_PROCESS.lock().unwrap();
//    if let Some(mut child) = lock.take() {
//        let _ = child.kill();
//        let _ = child.wait();
//    }
//}



pub fn init_tone_thread(audio_device: &'static str) {
    let (tx, rx): (Sender<char>, Receiver<char>) = std::sync::mpsc::channel();

    SENDER.set(Mutex::new(tx)).unwrap_or_else(|_| {
        error!("⚠️ Tone thread already initialized");
    });

    thread::spawn(move || {
        info!("🎧 Tone thread started"); // confirm tone thread launched
        for digit in rx {
            info!("🎵 Playing tone for: {}", digit);
            let dtmf = match digit {
                '0'..='9' | '*' | '#' => digit.to_string(),
                _ => continue,
            };

let mut child = Command::new("sox")
    .args([
        "-n",
        "-c", "2",                  // 👈 Set to 1 for mono output
        "-t", "alsa",
        audio_device,
        "synth", "0.2",
        "sin", &dtmf_freq1(&dtmf),
        "sin", &dtmf_freq2(&dtmf),
        "gain", SOX_GAIN_DB,
    ])
    .stdout(Stdio::null())
    .stderr(Stdio::null())
    .spawn()
    .expect("Failed to spawn sox for tone");


            let _ = child.wait();
        }
    });
}

pub fn play_dtmf_tone(digit: char) {
    info!("📨 Sending digit to tone thread: {}", digit);
    if let Some(sender) = SENDER.get() {
        let _ = sender.lock().unwrap().send(digit);
    } else {
        error!("❌ Tone thread not initialized");
    }
}

// Dummy freq calculator — replace with real values if needed
fn dtmf_freq1(d: &str) -> String {
    match d {
        "1" | "2" | "3" => "697",
        "4" | "5" | "6" => "770",
        "7" | "8" | "9" => "852",
        "*" | "0" | "#" => "941",
        _ => "0",
    }.to_string()
}

fn dtmf_freq2(d: &str) -> String {
    match d {
        "1" | "4" | "7" | "*" => "1209",
        "2" | "5" | "8" | "0" => "1336",
        "3" | "6" | "9" | "#" => "1477",
        _ => "0",
    }.to_string()
}

