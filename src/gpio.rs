// gpio.rs
// use std::thread;
// use std::time::Duration;
use rppal::gpio::{Gpio, InputPin};
// const COIN_RELAY_PIN: u8 = 5; // GPIO5 for relay control

pub fn setup_gpio(switch_pin: u8) -> (Gpio, InputPin) {
    let gpio = Gpio::new().expect("Failed to access GPIO");
    let switch = gpio.get(switch_pin).unwrap().into_input_pullup();
    (gpio, switch)
}

// pub fn coin_collect() {
//     let gpio = Gpio::new().unwrap();
//     let pin = gpio.get(COIN_RELAY_PIN).unwrap().into_output();
// 
//     pin.set_high(); // Activate relay
//      thread::sleep(Duration::from_millis(500)); // Shortened to 0.5s
//     pin.set_low(); // Deactivate relay
// }
