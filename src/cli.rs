use crate::{config::MainConfig, MCF};

impl MCF {
    pub fn cli_used(&self) -> bool {
        let args: Vec<String> = std::env::args().collect();

        if args.len() < 2 {
            return false;
        }

        let arg = args[1].as_str();

        match arg {
            "/?" | "-h" | "--help" => {
                println!("\
                    /? | -h | --help => Shows this menu\n\
                    -s | --showall => Display all audio devices found on the system\n\
                    -c | --create => Create config file in the same directory as the program\n\
                    -a | --add <device_name> => Adds device to config file. Surround device in quotes \"\"\n\
                    -d | --delete <device_name> => Removes device from config file. Surround device in quotes \"\"\n\
                ");
            }
            "-s" | "--showall" => print_devices(), // TODO: Print stuff in config
            "-c" | "--create" => self.save(&MainConfig::default()).unwrap(),
            "-a" | "--add" => {
                if let Some(name) = args.get(2) {
                    self.add_to_config(name.clone());
                } else {
                    println!("Device name not entered correctly");
                }
            }
            "-d" | "--delete" => {
                if let Some(name) = args.get(2) {
                    self.remove_from_config(name.clone());
                } else {
                    println!("Device name not entered correctly");
                }
            }
            _ => println!("Argument not recognized. Use --help for more info"),
        }

        true
    }

    fn add_to_config(&self, device: String) {
        // if config not extist, create_config();
        let mut cfg = match self.load() {
            Ok(cfg) => cfg,
            Err(_) => {
                self.save(&MainConfig::default()).unwrap();
                self.load().unwrap() // panic if this errors out
            }
        };
        cfg.devices.push(device);

        self.save(&cfg).unwrap();
    }

    fn remove_from_config(&self, device: String) {
        // if config not exist, return
        let mut cfg = self.load().unwrap();
        cfg.devices.retain(|d| d != device.as_str());

        self.save(&cfg).unwrap();
    }
}

fn print_devices() {
    use rodio::cpal::traits::{DeviceTrait, HostTrait};

    let host = rodio::cpal::default_host();

    for (_, device) in host.devices().unwrap().enumerate() {
        println!("{}", device.name().unwrap());
    }
    println!();
}
