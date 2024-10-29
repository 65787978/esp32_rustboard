/*
to build: cargo build --release
to flash: espflash flash ./target/riscv32imc-esp-espidf/release/esp32_rustboard --monitor
*/
extern crate alloc;

use crate::ble_keyboard::*;
use alloc::sync::Arc;
use anyhow;
use embassy_futures::select::select;
use embassy_time::Instant;
use esp32_rustboard::*;
use esp_idf_hal::task::block_on;
use heapless::FnvIndexMap;
use spin::Mutex;

fn main() -> anyhow::Result<()> {
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    /* initialize BLE */
    let mut ble_keyboard = BleKeyboard::new().unwrap();
    log::info!("BLE Keyboard Initialized...");

    /* initialize matrix */
    let mut pin_matrix = PinMatrix::new();
    log::info!("Pin Matrix Initialized...");

    /* initialize keys pressed hashmap */
    let keys_pressed: Arc<
        Mutex<FnvIndexMap<(i8, i8), (Instant, bool), PRESSED_KEYS_INDEXMAP_SIZE>>,
    > = Arc::new(Mutex::new(FnvIndexMap::new()));

    /* run the tasks concurrently */
    block_on(async {
        select(
            ble_keyboard.send_key(&keys_pressed),
            pin_matrix.scan_grid(&keys_pressed),
        )
        .await;
    });

    Ok(())
}
