use crate::MCF;
use rodio::cpal::traits::{DeviceTrait, HostTrait};
use std::io;

impl MCF {
    pub fn cli_used(&self) -> bool {
        let args: Vec<String> = std::env::args().collect();

        if args.len() < 2 {
            return false;
        }

        let arg = args[1].as_str();

        match arg {
            "/?" | "-h" | "--help" => {
                println!(
                    "\
                    /? | -h | --help => Shows this menu\n\
                    -e | --edit => Edit device config\n\
                "
                );
            }
            "-e" | "--edit" => {
                if let Err(e) = self.edit_config() {
                    println!("{:#}", e);
                }
            }
            _ => println!("Argument not recognized. Use --help for more info"),
        }

        true
    }

    fn edit_config(&self) -> anyhow::Result<()> {
        loop {
            println!("\nCurrent devices in config:");
            for device in self.cfgmgr.cfg.borrow().devices.iter() {
                println!("{}", device);
            }

            println!("\nEnter an option:\n1: Add a device\n2: Remove a device\n3: Exit");

            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            println!();

            if let Err(e) = match input.trim() {
                "1" => self.add_device(),
                "2" => self.remove_device(),
                "3" => break,
                _ => {
                    println!("Unknown command\n");
                    Ok(())
                }
            } {
                println!("Error: {}", e);
            }
        }

        Ok(())
    }

    fn add_device(&self) -> anyhow::Result<()> {
        let mut cfg = self.cfgmgr.cfg.borrow_mut();
        let system_devices: Vec<String> = rodio::cpal::default_host()
            .devices()?
            .map(|d| d.name())
            .collect::<Result<Vec<_>, _>>()?;

        for (i, device) in system_devices.iter().enumerate() {
            println!("{}: {}", i + 1, device);
        }

        println!("\nEnter the number of the device to add");
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let key: usize = input.trim().parse()?;

        if let Some(name) = system_devices.get(key - 1) {
            cfg.devices.insert(name.to_string());
        }

        drop(cfg);
        self.cfgmgr.save_config()?;
        Ok(())
    }

    fn remove_device(&self) -> anyhow::Result<()> {
        let mut cfg = self.cfgmgr.cfg.borrow_mut();
        let mut sorted_devices: Vec<String> = cfg.devices.iter().cloned().collect();
        sorted_devices.sort();

        for (i, device) in sorted_devices.iter().enumerate() {
            println!("{}: {}", i + 1, device);
        }

        println!("\nEnter the number of the device to remove");
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let key: usize = input.trim().parse()?;

        if let Some(name) = sorted_devices.get(key - 1) {
            cfg.devices.remove(name.as_str());
        }

        drop(cfg);
        self.cfgmgr.save_config()?;
        Ok(())
    }
}
