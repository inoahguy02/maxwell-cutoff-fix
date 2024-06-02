#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

///////////////////////////////////////////////////////////
/// `cargo run` WILL NOT WORK IF YOU WANT TO USE CONFIG ///
///////////////////////////////////////////////////////////
mod cli;
mod config;

use config::MainConfig;
use flexi_logger::Logger;
use fs2::FileExt;
use log::{debug, error, info, warn};
use rodio::cpal;
use rodio::cpal::traits::HostTrait;
use rodio::{DeviceTrait, OutputStream, OutputStreamHandle};
use std::fs::File;
use std::path::PathBuf;
use std::{env, fs};
use std::{thread, time::Duration};

fn main() {
    let mcf = MCF::init();

    if mcf.cli_used() {
        return; // Don't run
    }

    // Debug logger
    #[cfg(debug_assertions)]
    Logger::try_with_str("debug").unwrap().start().unwrap();

    let lock = mcf.get_lock_file().unwrap(); // TODO debug log

    match lock.try_lock_exclusive() {
        Ok(()) => {
            let config = match mcf.load() {
                // Could make config updates in real time if it is in the loop
                Ok(cfg) => {
                    #[cfg(not(debug_assertions))]
                    mcf.initialize_logger(cfg.num_of_kept_logs.into());

                    info!("Config found. Initialized logger");

                    // Add missing fields to the config
                    mcf.save(&cfg).unwrap(); // TODO

                    Some(cfg)
                }
                Err(e) => {
                    #[cfg(not(debug_assertions))]
                    mcf.initialize_logger(10);

                    error!("Error: {}", e);
                    warn!("Config not found. Defaulting 'num_of_kept_logs' to 10");
                    info!("Using default device");
                    None
                }
            };

            loop {
                let _streams = if let Some(ref cfg) = config {
                    stream_with_cfg(&cfg)
                } else {
                    (vec![], vec![OutputStream::try_default().unwrap()])
                };

                thread::sleep(Duration::from_secs(5)); // Make configurable?
            }
        }
        Err(_) => {} // TODO log
    }
}

fn stream_with_cfg(
    cfg: &MainConfig,
) -> (Vec<cpal::Stream>, Vec<(OutputStream, OutputStreamHandle)>) {
    // This return type is garbo lol
    let host = cpal::default_host();
    let mut output_streams = Vec::new();
    let mut input_streams = Vec::new();

    for device_name in &cfg.devices {
        let system_devices = host.devices().unwrap();
        for device in system_devices {
            if device.name().unwrap_or_default() == *device_name {
                debug!("Registering device {}", device.name().unwrap_or_default());
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
                                error!("Error occured on input stream: {}", e);
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

pub struct MCF {
    pub extra_files_dir: PathBuf,
}

impl MCF {
    fn init() -> Self {
        let dir = env::current_dir().unwrap().join("maxwell-cutoff-fix");

        if !dir.exists() {
            fs::create_dir(&dir).unwrap();
        }

        Self {
            extra_files_dir: dir,
        }
    }

    fn get_lock_file(&self) -> anyhow::Result<File> {
        let lock_path = self.extra_files_dir.join("maxwell-cutoff-fix.lock");

        Ok(fs::File::create(lock_path)?)
    }

    #[cfg(not(debug_assertions))]
    fn initialize_logger(&self, num_of_kept_files: usize) {
        Logger::try_with_str("info")
            .unwrap()
            .log_to_file(flexi_logger::FileSpec::default().directory(&self.extra_files_dir))
            .rotate(
                flexi_logger::Criterion::Size(10 * 1024 * 1024), // 10 MB
                flexi_logger::Naming::Timestamps,
                flexi_logger::Cleanup::KeepLogFiles(num_of_kept_files),
            )
            .format_for_files(flexi_logger::detailed_format)
            .start()
            .unwrap();
    }
}
