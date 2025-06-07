mod cli;
mod gpio;
mod db;
mod keypad;
mod hook;
mod playback;
mod tone;
mod recording;
mod coin_counter; // ← include the new module

use crate::cli::handle_cli_args;
use crate::gpio::setup_gpio;
use crate::db::init_db;
use crate::hook::handle_hook_state;
use crate::recording::handle_unknown_number;
use crate::coin_counter::start_coin_counter_thread;

use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use std::{env, thread};
use ctrlc;

const SWITCH_PIN: u8 = 26;

fn main() -> db::Result<()> {
    tone::init_tone_thread("hw:0,0");
    println!("✅ init_tone_thread called from main");

    let args: Vec<String> = env::args().collect();

    let (gpio, switch) = setup_gpio(SWITCH_PIN);
    let conn = init_db()?;

    if handle_cli_args(&args, &conn) {
        return Ok(());
    }

    let running = Arc::new(AtomicBool::new(true));
    let is_offhook = Arc::new(AtomicBool::new(false));
    let coin_total = Arc::new(AtomicBool::new(false)); // ← shared flag for coins

    {
        let running = running.clone();
        ctrlc::set_handler(move || {
            println!("\nCtrl+C pressed. Exiting...");
            running.store(false, Ordering::SeqCst);
        }).expect("Error setting Ctrl-C handler");
    }

    // 🪙 Start the coin detection thread
    start_coin_counter_thread(gpio.clone(), running.clone(), coin_total.clone());

    // ☎️ Start the main hook detection loop
    handle_hook_state(&gpio, &switch, running.clone(), is_offhook.clone(), &conn, coin_total.clone());

    println!("👋 Goodbye. GPIO will clean up automatically.");
    Ok(())
}

