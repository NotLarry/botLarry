pub fn handle_unknown_number(gpio: &Gpio, switch: &InputPin, number: &str) -> bool {
    use chrono::Local;
    use std::fs;
    use std::process::Command;
    use std::thread;
    use std::time::Duration;
    use crate::keypad::get_key;

    let rows: Vec<_> = crate::keypad::ROW_PINS
        .iter()
        .map(|&pin| gpio.get(pin).unwrap().into_input_pullup())
        .collect();
    let mut cols: Vec<_> = crate::keypad::COL_PINS
        .iter()
        .map(|&pin| {
            let mut col = gpio.get(pin).unwrap().into_output();
            col.set_high();
            col
        })
        .collect();

    let area_code = &number[0..3];
    let timestamp = Local::now().format("%Y%m%d%H%M%S").to_string();
    let dir_path = format!("recordings/{}", area_code);
    let file_path = format!("{}/{}-{}.mp3", dir_path, number, timestamp);

    fs::create_dir_all(&dir_path).expect("Failed to create recording directory");

    // Ring 3 times
    for _ in 0..3 {
        let mut child = Command::new("sox")
            .args(["-n", "-t", "alsa", "hw:0,0", "synth", "2", "sin", "440", "sin", "480"])
            .spawn()
            .expect("Failed to play ring tone");
        thread::sleep(Duration::from_secs(2));
        let _ = child.kill();
        thread::sleep(Duration::from_secs(4));
    }

    // Beep
    let _ = Command::new("sox")
        .args(["-n", "-t", "alsa", "hw:0,0", "synth", "0.2", "sin", "1000"])
        .spawn()
        .and_then(|mut c| c.wait());

    // Start recording
    let mut arecord = Command::new("arecord")
        .args(["-D", "hw:1,0", "-f", "cd", &file_path])
        .spawn()
        .expect("Failed to start recording");

    let mut success = false;

    loop {
        if switch.read() == Level::High {
            println!("⏹️  On-hook detected. Abandoning recording.");
            break;
        }

        if let Some(key) = get_key(&rows, &mut cols) {
            if key == '#' {
                println!("⏹️  '#' pressed. Finalizing recording.");
                success = true;
                break;
            }
        }

        thread::sleep(Duration::from_millis(100));
    }

    let _ = arecord.kill();
    success
}

