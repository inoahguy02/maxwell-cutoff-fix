use crate::config;

pub fn cli_used() -> bool {
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
        "-c" | "--create" => config::create_default().unwrap(),
        "-a" | "--add" => {
            if let Some(name) = args.get(2) {
                add_to_config(name.clone());
            } else {
                println!("Device name not entered correctly");
            }
        }
        "-d" | "--delete" => {
            if let Some(name) = args.get(2) {
                remove_from_config(name.clone());
            } else {
                println!("Device name not entered correctly");
            }
        }
        _ => println!("Argument not recognized. Use --help for more info")
    }
    
    true
}

fn print_devices() {
    use cpal::traits::{DeviceTrait, HostTrait};

    let host = cpal::default_host();

    for (_, device) in host.devices().unwrap().enumerate() {
        println!("{}",device.name().unwrap());
    }
    println!();
}


fn add_to_config(device: String) {
    // if config not extist, create_config();
    let mut cfg = match config::load() {
        Ok(cfg) => cfg,
        Err(_) => {
            config::create_default().unwrap();
            config::load().unwrap() // panic if this errors out
        }
    };
    cfg.devices.push(device);
    
    config::save(&cfg).unwrap();
}

fn remove_from_config(device: String) {
    // if config not exist, return
    let mut cfg = config::load().unwrap();
    cfg.devices.retain(|d| d != device.as_str());

    config::save(&cfg).unwrap();
}