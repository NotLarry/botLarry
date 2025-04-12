use rppal::gpio::{Gpio, InputPin, OutputPin, Level};
use std::fs::OpenOptions;
use std::io::Write;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::{thread, time};
use ctrlc;

const SWITCH_PIN: u8 = 16;
const ROW_PINS: [u8; 4] = [26, 13, 6, 5];
const COL_PINS: [u8; 3] = [22, 27, 17];

const KEYPAD: [[char; 3]; 4] = [
    ['1', '2', '3'],
    ['4', '5', '6'],
    ['7', '8', '9'],
    ['*', '0', '#'],
];

fn main() {
    let gpio = Gpio::new().expect("Failed to access GPIO");
    let switch = gpio.get(SWITCH_PIN).unwrap().into_input_pullup();

    let running = Arc::new(AtomicBool::new(true));
    let is_offhook = Arc::new(AtomicBool::new(false));

    // Ctrl+C handler
    {
        let running = running.clone();
        ctrlc::set_handler(move || {
            println!("\nCtrl+C pressed. Exiting...");
            running.store(false, Ordering::SeqCst);
        })
        .expect("Error setting Ctrl-C handler");
    }

    // Main loop
    while running.load(Ordering::SeqCst) {
        let hook_state = switch.read();

        if hook_state == Level::Low && !is_offhook.load(Ordering::SeqCst) {
            println!("📞 Offhook detected. Starting keypad entry...");
            is_offhook.store(true, Ordering::SeqCst);

            while switch.read() == Level::Low && running.load(Ordering::SeqCst) {
                collect_digits(&gpio, &running, &switch);
            }

            println!("📴 Onhook detected. Resetting...");
            is_offhook.store(false, Ordering::SeqCst);
        }

        thread::sleep(time::Duration::from_millis(100));
    }

    println!("👋 Goodbye. GPIO will clean up automatically.");
}

// === KEYPAD HANDLING ===

fn collect_digits(gpio: &Gpio, running: &AtomicBool, switch: &InputPin) {
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

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("digits.txt")
        .expect("Failed to open digits.txt");

    writeln!(file, "{}", digit_string).expect("Failed to write digits");

    println!("🔁 Ready for next call...");
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

