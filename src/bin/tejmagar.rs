use signal_hook::consts::SIGHUP;
use signal_hook::consts::SIGINT;
use signal_hook::consts::SIGQUIT;
use signal_hook::consts::SIGTERM;
use signal_hook::iterator::Signals;
use std::{thread, time::Duration};

use std::process::Command;

pub fn start() {
    let start_front = if cfg!(target_os = "windows") {
        Command::new("tejmagar-main")
            .args(["/C", ""])
            .output()
            .expect("failed to execute process")
    } else if cfg!(target_os = "macos") {
        Command::new("tejmagar-main")
            .arg("-c")
            .arg("")
            .output()
            .expect("failed to execute process")
    } else if cfg!(target_os = "linux") {
        Command::new("tejmagar-main")
            .arg("-c")
            .arg("")
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("tejmagar-main")
            .arg("-c")
            .arg("")
            .output()
            .expect("failed to execute process")
    };

    let start_front = String::from_utf8(start_front.stdout)
        .map_err(|non_utf8| String::from_utf8_lossy(non_utf8.as_bytes()).into_owned())
        .unwrap();

    println!("{}", start_front);
}

pub fn process_daemon() {
    let mut signals = match Signals::new(&[SIGHUP, SIGINT, SIGQUIT, SIGTERM]) {
        Ok(t) => t,
        Err(e) => panic!("{}", e),
    };

    thread::spawn(move || {
        for _sig in signals.forever() {
            start();
            //println!("53:{:?}", _sig);
        }
    });

    thread::sleep(Duration::from_secs(2));
}

fn main() {
    loop {
        //start();
        //println!("62:");
        process_daemon();
    }
}
