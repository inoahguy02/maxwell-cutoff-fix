#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

///////////////////////////////////////////////////////////
/// `cargo run` WILL NOT WORK IF YOU WANT TO USE CONFIG ///
///////////////////////////////////////////////////////////
mod cli;
mod config;

use auto_launch::AutoLaunch;
use config::MainConfig;
use flexi_logger::Logger;
use log::{debug, error, info, warn};
use rodio::cpal;
use rodio::cpal::traits::HostTrait;
use rodio::{DeviceTrait, OutputStream, OutputStreamHandle};
use std::path::PathBuf;
use std::{env, fs};
use std::{thread, time::Duration};
use sysinfo::System;

fn main() {
    let mcf = MCF::init();

    if mcf.cli_used() {
        return; // Don't run
    }

    if mcf.already_running() {
        return; // Don't run
    }

    // Make sure app will run on startup
    enable_autorun();

    // Debug logger
    #[cfg(debug_assertions)]
    Logger::try_with_str("debug").unwrap().start().unwrap();

    let config = match mcf.load() {
        // Could make config updates in real time if it is in the loop
        Ok(cfg) => {
            #[cfg(not(debug_assertions))]
            mcf.initialize_logger(cfg.num_of_kept_logs.into());

            info!("Config found. Initialized logger");

            // Add missing fields to the config
            if let Err(e) = mcf.save(&cfg) {
                warn!("Failed to add missing fields to config: {}", e);
            }

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
            (
                vec![],
                vec![OutputStream::try_default().unwrap_or_else(|e| {
                    error!("Failed to initialize default device: {}", e);
                    panic!()
                })],
            )
        };

        thread::sleep(Duration::from_secs(5)); // Make configurable?
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
        let system_devices = host.devices().unwrap_or_else(|e| {
            error!("Failed to initialize devices: {}", e);
            panic!()
        });
        for device in system_devices {
            if device.name().unwrap_or_default() == *device_name {
                debug!("Registering device {}", device.name().unwrap_or_default());
                if let Ok(stream) = OutputStream::try_from_device(&device) {
                    output_streams.push(stream);
                } else {
                    // Going to assume that this is an input device
                    let default_config = device.default_input_config().unwrap_or_else(|e| {
                        error!("Failed to get default input config: {}", e);
                        panic!()
                    });
                    let stream = device
                        .build_input_stream(
                            &default_config.config(),
                            move |_data: &[f32], _: &cpal::InputCallbackInfo| {},
                            move |e| {
                                error!("Error occured on input stream: {}", e);
                            },
                            None,
                        )
                        .unwrap_or_else(|e| {
                            error!("Failed to build input stream: {}", e);
                            panic!()
                        });
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
        let dir = env::current_dir().unwrap().join("maxwell-fix");

        if !dir.exists() {
            fs::create_dir(&dir).unwrap();
        }

        Self {
            extra_files_dir: dir,
        }
    }

    fn already_running(&self) -> bool {
        // We can unwrap here because we can't log, and can't do anything if this fails
        let current_exe = env::current_exe().unwrap();
        let current_exe_name_osstr = current_exe.file_name().unwrap();
        let current_exe_name = current_exe_name_osstr.to_str().unwrap();

        let system = System::new_all();

        let process_list = system.processes_by_name(current_exe_name);

        if process_list.count() > 1 {
            return true;
        }

        false
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

fn enable_autorun() {
    // I think this works for both windows and linux

    let app_name = "maxwell-fix"; // This will need to be changed if name changed

    let path = env::current_exe().unwrap();
    let app_path = path.as_os_str().to_str().unwrap();
    // path/to/app.exe

    let stuff = AutoLaunch::new(app_name, app_path, &[] as &[&str]);
    _ = stuff.enable().is_ok();
}
