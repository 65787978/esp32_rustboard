/*
to build: cargo build --release
to flash: espflash flash ./target/riscv32imc-esp-espidf/release/esp32-rust-split-keyboard --monitor
*/

use crate::ble_keyboard::*;

use anyhow;
use embassy_futures::select::select;
use embassy_time::Instant;
use enums::HidKeys;
use esp32_rust_split_keyboard::*;
use esp_idf_hal::task::block_on;
use esp_idf_sys::{
    esp_ble_power_type_t_ESP_BLE_PWR_TYPE_CONN_HDL0, esp_power_level_t_ESP_PWR_LVL_N24,
};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
fn main() -> anyhow::Result<()> {
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    /* initialize matrix */
    let mut pin_matrix = PinMatrix::new();

    log::info!("Pin Matrix Initialized...");

    /* initialize layers */
    let mut layers = Layers::new();
    layers.initialize_base_layer();

    /* initialize keys pressed hashmap */
    let keys_pressed: Arc<Mutex<HashMap<(i8, i8), Instant>>> = Arc::new(Mutex::new(HashMap::new()));

    /* run the tasks concurrently */
    block_on(async {
        select(
            send_report(&keys_pressed, &layers),
            pin_matrix.scan_grid(&keys_pressed),
        )
        .await;
    });

    Ok(())
}

async fn send_report(keys_pressed: &Arc<Mutex<HashMap<(i8, i8), Instant>>>, layers: &Layers) -> ! {
    /* initialize BLE */
    let mut ble_connection = Keyboard::new().unwrap();

    log::info!("BLE Initialized...");

    /* initialize modifier */
    let mut modifier: u8 = 0;

    /* initialize set_ble_power_flag */
    let mut set_ble_power_flag = true;

    /* Run the main loop */
    loop {
        if ble_connection.connected() {
            /* set ble power save */
            if set_ble_power_flag {
                /* set power save */
                set_ble_power();

                /* set flag to false */
                set_ble_power_flag = false;
            }
            /* store the keys that have been reported */
            let mut keys_reported: Vec<(i8, i8)> = Vec::new();
            /* try to lock the hashmap */
            match keys_pressed.try_lock() {
                Ok(mut keys_pressed_locked) => {
                    /* check if there are pressed keys */
                    if !keys_pressed_locked.is_empty() {
                        /* iter trough the pressed keys */
                        for ((row, col), time_pressed) in keys_pressed_locked.iter() {
                            /* check if the current key has valid debounce */
                            if Instant::now() >= *time_pressed + DEBOUNCE_DELAY {
                                /* get the key from the layer */
                                if let Some(valid_key) = layers.base.get(&(*row, *col)) {
                                    /* */
                                    match *valid_key {
                                        HidKeys::Shift => modifier |= HidKeys::Shift as u8,
                                        HidKeys::Control => modifier |= HidKeys::Control as u8,
                                        // HidKeys::Alt => modifier |= HidKeys::Alt,
                                        // HidKeys::Super => modifier |= HidKeys::Super,
                                        _ => {}
                                    }

                                    /* send the key */
                                    ble_connection.press(*valid_key as u8, modifier as u8);
                                    ble_connection.release();

                                    /* store row and col */
                                    keys_reported.push((*row, *col));
                                }
                            }
                        }

                        for row_col in keys_reported.iter() {
                            /* key reported - remove the key */
                            keys_pressed_locked.remove(row_col);
                        }

                        /* reset the modifier */
                        modifier = 0;
                    }
                }
                Err(_) => {}
            }

            delay::delay_us(10).await;
        } else {
            log::info!("Keyboard not connected!");

            /* reset ble power save flag*/
            set_ble_power_flag = true;

            delay::delay_ms(100).await;
        }
    }
}

fn set_ble_power() {
    /* set power save */
    unsafe {
        esp_idf_sys::esp_ble_tx_power_set(
            esp_ble_power_type_t_ESP_BLE_PWR_TYPE_CONN_HDL0,
            esp_power_level_t_ESP_PWR_LVL_N24,
        );
    }
}
