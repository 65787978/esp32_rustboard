/*
to build: cargo build --release
to flash: espflash flash ./target/riscv32imc-esp-espidf/release/esp32-rust-split-keyboard --monitor
*/

use crate::ble_keyboard::*;

use anyhow;
use enums::HidMapings;
use esp32_rust_split_keyboard::*;

fn main() -> anyhow::Result<()> {
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    /* initialize BLE */
    let mut ble_connection = Keyboard::new().unwrap();

    log::info!("BLE Initialized...");

    /* initialize matrix */
    let mut keyboard = KeyboardLeftSide::new();
    keyboard.initialize_base_layer();

    log::info!("Keyboard Initialized...");

    /* initialize bool matrix */
    let mut bool_matrix: [[bool; COLS]; ROWS] = [[false; COLS]; ROWS];

    /* Run the main loop */
    loop {
        if ble_connection.connected() {
            poll_matrix(&mut keyboard.key_matrix, &mut bool_matrix).unwrap();

            /* */
            let keys = get_keys_pressed(&mut bool_matrix);

            if !keys.is_empty() {
                for key in keys {
                    if key == LAYER_KEY {
                        match keyboard.layer {
                            Layer::Base => keyboard.layer = Layer::Upper,
                            Layer::Upper => keyboard.layer = Layer::Base,
                        }
                    }

                    match keyboard.layer {
                        Layer::Base => {
                            if let Some(valid_key) = keyboard.base_layer.get(&key) {
                                // if *valid_key ==
                                let modifier = HidMapings::None;

                                ble_connection.press(*valid_key, modifier);
                                ble_connection.release();
                            }
                        }
                        Layer::Upper => {
                            todo!("Not yet implemented...");
                        }
                    }
                }
            }
            delay::delay_us(100);
        } else {
            log::info!("Keyboard not connected!");
            delay::delay_us(100000);
        }
    }
}

fn poll_matrix(
    matrix: &mut KeyMatrix<'_>,
    bool_matrix: &mut [[bool; COLS]; ROWS],
) -> anyhow::Result<()> {
    /* initialize counts */
    let mut row_count: usize = 0;
    let mut col_count: usize = 0;

    /* check rows and cols */
    for row in matrix.rows.iter_mut() {
        /* set row to high */
        row.set_high().unwrap();

        /* delay so pin can propagate */
        delay::delay_us(10);

        /* check if a col is high */
        for col in matrix.cols.iter_mut() {
            /* if a col is high */
            if col.is_high() {
                /* store pressed keys */
                bool_matrix[row_count][col_count] = true;
            }
            /* increment col */
            col_count += 1;
        }

        /* set row to low */
        row.set_low().unwrap();

        /* increment row */
        row_count += 1;

        /* reset col count */
        col_count = 0;
    }

    Ok(())
}

fn get_keys_pressed(bool_matrix: &mut [[bool; COLS]; ROWS]) -> Vec<(i8, i8)> {
    /* initialize variables */
    let mut keys_pressed: Vec<(i8, i8)> = Vec::new();
    let (mut row_count, mut col_count): (i8, i8) = (0, 0);

    /* itter the matrix */
    for row in bool_matrix {
        /* itter col */
        for col in row {
            /* if an element is true */
            if *col {
                /* store it */
                keys_pressed.push((row_count, col_count));
                *col = false;
            }

            /* increment col count */
            col_count += 1;
        }

        /* increment row count */
        row_count += 1;

        /* reset col count */
        col_count = 0;
    }

    /* return pressed keys */
    keys_pressed
}
