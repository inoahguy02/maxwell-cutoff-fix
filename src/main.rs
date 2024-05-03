#![windows_subsystem = "windows"]
#![allow(unused_variables)]
use std::{
    thread,
    time::Duration,
};
use soloud::Soloud;

fn main() {
    let player = Soloud::default().unwrap();
    loop { thread::sleep(Duration::from_secs(86400)) } // 24 hours
}