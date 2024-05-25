#![allow(unused_variables)]
mod cli;
mod config;

use std::{thread, time::Duration};
use config::DeviceConfig;
use rodio::cpal::traits::HostTrait;
use rodio::{DeviceTrait, OutputStream, OutputStreamHandle};

fn main() {
    if cli::cli_used() { 
        return; // Don't run
    } 
    
    if cfg!(target_os = "Windows") {
        hide_console_window();
    }

    let config = match config::load() { // Could make config updates in real time if it is in the loop
        Ok(cfg) => Some(cfg),
        Err(e) => {
            println!("Error: {}", e);
            println!("Using default device instead");
            None
        }
    };

    loop {
        let streams = if let Some(ref config) = config {
            stream_with_cfg(&config)
        } else {
            vec![OutputStream::try_default().unwrap()]
        };
        
        thread::sleep(Duration::from_secs(5)); // Make configurable?
    } 
}

fn stream_with_cfg(cfg: &DeviceConfig) -> Vec<(OutputStream, OutputStreamHandle)> {
    let host = rodio::cpal::default_host();
    let mut streams = Vec::new();
    
    for device_name in &cfg.devices {
        let system_devices = host.devices().unwrap();
        for device in system_devices {
            if device.name().unwrap_or_default() == *device_name {
                println!("Registering device {}", device.name().unwrap_or_default());
                let stream = OutputStream::try_from_device(&device).unwrap_or_else(|e| {
                    println!("Error occurred: {}", e);
                    panic!("Panicked from previous error");
                });
                streams.push(stream);
            }
        }
    }

    streams
}

fn hide_console_window() {
    use std::ptr;
    use winapi::um::wincon::GetConsoleWindow;
    use winapi::um::winuser::{ShowWindow, SW_HIDE};

    let window = unsafe {GetConsoleWindow()};
    if window != ptr::null_mut() {
        unsafe {
            ShowWindow(window, SW_HIDE);
        }
    }
}
