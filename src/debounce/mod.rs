use crate::{
    config::config::{DEBOUNCE_DELAY, PRESSED_KEYS_INDEXMAP_SIZE},
    delay::delay_us,
    matrix::Key,
};
use embassy_time::Instant;
use heapless::FnvIndexMap;
use spin::Mutex;

#[derive(Debug)]
pub struct Debounce {
    pub key_pressed_time: Instant,
    pub key_ready_for_removal: bool,
    pub key_falling_edge: bool,
    pub key_rising_edge: bool,
}

pub async fn calculate_debounce(
    keys_pressed: &Mutex<FnvIndexMap<Key, Debounce, PRESSED_KEYS_INDEXMAP_SIZE>>,
) -> ! {
    loop {
        /* try to get a lock on keys_pressed */
        if let Some(mut keys_pressed) = keys_pressed.try_lock() {
            /* itter throught the pressed keys */
            for (_key, debounce) in keys_pressed.iter_mut() {
                /* check if the key has passed the debounce delay */
                if debounce.key_rising_edge {
                    debounce.key_ready_for_removal = true;
                }
            }
        }
        delay_us(10).await;
    }
}
