mod cli;
mod gpio;
mod db;
mod keypad;
mod hook;
mod playback;
mod tone;
mod recording;
mod coin_counter; // ← Include your efficient coin watcher
mod web;
mod audio;

use crate::cli::handle_cli_args;
use crate::gpio::setup_gpio;
use crate::db::init_db;
use crate::hook::handle_hook_state;
use crate::coin_counter::start_coin_watcher;

use std::sync::{atomic::{AtomicBool, Ordering}};
use std::env;
use ctrlc;
use rppal::gpio::Gpio;
use crate::playback::setup_volume_button;
use simplelog::*;
use std::fs::File;
use log::info;
use std::sync::Arc;
use tokio::sync::Mutex;


const SWITCH_PIN: u8 = 26;

fn main() -> db::Result<()> {

    CombinedLogger::init(vec![
        WriteLogger::new(
            LevelFilter::Info,
            Config::default(),
            File::create("/var/log/botLarry.log").expect("Could not create log file"),
        ),
    ]).expect("Logger failed to initialize");

    info!("🔧 botLarry starting up...");

    tone::init_tone_thread("hw:0,0");
    info!("✅ init_tone_thread called from main");

    let conn = Arc::new(Mutex::new(init_db()?));

        // Clone for use in the web server thread
            web::spawn_web_server_thread(conn.clone());

    let gpio = Gpio::new().unwrap();
    setup_volume_button(&gpio);
    
    let args: Vec<String> = env::args().collect();

    let (gpio, switch) = setup_gpio(SWITCH_PIN);
    let conn = init_db()?;

    if handle_cli_args(&args, &conn) {
        return Ok(());
    }

    let running = Arc::new(AtomicBool::new(true));
    let is_offhook = Arc::new(AtomicBool::new(false));
    let coin_total = Arc::new(AtomicBool::new(false)); // Shared flag

    {
        let running = running.clone();
        ctrlc::set_handler(move || {
            info!("\nCtrl+C pressed. Exiting...");
            running.store(false, Ordering::SeqCst);
        }).expect("Error setting Ctrl-C handler");
    }

    // 🪙 Start the interrupt-based coin watcher thread
    start_coin_watcher(gpio.clone(), Arc::new(AtomicBool::new(false)), coin_total.clone());

    // ☎️ Start hook handling, which will read coin_total
    handle_hook_state(&gpio, &switch, running.clone(), is_offhook.clone(), &conn, coin_total.clone());

    info!("👋 Goodbye. GPIO will clean up automatically.");
    Ok(())
}

