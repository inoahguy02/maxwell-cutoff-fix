///////////////////////////////////////////////////////////
/// `cargo run` WILL NOT WORK IF YOU WANT TO USE CONFIG ///
///////////////////////////////////////////////////////////
mod cli;
mod config;

use anyhow::{anyhow, Context};
use auto_launch::AutoLaunch;
use flexi_logger::Logger;
use log::error;
use rodio::cpal;
use rodio::cpal::traits::HostTrait;
use rodio::{DeviceTrait, OutputStream, OutputStreamHandle};
use single_instance::SingleInstance;
use std::env;
use std::process::Command;
use std::{thread, time::Duration};

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

#[cfg(target_os = "windows")]
use windows::Win32::System::Console::FreeConsole;

use crate::config::ConfigManager;

fn main() {
    if let Err(e) = start() {
        println!("Error returned from start(): {:#}", e);
        error!("Error returned from start(): {:#}", e);
    }
}

fn start() -> anyhow::Result<()> {
    // Try to create the global mutex. Need to ensure we are the only one
    let lock = SingleInstance::new("maxwell-fix")?;
    if !lock.is_single() {
        println!("Another instance is currently running\nClosing program in 5 seconds...");
        thread::sleep(Duration::from_secs(5));
        return Err(anyhow!("Another instance is currently running"));
    }

    let mcf = MCF::init()?;

    if mcf.cli_used() {
        return Ok(()); // Don't run
    }

    mcf.run()
}

impl MCF {
    fn run(&self) -> anyhow::Result<()> {
        self.initialize_logger()?;

        // Make sure app doesn't take console window hostage
        self.detach_process()?;

        // Make console window go away (Windows only)
        #[cfg(target_os = "windows")]
        unsafe {
            FreeConsole()?;
        }

        // Make sure app will run on startup
        if self.cfgmgr.cfg.borrow().autostart {
            self.enable_autorun()?;
        }

        loop {
            self.cfgmgr.reload_config()?;
            let _streams = self.get_streams()?;
            thread::sleep(Duration::from_secs(5)); // Make configurable?
        }
    }

    // I think this works for both windows and linux
    fn enable_autorun(&self) -> anyhow::Result<()> {
        let app_name = "maxwell-fix"; // This will need to be changed if name changed

        let path = env::current_exe()?;
        let app_path = path
            .as_os_str()
            .to_str()
            .context("Failed to get app_path")?;
        // path/to/app.exe

        AutoLaunch::new(app_name, app_path, &[] as &[&str]).enable()?;

        Ok(())
    }

    fn detach_process(&self) -> anyhow::Result<()> {
        if let Ok(is_detached) = env::var("DETACHED") {
            if is_detached == "1" {
                return Ok(());
            }
        }

        let current_exe = env::current_exe()?;
        let mut command = Command::new(current_exe);
        command.env("DETACHED", "1");

        #[cfg(target_os = "windows")]
        command.creation_flags(0x00000008); // Detached process flag

        command.spawn()?;

        println!("Started maxwell-fix");

        std::process::exit(0);
    }

    fn init() -> anyhow::Result<Self> {
        Ok(Self {
            cfgmgr: ConfigManager::init()?,
        })
    }

    fn initialize_logger(&self) -> anyhow::Result<()> {
        if cfg!(debug_assertions) {
            Logger::try_with_str("debug")?.start()?;
            return Ok(());
        }

        let logdir = self
            .cfgmgr
            .cfg_file_path
            .parent()
            .context("Failed to get parent dir")?
            .join("maxwell-fix-logs");

        Logger::try_with_str("info")?
            .log_to_file(flexi_logger::FileSpec::default().directory(&logdir))
            .rotate(
                flexi_logger::Criterion::Size(10 * 1024 * 1024), // 10 MB
                flexi_logger::Naming::Timestamps,
                flexi_logger::Cleanup::KeepLogFiles(
                    self.cfgmgr.cfg.borrow().num_of_kept_logs as usize,
                ),
            )
            .format_for_files(flexi_logger::detailed_format)
            .start()?;

        Ok(())
    }

    fn get_streams(&self) -> anyhow::Result<Streams> {
        let mut streams = Streams::default();

        if self.cfgmgr.cfg.borrow().devices.len() == 0 {
            // No device found in config. Attempt default
            streams
                .output_streams
                .push(OutputStream::try_default().context(
                "No device found in config. Failed to create stream using default output device",
            )?);
            return Ok(streams);
        }

        let system_devices = cpal::default_host().devices()?;

        for system_device in system_devices {
            if self
                .cfgmgr
                .cfg
                .borrow()
                .devices
                .contains(&system_device.name()?)
            {
                if let Ok(stream) = OutputStream::try_from_device(&system_device) {
                    // Output device
                    streams.output_streams.push(stream);
                    break;
                }

                // Going to assume that this is an input device
                let input_cfg = system_device.default_input_config()?;
                let stream = system_device.build_input_stream(
                    &input_cfg.config(),
                    move |_data: &[f32], _: &cpal::InputCallbackInfo| {},
                    move |e| {
                        error!("Error occured on input stream: {}", e);
                    },
                    None,
                )?;
                streams.input_streams.push(stream)
            }
        }

        Ok(streams)
    }
}

pub struct MCF {
    // pub extra_files_dir: PathBuf,
    pub cfgmgr: ConfigManager,
}

#[derive(Default)]
struct Streams {
    input_streams: Vec<cpal::Stream>,
    output_streams: Vec<(OutputStream, OutputStreamHandle)>,
}
