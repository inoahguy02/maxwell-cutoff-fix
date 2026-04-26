## What's this for?

This is a fix for the Audeze Maxwell's inital audio cutoff<br>
Currently, the common fix for this problem is to play a silent .mp3 file<br>
That does fix the problem, but it makes it to where the headset never shuts off<br>
This program however makes it to where the audio cutoff is gone and allows the headset to shut off after being inactive for some time

## How to install (Windows)
1. Go to releases and download the most recent patch
2. Place executable anywhere on the system
3. Double click executable if you are wanting to run it now

## Linux Installation (Build from Source)

**Note: Currently only tested on Arch Linux. Multi-Device support was not tested.**

### Prerequisites
1. Install Rust:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source $HOME/.cargo/env
   ```

### Build and Install
1. Clone the repository:
   ```bash
   git clone https://github.com/inoahguy02/maxwell-cutoff-fix.git
   cd maxwell-cutoff-fix
   ```

2. Compile the project:
   ```bash
   cargo build --release
   ```

3. The compiled binary will be located at `target/release/maxwell-fix`

### Autostart
To run maxwell-fix automatically on system startup, you can use one of the following methods:
- Desktop environment autostart (`~/.config/autostart/*.desktop`)
- systemd user services
- Your distribution's specific autostart mechanism

## Multi-Device support

This program now supports keeping more than one device alive. This is done with the use of a config. Here's how to set that up:

1. Open a terminal in the location of the program and type `./maxwell-fix --edit`
2. You will then be prompted to either add or remove a device from the config. Type `1` and press enter
3. From the list that is displayed, choose the number of the device you want to add and press enter
4. Once everything is added, go ahead and run the program
