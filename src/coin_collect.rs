use rppal::gpio::Gpio;
use std::thread::sleep;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let gpio = Gpio::new()?;
    let mut pin = gpio.get(5)?.into_output();

    println!("Relay ON for 2 seconds...");
    pin.set_high(); // Turn relay on (active HIGH)
    sleep(Duration::from_secs(2));

    println!("Relay OFF.");
    pin.set_low(); // Turn relay off
    Ok(())
}
