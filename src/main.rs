use esp::KeyboardLeftSide;
use esp_idf_svc::{bt::*, log::EspLogger, sys};
use std::{thread::sleep, time::Duration};

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Hello, world!");

    /* Declare keys */
    let mut keyboard_left_side = KeyboardLeftSide::new();
    keyboard_left_side.initialize_keys();

    let mut has_key_been_pressed_this_cycle = false;
    let mut previous_key_pressed: (u8, u8) = (0, 0);

    loop {
        /* Implement hardware matrix scan */
        let key_pressed_matrix: Option<(u8, u8)> = Some((0, 0));

        match key_pressed_matrix {
            Some(key_pressed) => {
                if let Some(key) = keyboard_left_side.key.get_mut(&key_pressed) {
                    *key = true;
                    previous_key_pressed = key_pressed;
                    has_key_been_pressed_this_cycle = true;
                    log::info!("Key pressed: {:?}", key_pressed);
                }
            }

            /* If a key is not pressed, set the previous key to false */
            None => {
                if has_key_been_pressed_this_cycle {
                    keyboard_left_side.key.insert(previous_key_pressed, false);
                    has_key_been_pressed_this_cycle = false;
                }
            }
        }

        /* Sleep for 20 milliseconds before fetching the matrix */
        sleep(Duration::from_millis(20));
    }
}
