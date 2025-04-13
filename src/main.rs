use rppal::gpio::{Gpio, InputPin, OutputPin, Level};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::{thread, time};
use ctrlc;
use rusqlite::{params, Connection, Result};
use rusqlite::OptionalExtension;
use std::env;

const SWITCH_PIN: u8 = 16;
const ROW_PINS: [u8; 4] = [26, 13, 6, 5];
const COL_PINS: [u8; 3] = [22, 27, 17];

const KEYPAD: [[char; 3]; 4] = [
    ['1', '2', '3'],
    ['4', '5', '6'],
    ['7', '8', '9'],
    ['*', '0', '#'],
];

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    let gpio = Gpio::new().expect("Failed to access GPIO");
    let switch = gpio.get(SWITCH_PIN).unwrap().into_input_pullup();

    let conn = Connection::open("calls.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS calls (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            areacode TEXT NOT NULL,
            phonenumber TEXT NOT NULL,
            recording_path TEXT NOT NULL,
            timestamp DATETIME DEFAULT CURRENT_TIMESTAMP
        )",
        [],
    )?;

    // If --show-calls flag is present, display logs and exit
    if args.len() > 1 && args[1] == "--show-calls" {
        show_call_logs(&conn)?;
        return Ok(());
    }

    let running = Arc::new(AtomicBool::new(true));
    let is_offhook = Arc::new(AtomicBool::new(false));

    {
        let running = running.clone();
        ctrlc::set_handler(move || {
            println!("\nCtrl+C pressed. Exiting...");
            running.store(false, Ordering::SeqCst);
        })
        .expect("Error setting Ctrl-C handler");
    }

    while running.load(Ordering::SeqCst) {
        let hook_state = switch.read();

        if hook_state == Level::Low && !is_offhook.load(Ordering::SeqCst) {
            println!("📞 Offhook detected. Starting keypad entry...");
            is_offhook.store(true, Ordering::SeqCst);

            while switch.read() == Level::Low && running.load(Ordering::SeqCst) {
                collect_digits(&gpio, &running, &switch, &conn);
            }

            println!("📴 Onhook detected. Resetting...");
            is_offhook.store(false, Ordering::SeqCst);
        }

        thread::sleep(time::Duration::from_millis(100));
    }

    println!("👋 Goodbye. GPIO will clean up automatically.");
    Ok(())
}

fn collect_digits(gpio: &Gpio, running: &AtomicBool, switch: &InputPin, conn: &Connection) {
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
                thread::sleep(time::Duration::from_millis(300)); // debounce
            }
        }

        thread::sleep(time::Duration::from_millis(50)); // scan delay
    }

    let digit_string: String = digits.iter().collect();
    println!("📋 Digits recorded: {}", digit_string);

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
            println!("📀 Number already logged. Recording path: {}", path);
        }
        None => {
            let recording_path = format!("recordings/{}.mp3", digit_string);
            conn.execute(
                "INSERT INTO calls (areacode, phonenumber, recording_path) VALUES (?1, ?2, ?3)",
                params![areacode, phonenumber, recording_path],
            )
            .expect("Failed to insert call record");

            println!("💾 New call logged. Recording path: {}", recording_path);
        }
    }
}

fn get_key(rows: &Vec<InputPin>, cols: &mut Vec<OutputPin>) -> Option<char> {
    for (col_idx, col) in cols.iter_mut().enumerate() {
        col.set_low();
        for (row_idx, row) in rows.iter().enumerate() {
            if row.read() == Level::Low {
                thread::sleep(time::Duration::from_millis(50)); // debounce
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

fn show_call_logs(conn: &Connection) -> Result<()> {
    let mut stmt = conn.prepare(
        "SELECT id, areacode, phonenumber, recording_path, timestamp FROM calls ORDER BY timestamp DESC"
    )?;

    let call_iter = stmt.query_map([], |row| {
        Ok((
            row.get::<_, i32>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, String>(2)?,
            row.get::<_, String>(3)?,
            row.get::<_, String>(4)?,
        ))
    })?;

    println!("\n 📄 Call Log:");
    for call in call_iter {
        let (id, areacode, number, recording, timestamp) = call?;
        println!(
            "[{}] ({}) {} => {} at {}",
            id, areacode, number, recording, timestamp
        );
    }

    Ok(())
}

