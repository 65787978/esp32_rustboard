/*
to flash:
espflash flash ../target/riscv32imc-esp-espidf/debug/esp32-rust-split-keyboard --monitor
*/

use crate::ble_keyboard::*;
use anyhow;
use esp32_rust_split_keyboard::*;
use esp_idf_hal::delay::FreeRtos;

fn main() -> anyhow::Result<()> {
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    let mut keyboard = Keyboard::new()?;

    let mut keyboard_left_side = KeyboardSide::new();
    keyboard_left_side.initialize_layers();

    let mut key_pressed: bool = false;

    loop {
        if keyboard.connected() {
            /* check rows and cols */
            for row in keyboard_left_side.key_matrix.rows.iter_mut() {
                /* set row to high */
                row.set_high()?;

                /* check if a col is high */
                for col in keyboard_left_side.key_matrix.cols.iter_mut() {
                    /* if a col is high */
                    while col.is_high() {
                        /* and if the row-col combination is valid */
                        if let Some(valid_key) =
                            keyboard_left_side.base_layer.get(&(row.pin(), col.pin()))
                        {
                            /* send press key */
                            log::info!("Valid_Key = {:?}", *valid_key);
                            keyboard.press(*valid_key);
                            key_pressed = true;
                        }
                    }
                    /* if a key has been pressed, send a release */
                    if key_pressed {
                        keyboard.release();
                        key_pressed = false;
                    }
                }

                row.set_low()?;

                /* Wait 1 ms */
                FreeRtos::delay_ms(10);
            }
        }
    }
}
