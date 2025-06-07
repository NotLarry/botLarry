use rppal::gpio::{Gpio, Trigger};
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use std::thread;
use std::time::Duration;

pub fn start_coin_watcher(
    gpio: Gpio,
    coin_inserted: Arc<AtomicBool>,
    coin_total: Arc<AtomicBool>,
) {
    // Use 'static strings so they can be safely moved into the closure
    let coin_defs: [(u8, &'static str); 3] = [
        (19, "Quarter"),
        (13, "Dime"),
        (12, "Nickel"),
    ];

    for (pin, name) in coin_defs {
        let mut input_pin = gpio.get(pin).unwrap().into_input_pullup();
        let coin_inserted = Arc::clone(&coin_inserted);
        let coin_total = Arc::clone(&coin_total);

        input_pin.set_async_interrupt(
            Trigger::FallingEdge,
            Some(Duration::from_millis(50)), // debounce
            move |_| {
                println!("🪙 {} inserted", name);
                coin_inserted.store(true, Ordering::SeqCst);
                coin_total.store(true, Ordering::SeqCst);
            }
        ).expect("Failed to set interrupt");
    }

    // Keep the thread alive so interrupts stay active
    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_secs(1));
        }
    });
}

