/*
to flash:
espflash flash ../target/riscv32imc-esp-espidf/debug/esp32-rust-split-keyboard --monitor
*/

use crate::ble_keyboard::*;
use anyhow;
use esp32_rust_split_keyboard::*;

fn main() -> anyhow::Result<()> {
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    let mut keyboard = Keyboard::new()?;

    let mut keyboard_left_side = KeyboardSide::new();
    keyboard_left_side.initialize_layers();

    loop {
        if keyboard.connected() {
            keyboard_left_side.iter_rows_cols();

            /* send report if delay is 0 */
            if keyboard_left_side.report_delay == 0 {
                /* Check if the pins pressed have a valid combination in the hashmap */
                for pins in keyboard_left_side.pins_active_buffer.iter() {
                    if let Some(valid_key) = keyboard_left_side.base_layer.get(pins) {
                        log::info!("Valid_Key = {:?}", *valid_key);
                        keyboard.press(*valid_key);
                        keyboard.release();
                    }
                }

                /* reset counters */
                keyboard_left_side.reset();
            }
        }
    }
}
