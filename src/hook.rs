use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use std::{thread, time};
use rusqlite::Connection;
use rppal::gpio::{InputPin, OutputPin, Gpio, Level};
use log::{info, warn, error, debug};


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
    coin_total: Arc<AtomicBool>,
) {
    while running.load(Ordering::SeqCst) {
        if is_offhook(switch) && !is_offhook_flag.load(Ordering::SeqCst) {
            info!("📞 Offhook detected. Starting keypad entry...");
            is_offhook_flag.store(true, Ordering::SeqCst);

            while is_offhook(switch) && running.load(Ordering::SeqCst) {
                collect_digits(gpio, &running, switch, conn);
            }

            info!("📴 Onhook detected. Resetting...");
            is_offhook_flag.store(false, Ordering::SeqCst);

            // ✅ Only trigger solenoid if coins were inserted
            if coin_total.load(Ordering::SeqCst) {
                info!("💰 Coins inserted. Triggering coin collection solenoid...");

                if let Ok(mut solenoid) = gpio.get(5).map(|p| p.into_output()) {
                    solenoid.set_high();
                    thread::sleep(time::Duration::from_millis(300)); // Adjust as needed
                    solenoid.set_low();
                } else {
                    error!("⚠️ Failed to access GPIO 6 for solenoid.");
                }

                coin_total.store(false, Ordering::SeqCst);
            }
        }

        thread::sleep(time::Duration::from_millis(100));
    }
}

