use std::sync::{Arc, Mutex};
use rppal::gpio::{Gpio, InputPin, Trigger};
use std::thread;
use std::time::Duration;

#[derive(Clone, Copy)]
pub enum VolumeLevel {
    Low,
    Medium,
    High,
}

impl VolumeLevel {
    pub fn as_percentage(&self) -> u8 {
        match self {
            VolumeLevel::Low => 30,
            VolumeLevel::Medium => 60,
            VolumeLevel::High => 100,
        }
    }

    pub fn next(self) -> Self {
        match self {
            VolumeLevel::Low => VolumeLevel::Medium,
            VolumeLevel::Medium => VolumeLevel::High,
            VolumeLevel::High => VolumeLevel::Low,
        }
    }
}

pub fn setup_volume_button(shared_volume: Arc<Mutex<VolumeLevel>>) {
    let gpio = Gpio::new().expect("Failed to access GPIO");
    let mut pin = gpio.get(6).expect("Failed to get GPIO6").into_input_pulldown();

    let shared = Arc::clone(&shared_volume);

    pin.set_async_interrupt(Trigger::RisingEdge, move |_| {
        let mut volume = shared.lock().unwrap();
        *volume = volume.next();
        println!("Volume changed to: {:?}", volume.as_percentage());
        // Optionally set system volume here, depending on playback method
    }).expect("Failed to set up interrupt");

    // Keep thread alive to maintain interrupt
    thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(1));
    });
}

