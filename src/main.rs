/*
to flash:
espflash flash ../target/riscv32imc-esp-espidf/debug/esp32-rust-split-keyboard --monitor
*/

use crate::ble_keyboard::*;
use anyhow;
use esp32_rust_split_keyboard::*;
use esp_idf_svc::hal::delay::FreeRtos;

fn main() -> anyhow::Result<()> {
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Hello, world!");

    let mut keyboard = Keyboard::new()?;

    let mut keyboard_left_side = KeyboardSide::new();
    keyboard_left_side.initialize_layers();

    let mut report_delay: u32 = REPORT_DELAY;

    loop {
        if keyboard.connected() {
            keyboard_left_side.set_rows("high");

            keyboard_left_side.check_cols();

            if report_delay == 0 {
                /* Check if the pins pressed have a valid combination in the hashmap */
                for pins in keyboard_left_side.pins_active_buffer.iter() {
                    if let Some(valid_key) = keyboard_left_side.base_layer.get(pins) {
                        log::info!("Valid_Key = {:?}", *valid_key);
                        keyboard.press(*valid_key);
                        keyboard.release();
                    }
                }

                /* Reset report_delay */
                report_delay = REPORT_DELAY;

                /* Reset active_pins */
                for pins in keyboard_left_side.pins_active_buffer.iter_mut() {
                    *pins = (PIN_INACTIVE, PIN_INACTIVE);
                }
                keyboard_left_side.pins_active_cnt = 0;
            }

            keyboard_left_side.set_rows("low");

            report_delay -= 1;
            FreeRtos::delay_ms(1);
        }
    }
}
