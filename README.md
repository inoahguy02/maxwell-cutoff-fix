# maxwell-cutoff-fix

This is a fix for the Audeze Maxwell's inital audio cutoff

## How it works
This program will be ran on startup. It will play a silent audio file to keep the headset in an active state.
In an effort to save as much battery as possible, this program will pause the audio file after
5 minutes of innactivity on the computer (no keyboard or mouse inputs). This will allow the headset
to go into sleep mode instead of staying on until the battery dies or when the computer/headset is turned off


## Language choice
I chose Rust over a scripting language like Python because it is relatively simple program and I want a simple executable
with no strings attached
