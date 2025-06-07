use rppal::gpio::{Gpio, Level, Trigger};
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use std::thread;

pub fn start_coin_watcher(
    gpio: Gpio,
    coin_inserted: Arc<AtomicBool>,
    coin_total: Arc<AtomicBool>, // set to true if at least one coin is detected
) {
    let coins = [
        (19, "Quarter"),
        (13, "Dime"),
        (12, "Nickel"),
    ];

    for (pin, name) in coins.iter() {
        let mut input_pin = gpio.get(*pin).unwrap().into_input_pullup();
        let coin_inserted = Arc::clone(&coin_inserted);
        let coin_total = Arc::clone(&coin_total);

        input_pin.set_async_interrupt(Trigger::FallingEdge, move |_| {
            println!("🪙 {} inserted", name);
            coin_inserted.store(true, Ordering::SeqCst);
            coin_total.store(true, Ordering::SeqCst);
        }).expect("Failed to set interrupt");
    }

    thread::spawn(move || {
        // Keep the thread alive forever
        loop {
            thread::sleep(std::time::Duration::from_secs(1));
        }
    });
}

