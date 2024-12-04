use crate::{
    config::config::{DEBOUNCE_DELAY, DEBOUNCE_DELAY_LAYER_KEY,  PRESSED_KEYS_INDEXMAP_SIZE, LAYER_KEY},
    delay::delay_us,
    matrix::Key,
};
use embassy_time::{Duration, Instant};
use heapless::FnvIndexMap;
use spin::mutex::Mutex;

pub const KEY_PRESSED: u8 = 1;
pub const KEY_RELEASED: u8 = 2;

#[derive(Debug)]
pub struct Debounce {
    pub key_pressed_time: Instant,
    pub key_state: u8,
}

pub async fn calculate_debounce(
    keys_pressed: &Mutex<FnvIndexMap<Key, Debounce, PRESSED_KEYS_INDEXMAP_SIZE>>,
) -> ! {
    loop {
        /* try to get a lock on keys_pressed */
        if let Some(mut keys_pressed) = keys_pressed.try_lock() {
            /* itter throught the pressed keys */
            for (key, debounce) in keys_pressed.iter_mut() {
                let debounce_delay: Duration = match *key {
                    LAYER_KEY => DEBOUNCE_DELAY_LAYER_KEY,
                    _ =>  DEBOUNCE_DELAY,
                };

                /* check if the key has passed the debounce delay or has been released */
                if Instant::now() >= debounce.key_pressed_time + debounce_delay {
                    debounce.key_state = KEY_RELEASED;
                }
            }
        }
        delay_us(10).await;
    }
}
