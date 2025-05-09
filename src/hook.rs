use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use std::{thread, time};
use rusqlite::Connection;
use rppal::gpio::{InputPin, Gpio, Level};
use crate::keypad::collect_digits;
// use crate::coin_collect;

pub fn handle_hook_state(
    gpio: &Gpio,
    switch: &InputPin,
    running: Arc<AtomicBool>,
    is_offhook: Arc<AtomicBool>,
    conn: &Connection,
) {
    while running.load(Ordering::SeqCst) {
        let hook_state = switch.read();

        if hook_state == Level::Low && !is_offhook.load(Ordering::SeqCst) {
            println!("📞 Offhook detected. Starting keypad entry...");
            is_offhook.store(true, Ordering::SeqCst);

            while switch.read() == Level::Low && running.load(Ordering::SeqCst) {
                collect_digits(gpio, &running, switch, conn);
            }

            println!("📴 Onhook detected. Resetting...");
            is_offhook.store(false, Ordering::SeqCst);
        }

        thread::sleep(time::Duration::from_millis(100));
    }
}

