// #![windows_subsystem = "windows"]
#![allow(unused_variables)]
mod cli;
mod config;

use std::{thread, time::Duration, u64::MAX};
use config::DeviceConfig;
use cpal::traits::HostTrait;
use rodio::{DeviceTrait, OutputStream, OutputStreamHandle};

fn main() {
    if cli::cli_used() { return; } // Don't run

    // if cfg file is not present, just do default device
    let stream = match config::load() {
        Ok(cfg) => stream_with_cfg(cfg),
        Err(_) => vec![OutputStream::try_default().unwrap()] // Just panic here tbh
    };
    println!("Going to sleep. gn");
    loop { thread::sleep(Duration::from_secs(MAX)) } // 584 eons
}

fn stream_with_cfg(cfg: DeviceConfig) -> Vec<(OutputStream, OutputStreamHandle)> {
    let host = cpal::default_host();
    let mut streams = Vec::new();
    
    for device_name in cfg.devices {
        let system_devices = host.devices().unwrap();
        for device in system_devices {
            if device.name().unwrap_or_default() == device_name {
                let stream = OutputStream::try_from_device(&device).unwrap();
                streams.push(stream);
            }
        }
    }

    streams
}
