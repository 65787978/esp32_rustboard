/*
to build: cargo build --release
to flash: espflash flash ./target/riscv32imc-esp-espidf/release/esp32-rust-split-keyboard --monitor
*/

use crate::ble_keyboard::*;

use anyhow;
use enums::HidMapings;
use esp32_rust_split_keyboard::*;
use esp_idf_svc::hal::delay::FreeRtos;

fn main() -> anyhow::Result<()> {
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    /* initialize BLE */
    let mut ble_connection = Keyboard::new().unwrap();

    log::info!("BLE Initialized...");

    /* initialize matrix */
    let mut keyboard = KeyboardSide::new();
    keyboard.initialize_base_layer();

    log::info!("Keyboard Initialized...");

    /* initialize bool matrix */
    let mut bool_matrix: [[bool; COLS]; ROWS] = [[false; COLS]; ROWS];

    /* Run the tasks in parallel */
    loop {
        if ble_connection.connected() {
            poll_matrix(&mut keyboard.key_matrix, &mut bool_matrix);
        }
    }

    Ok(())
}

fn ble_transmit() -> ! {
    loop {
        /* check if connected */
        if keyboard.connected() {
            /* wait 10 ms */
            FreeRtos::delay_ms(1);

            /* send press key */
            keyboard.press(key, modifier);
            keyboard.release();
        }

        /* Delay so wdt doesn't kick in */
        FreeRtos::delay_ms(1);
    }
}
fn poll_matrix(matrix: &mut KeyMatrix<'_>, bool_matrix: &mut [[bool; COLS]; ROWS]) -> ! {
    let mut arr_row: usize;
    let mut arr_col: usize;

    loop {
        /* reset variables */
        arr_row = 0;
        arr_col = 0;

        /* check rows and cols */
        for row in matrix.rows.iter_mut() {
            /* set row to high */
            row.set_high().unwrap();

            /* delay so pin can propagate */
            FreeRtos::delay_ms(1);

            /* check if a col is high */
            for col in matrix.cols.iter_mut() {
                /* if a col is high */
                if col.is_high() {
                    /* store pressed keys */
                    bool_matrix[arr_col][arr_row] = true;
                }
                /* increment col */
                arr_col += 1;
            }

            /* set row to low */
            row.set_low().unwrap();

            /* increment row */
            arr_row += 1;
        }
    }
}
