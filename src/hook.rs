use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use std::{thread, time};
use rusqlite::Connection;
use rppal::gpio::{InputPin, Gpio, Level};

use crate::keypad::collect_digits;

/// Debounced check for off-hook state.
pub fn is_offhook(switch: &InputPin) -> bool {
    let mut count = 0;
    for _ in 0..5 {
        if switch.read() == Level::Low {
            count += 1;
        }
        thread::sleep(time::Duration::from_millis(5));
    }
    count >= 4
}

pub fn handle_hook_state(
    gpio: &Gpio,
    switch: &InputPin,
    running: Arc<AtomicBool>,
    is_offhook_flag: Arc<AtomicBool>,
    conn: &Connection,
) {
    while running.load(Ordering::SeqCst) {
        if is_offhook(switch) && !is_offhook_flag.load(Ordering::SeqCst) {
            println!("📞 Offhook detected. Starting keypad entry...");
            is_offhook_flag.store(true, Ordering::SeqCst);

            collect_digits(gpio, &running, switch, conn);


            println!("📴 Onhook detected. Resetting...");
            is_offhook_flag.store(false, Ordering::SeqCst);
        }

        thread::sleep(time::Duration::from_millis(100));
    }
}

