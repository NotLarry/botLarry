mod cli;
mod gpio;
mod db;
mod keypad;
mod hook;
mod playback;
mod tone;
//mod coin_collect;
//mod tone;
//mod coin;

//use crate::coin::CoinInputs;
use crate::cli::handle_cli_args;
use crate::gpio::setup_gpio;
use crate::db::init_db;
use crate::hook::handle_hook_state;

use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use std::{env};
use ctrlc;

const SWITCH_PIN: u8 = 26;


fn main() {
    tone::play_dtmf_tone('5', "hw:0,0");
}



