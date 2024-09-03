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
                if let Some(valid_key) = keyboard_left_side.provide_value() {
                    log::info!("Valid_Key = {:?}", *valid_key);
                    keyboard.press(*valid_key);
                    keyboard.release();
                }

                report_delay = REPORT_DELAY;
            }

            keyboard_left_side.set_rows("low");

            report_delay -= 1;
            FreeRtos::delay_ms(1);
        }
    }
}
