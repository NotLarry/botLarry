// gpio.rs
use rppal::gpio::{Gpio, InputPin};

pub fn setup_gpio(switch_pin: u8) -> (Gpio, InputPin) {
    let gpio = Gpio::new().expect("Failed to access GPIO");
    let switch = gpio.get(switch_pin).unwrap().into_input_pullup();
    (gpio, switch)
}

pub fn coin_collect() {
    let pin = Gpio::new().unwrap().get(5).unwrap().into_output();
    pin.set_high();
    std::thread::sleep(std::time::Duration::from_millis(500));
    pin.set_low();
}
