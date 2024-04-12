# maxwell-cutoff-fix

This is a fix for the Audeze Maxwell's inital audio cutoff

## Language choice
I chose Rust over a scripting language like Python because it is relatively simple program and I want a simple executable
with no strings attached

## How it works
After many trials and errors with using different rust crates, I finally found a couple that I could work with.
My first approach was to play a silent .mp3 file, but I found out that if I just initialize an object
of the Soloud crate, it keeps a constant connection to the headset.<br><br>

So as soon as it detects a form of input (mouse click or keyboard press), it will make the object and starts a
5 minute timer. After 5 minutes of no input, it set the object to None and allows the headset to shut off

## How to install
1. Go to releases and download the most recent patch
2. Press Win+R and type "shell:startup"
3. Place exectuable in that directory. This allows the program to start on startup
4. Go to windows defender or any other antivirus and whitelist the program. This is necessary because antivirus software will treat this program as a key logger
5. Double click executable if you are wanting to run it now

## Future wants
I want it to detect mouse movement as well
