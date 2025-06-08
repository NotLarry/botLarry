use rppal::gpio::{Gpio, Level};
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use std::thread;
use std::time::Duration;

pub fn start_coin_watcher(
    gpio: Gpio,
    coin_inserted: Arc<AtomicBool>,
    coin_total: Arc<AtomicBool>,
) {
    let coins = [
        (19, "Dime"),
        (13, "Nickel"),
        (12, "Quarter"),
    ];

    let mut last_state = [Level::High; 3];
    let mut pins = Vec::new();

    for (pin, _) in coins.iter() {
        pins.push(gpio.get(*pin).unwrap().into_input_pullup());
    }

    thread::spawn(move || {
        loop {
            for (i, (_pin, name)) in coins.iter().enumerate() {
                let level = pins[i].read();
                if level == Level::Low && last_state[i] == Level::High {
                    println!("🪙 {} inserted", name);
                    coin_inserted.store(true, Ordering::SeqCst);
                    coin_total.store(true, Ordering::SeqCst);
                }
                last_state[i] = level;
            }
            thread::sleep(Duration::from_millis(50));
        }
    });
}

