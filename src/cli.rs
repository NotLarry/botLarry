// src/cli.rs
use rusqlite::Connection;
use crate::db::show_call_logs;
use log::{info, warn, error, debug};


pub fn handle_cli_args(args: &[String], conn: &Connection) -> bool {
    if args.len() > 1 {
        match args[1].as_str() {
            "--show-calls" => {
                show_call_logs(conn).expect("Failed to show call logs");
                return true;
            }
            "-V" | "--version" => {
                println!("botLarry version {}", env!("CARGO_PKG_VERSION"));
                return true;
            }
            _ => {
                info!("❓ Unknown option: {}", args[1]);
                info!("📌 Try: --show-calls");
                return true;
            }
        }
    }
    false
}


