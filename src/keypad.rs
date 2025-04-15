// keypad.rs
use rppal::gpio::{Gpio, InputPin, OutputPin, Level};
use std::sync::{atomic::{AtomicBool, Ordering}};
use std::{thread, time};
use rusqlite::{params, Connection};
use rusqlite::OptionalExtension;

const ROW_PINS: [u8; 4] = [26, 13, 6, 5];
const COL_PINS: [u8; 3] = [22, 27, 17];

const KEYPAD: [[char; 3]; 4] = [
    ['1', '2', '3'],
    ['4', '5', '6'],
    ['7', '8', '9'],
    ['*', '0', '#'],
];

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
    println!("⌨️  Waiting for 10 digits...");

    while digits.len() < 10 {
        if !running.load(Ordering::SeqCst) || switch.read() == Level::High {
            println!("❌ Digit entry canceled (on-hook or Ctrl+C).");
            return;
        }

        if let Some(key) = get_key(&rows, &mut cols) {
            if key.is_ascii_digit() {
                digits.push(key);
                println!("✅ Key pressed: {}", key);
                thread::sleep(time::Duration::from_millis(300));
            }
        }

        thread::sleep(time::Duration::from_millis(50));
    }

    let digit_string: String = digits.iter().collect();
    println!("📋 Digits recorded: {}", digit_string);

    let (areacode, phonenumber) = digit_string.split_at(3);
    let recording_path = format!("recordings/{}.mp3", digit_string);

    let mut stmt = conn.prepare(
        "SELECT recording_path FROM calls WHERE areacode = ?1 AND phonenumber = ?2"
    ).expect("Failed to prepare SELECT");

    let existing: Option<String> = stmt
        .query_row(params![areacode, phonenumber], |row| row.get(0))
        .optional()
        .expect("Failed to query DB");

    match existing {
        Some(path) => {
            println!("📀 Number already logged. Recording path: {}", path);
        }
        None => {
            conn.execute(
                "INSERT INTO calls (areacode, phonenumber, recording_path) VALUES (?1, ?2, ?3)",
                params![areacode, phonenumber, recording_path],
            ).expect("Failed to insert call record");

            println!("💾 New call logged. Recording path: {}", recording_path);
        }
    }
}

fn get_key(rows: &Vec<InputPin>, cols: &mut Vec<OutputPin>) -> Option<char> {
    for (col_idx, col) in cols.iter_mut().enumerate() {
        col.set_low();
        for (row_idx, row) in rows.iter().enumerate() {
            if row.read() == Level::Low {
                thread::sleep(time::Duration::from_millis(50));
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

