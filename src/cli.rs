// src/cli.rs
use rusqlite::Connection;
use crate::db::show_call_logs;

pub fn handle_cli_args(args: &[String], conn: &Connection) -> bool {
    if args.len() > 1 {
        match args[1].as_str() {
            "--show-calls" => {
                show_call_logs(conn).expect("Failed to show call logs");
                return true;
            }
            // Future CLI commands like `--add-note` can go here.
            _ => {
                println!("❓ Unknown option: {}", args[1]);
                println!("📌 Try: --show-calls");
                return true;
            }
        }
    }
    false
}

