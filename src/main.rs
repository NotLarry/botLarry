mod cli;
mod gpio;
mod db;
mod keypad;

use crate::cli::handle_cli_args;
use crate::gpio::setup_gpio;
use crate::keypad::collect_digits;
use crate::db::init_db;

use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use std::{thread, time};
use std::env;
use rppal::gpio::Level;
use ctrlc;

const SWITCH_PIN: u8 = 16;

fn main() -> rusqlite::Result<()> {
    let args: Vec<String> = env::args().collect();

    let (gpio, switch) = setup_gpio(SWITCH_PIN);
    let conn = init_db()?;

    if handle_cli_args(&args, &conn) {
        return Ok(());
    }

    let running = Arc::new(AtomicBool::new(true));
    let is_offhook = Arc::new(AtomicBool::new(false));

    {
        let running = running.clone();
        ctrlc::set_handler(move || {
            println!("\nCtrl+C pressed. Exiting...");
            running.store(false, Ordering::SeqCst);
        }).expect("Error setting Ctrl-C handler");
    }

    while running.load(Ordering::SeqCst) {
        let hook_state = switch.read();

        if hook_state == Level::Low && !is_offhook.load(Ordering::SeqCst) {
            println!("📞 Offhook detected. Starting keypad entry...");
            is_offhook.store(true, Ordering::SeqCst);

            while switch.read() == Level::Low && running.load(Ordering::SeqCst) {
                collect_digits(&gpio, &running, &switch, &conn);
            }

            println!("📴 Onhook detected. Resetting...");
            is_offhook.store(false, Ordering::SeqCst);
        }

        thread::sleep(time::Duration::from_millis(100));
    }

    println!("👋 Goodbye. GPIO will clean up automatically.");
    Ok(())
}

