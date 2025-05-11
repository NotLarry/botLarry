use std::time::Duration;
use std::sync::{Arc, Mutex};
use alsa::{self, pcm::{Format, HwParams, Access}};
use alsa::pcm::{PCM, HwParamsSet, PCMAccess, PCMFormat};

const SAMPLE_RATE: u32 = 8000; // 8kHz sample rate for DTMF

// DTMF frequencies for digits 0-9 and symbols * and #
const DTMF_TONES: [(f64, f64); 12] = [
    (697.0, 1209.0), // 1
    (697.0, 1336.0), // 2
    (697.0, 1477.0), // 3
    (770.0, 1209.0), // 4
    (770.0, 1336.0), // 5
    (770.0, 1477.0), // 6
    (852.0, 1209.0), // 7
    (852.0, 1336.0), // 8
    (852.0, 1477.0), // 9
    (941.0, 1209.0), // 0
    (941.0, 1336.0), // *
    (941.0, 1477.0), // #
];

pub fn play_dtmf_tone(digit: char, duration_ms: u64) {
    let (low_freq, high_freq) = match digit {
        '1' => DTMF_TONES[0],
        '2' => DTMF_TONES[1],
        '3' => DTMF_TONES[2],
        '4' => DTMF_TONES[3],
        '5' => DTMF_TONES[4],
        '6' => DTMF_TONES[5],
        '7' => DTMF_TONES[6],
        '8' => DTMF_TONES[7],
        '9' => DTMF_TONES[8],
        '0' => DTMF_TONES[9],
        '*' => DTMF_TONES[10],
        '#' => DTMF_TONES[11],
        _ => return,
    };

    // Generate the tone waveform
    let low_wave = generate_sine_wave(low_freq, duration_ms);
    let high_wave = generate_sine_wave(high_freq, duration_ms);

    // Combine the two tones (this is mono, so add the waves together)
    let tone = combine_tones(&low_wave, &high_wave);

    // Play the generated tone via ALSA
    play_pcm(tone);
}

fn generate_sine_wave(frequency: f64, duration_ms: u64) -> Vec<i16> {
    let num_samples = (SAMPLE_RATE as f64 * duration_ms as f64 / 1000.0) as usize;
    let mut samples = Vec::with_capacity(num_samples);
    let two_pi = 2.0 * std::f64::consts::PI;
    for i in 0..num_samples {
        let sample = (two_pi * frequency * i as f64 / SAMPLE_RATE as f64).sin();
        samples.push((sample * std::i16::MAX as f64) as i16);
    }
    samples
}

fn combine_tones(low_wave: &[i16], high_wave: &[i16]) -> Vec<i16> {
    let mut combined = Vec::with_capacity(low_wave.len());
    for (low, high) in low_wave.iter().zip(high_wave.iter()) {
        let combined_sample = low.saturating_add(*high);
        combined.push(combined_sample);
    }
    combined
}

fn play_pcm(tone: Vec<i16>) {
    let pcm = PCM::open("default", alsa::Direction::Playback, false)
        .expect("Failed to open PCM device");

    let hwp = HwParams::any(&pcm).expect("Failed to get hardware params");
    hwp.set_rate(SAMPLE_RATE).expect("Failed to set rate");
    hwp.set_channels(1).expect("Failed to set channels");
    hwp.set_format(PCMFormat::S16LE).expect("Failed to set format");

    let buffer = tone.iter().map(|&sample| sample as i8).collect::<Vec<i8>>();

    let pcm = pcm.prepare().expect("Failed to prepare PCM");

    pcm.writei(&buffer)
        .expect("Failed to write PCM data");
}


