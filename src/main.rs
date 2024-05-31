#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod cli;
mod config;

use config::DeviceConfig;
use rodio::cpal;
use rodio::cpal::traits::HostTrait;
use rodio::{DeviceTrait, OutputStream, OutputStreamHandle};
use std::{thread, time::Duration};

fn main() {
    if cli::cli_used() {
        return; // Don't run
    }

    let config = match config::load() {
        // Could make config updates in real time if it is in the loop
        Ok(cfg) => Some(cfg),
        Err(e) => {
            println!("Error: {}", e);
            println!("Using default device instead");
            None
        }
    };

    loop {
        let _streams = if let Some(ref config) = config {
            stream_with_cfg(&config)
        } else {
            (vec![], vec![OutputStream::try_default().unwrap()])
        };

        thread::sleep(Duration::from_secs(5)); // Make configurable?
    }
}

fn stream_with_cfg(
    cfg: &DeviceConfig,
) -> (Vec<cpal::Stream>, Vec<(OutputStream, OutputStreamHandle)>) {
    // This return type is garbo lol
    let host = cpal::default_host();
    let mut output_streams = Vec::new();
    let mut input_streams = Vec::new();

    for device_name in &cfg.devices {
        let system_devices = host.devices().unwrap();
        for device in system_devices {
            if device.name().unwrap_or_default() == *device_name {
                println!("Registering device {}", device.name().unwrap_or_default());
                if let Ok(stream) = OutputStream::try_from_device(&device) {
                    output_streams.push(stream);
                } else {
                    // Going to assume that this is an input device
                    let default_config = device.default_input_config().unwrap(); // TODO
                    let stream = device
                        .build_input_stream(
                            &default_config.config(),
                            move |_data: &[f32], _: &cpal::InputCallbackInfo| {},
                            move |e| {
                                println!("Error occured on input stream: {}", e);
                            },
                            None,
                        )
                        .unwrap(); // TODO
                    input_streams.push(stream);
                }
            }
        }
    }

    (input_streams, output_streams)
}
