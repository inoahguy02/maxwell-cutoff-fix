#![windows_subsystem = "windows"]
use std::{
    sync::{Arc, Mutex},
    thread,
    time::{Duration, Instant},
};
use soloud::Soloud;
use inputbot::{KeybdKey, MouseButton};

// I apologize for how horrendous this code looks

fn main() {
    let last_press_time = Arc::new(Mutex::new(Instant::now()));
    let player = Arc::new(Mutex::new(None));

    let paused = Arc::new(Mutex::new(false));
    // Start the timer thread
    let last_press_time_clone = last_press_time.clone();
    let player_clone = player.clone();
    let paused_clone = paused.clone();
    let timer_thread = thread::spawn(move || {
        loop {
            // Check if it's been 5 minutes since the last button press
            if last_press_time_clone.lock().unwrap().elapsed().as_secs() >= 300  && !*paused_clone.lock().unwrap() { // 5 minutes
                //println!("Destroying player");
                // Destroy the player object
                let mut player = player_clone.lock().unwrap();
                *player = None;

                // set pause to true
                let mut paused_state = paused_clone.lock().unwrap();
                *paused_state = true;
            }
            thread::sleep(Duration::from_secs(1));
        }
    });

    // Handle keyboard and mouse events
    let paused_clone_kb = paused.clone();
    let last_press_time_clone_kb = last_press_time.clone();
    let player_clone_kb = player.clone();
    KeybdKey::bind_all(move |_| {
        //println!("Keyboard pressed");

        // set last key press time
        let mut last_press_time = last_press_time_clone_kb.lock().unwrap();
        *last_press_time = Instant::now();

        // set paused to false
        let mut paused_state = paused_clone_kb.lock().unwrap();
        *paused_state = false;

        // Create the player object
        let mut player = player_clone_kb.lock().unwrap();
        if player.is_none() {
            *player = Some(Soloud::default().unwrap());
        }
        
    });

    let paused_clone_mouse = paused.clone();
    let last_press_time_clone_mouse = last_press_time.clone();
    let player_clone_mouse = player.clone();
    MouseButton::bind_all(move |_| {
        //println!("Mouse pressed");

        // set last key press time
        let mut last_press_time = last_press_time_clone_mouse.lock().unwrap();
        *last_press_time = Instant::now();

        // set paused to false
        let mut paused_state = paused_clone_mouse.lock().unwrap();
        *paused_state = false;

        // Create the player object
        let mut player = player_clone_mouse.lock().unwrap();
        if player.is_none() {
            *player = Some(Soloud::default().unwrap());
        }
    });

    inputbot::handle_input_events();

    // Wait for the timer thread to finish (it won't finish, though, because it's an infinite loop)
    timer_thread.join().unwrap();
}