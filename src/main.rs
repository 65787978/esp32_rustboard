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

use crate::ble::BleKeyboard;
use crate::config::config::*;
use crate::debounce::*;
use crate::matrix::PinMatrix;

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
    let keys_pressed: Mutex<FnvIndexMap<(i8, i8), Debounce, PRESSED_KEYS_INDEXMAP_SIZE>> =
        Mutex::new(FnvIndexMap::new());

    /* run the tasks concurrently */
    block_on(async {
        select(
            ble_keyboard.send_key(&keys_pressed),
            join(
                pin_matrix.scan_grid(&keys_pressed),
                calculate_debounce(&keys_pressed),
            ),
        )
        .await;
    });

    Ok(())
}
