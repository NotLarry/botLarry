// src/tone.rs
use std::f32::consts::PI;
use std::io::Write;
use std::process::{Command, Stdio};
use std::thread;

const SAMPLE_RATE: u32 = 48000;
const DURATION_SECS: f32 = 0.2;

pub fn play_dtmf_tone(digit: char, device: &str) {
    if let Some((f1, f2)) = dtmf_freqs(digit) {
        let samples = generate_tone_samples(f1, f2, DURATION_SECS);
        play_raw_audio(&samples, device);
    }
}

fn dtmf_freqs(digit: char) -> Option<(f32, f32)> {
    match digit {
        '1' => Some((697.0, 1209.0)),
        '2' => Some((697.0, 1336.0)),
        '3' => Some((697.0, 1477.0)),
        '4' => Some((770.0, 1209.0)),
        '5' => Some((770.0, 1336.0)),
        '6' => Some((770.0, 1477.0)),
        '7' => Some((852.0, 1209.0)),
        '8' => Some((852.0, 1336.0)),
        '9' => Some((852.0, 1477.0)),
        '0' => Some((941.0, 1336.0)),
        '*' => Some((941.0, 1209.0)),
        '#' => Some((941.0, 1477.0)),
        _ => None,
    }
}

fn generate_tone_samples(freq1: f32, freq2: f32, duration_secs: f32) -> Vec<i16> {
    let num_samples = (SAMPLE_RATE as f32 * duration_secs) as usize;
    let mut samples = Vec::with_capacity(num_samples);

    for n in 0..num_samples {
        let t = n as f32 / SAMPLE_RATE as f32;
        let sample = (0.5 * (f32::sin(2.0 * PI * freq1 * t) + f32::sin(2.0 * PI * freq2 * t)) * i16::MAX as f32) as i16;
        samples.push(sample);
    }

    samples
}

fn play_raw_audio(samples: &[i16], device: &str) {
    let mut child = Command::new("aplay")
        .args(["-f", "S16_LE", "-c", "2", "-r", &SAMPLE_RATE.to_string(), "-D", device])
        .stdin(Stdio::piped())
        .spawn()
        .expect("Failed to start aplay");

    if let Some(stdin) = child.stdin.as_mut() {
        let bytes = unsafe {
            std::slice::from_raw_parts(
                samples.as_ptr() as *const u8,
                samples.len() * std::mem::size_of::<i16>(),
            )
        };
        stdin.write_all(bytes).ok();
    }

    let _ = child.wait();
}

