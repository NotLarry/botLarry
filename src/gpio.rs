// gpio.rs
use rppal::gpio::{Gpio, InputPin};

pub fn setup_gpio(switch_pin: u8) -> (Gpio, InputPin) {
    let gpio = Gpio::new().expect("Failed to access GPIO");
    let switch = gpio.get(switch_pin).unwrap().into_input_pullup();
    (gpio, switch)
}

