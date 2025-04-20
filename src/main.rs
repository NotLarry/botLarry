mod cli;
mod gpio;
mod db;
mod keypad;
mod hook; // <-- NEW MODULE

use crate::cli::handle_cli_args;
use crate::gpio::setup_gpio;
use crate::keypad::collect_digits;
use crate::db::init_db;
use crate::hook::handle_hook_state; // <-- USE NEW FUNCTION

use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use std::{env};
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

    handle_hook_state(&gpio, &switch, running.clone(), is_offhook.clone(), &conn);

    println!("👋 Goodbye. GPIO will clean up automatically.");
    Ok(())
}

