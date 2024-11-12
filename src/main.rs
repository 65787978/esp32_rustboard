/*
to build: cargo build --release
to flash: espflash flash ./target/riscv32imc-esp-espidf/release/esp32_rustboard --monitor
*/

use anyhow;
use embassy_futures::join::join;
use embassy_futures::select::select;
use esp32_rustboard::*;
use esp_idf_hal::task::block_on;
use heapless::FnvIndexMap;
use spin::Mutex;

use crate::ble::ble_send_keys;
use crate::config::config::*;
use crate::debounce::*;
use crate::matrix::{scan_grid, Key};

fn main() -> anyhow::Result<()> {
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    /* initialize keys pressed hashmap */
    let keys_pressed: Mutex<FnvIndexMap<Key, Debounce, PRESSED_KEYS_INDEXMAP_SIZE>> =
        Mutex::new(FnvIndexMap::new());

    /* run the tasks concurrently */
    block_on(async {
        select(
            ble_send_keys(&keys_pressed),
            join(scan_grid(&keys_pressed), calculate_debounce(&keys_pressed)),
        )
        .await;
    });

    Ok(())
}
