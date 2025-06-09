mod cli;
mod gpio;
mod db;
mod keypad;
mod hook;
mod playback;
mod tone;
mod recording;
mod coin_counter;
mod web; // Make sure you have a `mod web;` if it's in src/web.rs

use crate::cli::handle_cli_args;
use crate::gpio::setup_gpio;
use crate::db::init_db;
use crate::hook::handle_hook_state;
use crate::recording::handle_unknown_number;
use crate::coin_counter::start_coin_watcher;
use crate::playback::setup_volume_button;
use crate::web::create_router;

use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use std::env;
use ctrlc;
use rppal::gpio::Gpio;
use rusqlite::Connection;
use tokio::sync::Mutex;
use log::info;

const SWITCH_PIN: u8 = 26;

#[tokio::main]
async fn main() -> db::Result<()> {
    tone::init_tone_thread("hw:0,0");
    println!("✅ init_tone_thread called from main");

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
            println!("\nCtrl+C pressed. Exiting...");
            running.store(false, Ordering::SeqCst);
        }).expect("Error setting Ctrl-C handler");
    }

    // 🪙 Start the interrupt-based coin watcher thread
    start_coin_watcher(gpio.clone(), Arc::new(AtomicBool::new(false)), coin_total.clone());

    // 🌐 Start web server on a background task
    tokio::spawn(async {
        run_web_server().await;
    });

    // ☎️ Start hook handling, which will read coin_total
    handle_hook_state(&gpio, &switch, running.clone(), is_offhook.clone(), &conn, coin_total.clone());

    println!("👋 Goodbye. GPIO will clean up automatically.");
    Ok(())
}

async fn run_web_server() {
    let conn = Arc::new(Mutex::new(Connection::open("data/calls.db").unwrap()));
    let app = create_router(conn);
    let addr = ([0, 0, 0, 0], 3000).into();
    info!("🌐 Web server running at http://{}/", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

