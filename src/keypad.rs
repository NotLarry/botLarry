use rppal::gpio::{Gpio, InputPin, OutputPin, Level};
use std::sync::atomic::{AtomicBool, Ordering};
use std::{thread, time::Duration};
use rusqlite::{params, Connection};
use rusqlite::OptionalExtension;
use chrono::Local;

use crate::playback::{start_dial_tone, stop_dial_tone, play_digital_ring_then_mp3};
use crate::tone::play_dtmf_tone;
use crate::recording::handle_unknown_number;

// Pin mappings
pub const ROW_PINS: [u8; 4] = [16, 25, 24, 23];
pub const COL_PINS: [u8; 3] = [22, 27, 17];

const KEYPAD: [[char; 3]; 4] = [
    ['1', '2', '3'],
    ['4', '5', '6'],
    ['7', '8', '9'],
    ['*', '0', '#'],
];

pub fn get_key(rows: &Vec<InputPin>, cols: &mut Vec<OutputPin>) -> Option<char> {
    for (col_idx, col) in cols.iter_mut().enumerate() {
        col.set_low();
        for (row_idx, row) in rows.iter().enumerate() {
            if row.read() == Level::Low {
                thread::sleep(Duration::from_millis(50));
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

pub fn wait_for_keypress(rows: &Vec<InputPin>, cols: &mut Vec<OutputPin>) -> char {
    loop {
        if let Some(key) = get_key(rows, cols) {
            return key;
        }
        thread::sleep(Duration::from_millis(50));
    }
}

pub fn collect_digits(gpio: &Gpio, running: &AtomicBool, switch: &InputPin, conn: &Connection) {
    let rows: Vec<InputPin> = ROW_PINS
        .iter()
        .map(|&pin| gpio.get(pin).unwrap().into_input_pullup())
        .collect();

    let mut cols: Vec<OutputPin> = COL_PINS
        .iter()
        .map(|&pin| {
            let mut col = gpio.get(pin).unwrap().into_output();
            col.set_high();
            col
        })
        .collect();

    let mut digits = Vec::new();
    info!("⌨️  Waiting for 10 digits...");

    start_dial_tone("hw:0,0");

    while digits.len() < 10 {
        if !running.load(Ordering::SeqCst) || switch.read() == Level::High {
            info!("❌ Digit entry canceled (on-hook or Ctrl+C).");
            stop_dial_tone();
            return;
        }

        if let Some(key) = get_key(&rows, &mut cols) {
            if key.is_ascii_digit() {
                if digits.is_empty() {
                    stop_dial_tone();
                }
                play_dtmf_tone(key);
                digits.push(key);
                info!("✅ Key pressed: {}", key);
                thread::sleep(Duration::from_millis(300));
            }
        }

        thread::sleep(Duration::from_millis(50));
    }

    let digit_string: String = digits.iter().collect();
    info!("📋 Digits recorded: {}", digit_string);

    let (areacode, phonenumber) = digit_string.split_at(3);

    let mut stmt = conn.prepare(
        "SELECT recording_path FROM calls WHERE areacode = ?1 AND phonenumber = ?2"
    ).expect("Failed to prepare SELECT");

    let existing: Option<String> = stmt
        .query_row(params![areacode, phonenumber], |row| row.get(0))
        .optional()
        .expect("Failed to query DB");

    match existing {
        Some(path) => {
            info!("📀 Number already logged. Recording path: {}", path);
            info!("▶️ Playing recording: {}", path);
            play_digital_ring_then_mp3(switch, &path);
            info!("✅ Playback finished.");
        }
        None => {
            if handle_unknown_number(&rows, &mut cols, switch, &digit_string) {
                let recording_path = format!("/botLarry/recordings/{}/{}.mp3", areacode, digit_string);
                conn.execute(
                    "INSERT INTO calls (areacode, phonenumber, recording_path) VALUES (?1, ?2, ?3)",
                    params![areacode, phonenumber, &recording_path],
                ).expect("Failed to insert call record");

                info!("💾 New call logged. Recording path: {}", recording_path);
            } else {
                info!("⚠️ Recording aborted — not logging call.");
            }
        }
    }
}

