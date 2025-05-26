use std::process::{Command, Stdio};
use std::sync::mpsc::{Sender, Receiver};
use std::sync::{Mutex, OnceLock};
use std::thread;

static SENDER: OnceLock<Mutex<Sender<char>>> = OnceLock::new();

pub fn init_tone_thread(audio_device: &'static str) {
    let (tx, rx): (Sender<char>, Receiver<char>) = std::sync::mpsc::channel();

    SENDER.set(Mutex::new(tx)).unwrap_or_else(|_| {
        eprintln!("⚠️ Tone thread already initialized");
    });

    thread::spawn(move || {
        println!("🎧 Tone thread started"); // confirm tone thread launched
        for digit in rx {
            println!("🎵 Playing tone for: {}", digit);
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
    println!("📨 Sending digit to tone thread: {}", digit);
    if let Some(sender) = SENDER.get() {
        let _ = sender.lock().unwrap().send(digit);
    } else {
        eprintln!("❌ Tone thread not initialized");
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

