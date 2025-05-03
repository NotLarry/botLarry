use std::f32::consts::PI;
use std::io::Cursor;
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

use rodio::{OutputStream, OutputStreamHandle, Sink, Source};

/// Create a sine wave signal generator
fn sine_wave(freq: f32, duration_secs: f32, sample_rate: u32) -> Vec<f32> {
    let samples_count = (duration_secs * sample_rate as f32) as usize;
    (0..samples_count)
        .map(|i| {
            let t = i as f32 / sample_rate as f32;
            (2.0 * PI * freq * t).sin()
        })
        .collect()
}

/// Mix two tones into one
fn mix_tones(tone1: &[f32], tone2: &[f32]) -> Vec<f32> {
    tone1.iter()
        .zip(tone2.iter())
        .map(|(a, b)| (a + b) / 2.0)
        .collect()
}

/// Convert mono f32 buffer to rodio Source
fn buffer_to_source(buffer: Vec<f32>, sample_rate: u32) -> impl Source<Item = f32> + Send {
    rodio::buffer::SamplesBuffer::new(1, sample_rate, buffer)
}

/// Play tone to the default output device (handset earpiece)
fn play_dial_and_offhook_tone() -> Result<(), Box<dyn std::error::Error>> {
    let sample_rate = 44100;

    // Dial tone: 350 Hz + 440 Hz
    let dial_350 = sine_wave(350.0, 20.0, sample_rate);
    let dial_440 = sine_wave(440.0, 20.0, sample_rate);
    let dial_tone = mix_tones(&dial_350, &dial_440);

    // Off-hook tone: mix of four frequencies
    let duration = 2.0;
    let offhook_1400 = sine_wave(1400.0, duration, sample_rate);
    let offhook_2060 = sine_wave(2060.0, duration, sample_rate);
    let offhook_2450 = sine_wave(2450.0, duration, sample_rate);
    let offhook_2600 = sine_wave(2600.0, duration, sample_rate);
    let mut offhook_tone = offhook_1400.clone();
    for i in 0..offhook_tone.len() {
        offhook_tone[i] = (offhook_1400[i] + offhook_2060[i] + offhook_2450[i] + offhook_2600[i]) / 4.0;
    }

    // Loop off-hook tone indefinitely
    let offhook_loop: Vec<f32> = offhook_tone.repeat(100);

    // Set up audio stream
    let (_stream, stream_handle) = OutputStream::try_default()?;
    let sink = Sink::try_new(&stream_handle)?;

    // Play dial tone first
    sink.append(buffer_to_source(dial_tone, sample_rate));
    sink.sleep_until_end();

    // After 20 seconds, play off-hook tone in loop
    let sink2 = Sink::try_new(&stream_handle)?;
    sink2.append(buffer_to_source(offhook_loop, sample_rate));
    sink2.sleep_until_end(); // Optional: Replace with break condition in real use

    Ok(())
}

