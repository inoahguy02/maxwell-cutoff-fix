#![cfg_attr(
    all(
      target_os = "windows",
      not(debug_assertions),
    ),
    windows_subsystem = "windows"
  )]
#![allow(unused_variables)]
mod cli;
mod config;

use std::{thread, time::Duration};
use config::DeviceConfig;
use cpal::traits::HostTrait;
use rodio::{DeviceTrait, OutputStream, OutputStreamHandle};

fn main() {
    if cli::cli_used() { return; } // Don't run
    
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
    let host = cpal::default_host();
    let mut streams = Vec::new();
    
    for device_name in &cfg.devices {
        let system_devices = host.devices().unwrap();
        for device in system_devices {
            if device.name().unwrap_or_default() == *device_name {
                println!("Registering device {}", device.name().unwrap_or_default());
                let stream = OutputStream::try_from_device(&device).unwrap();
                streams.push(stream);
            }
        }
    }

    streams
}
