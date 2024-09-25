/*
to flash:
espflash flash ./target/riscv32imc-esp-espidf/release/esp32-rust-split-keyboard --monitor
*/

use crate::ble_keyboard::*;
use crate::delay::delay_ms;

use anyhow;
use embassy_futures::select::select3;
use enums::HidMapings;
use esp32_rust_split_keyboard::*;
use esp_idf_hal::task::block_on;
use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};

static ATOMIC_KEY_ROW: AtomicI32 = AtomicI32::new(0);
static ATOMIC_KEY_COL: AtomicI32 = AtomicI32::new(0);
static ATOMIC_KEY_BOOL: AtomicBool = AtomicBool::new(false);

static ATOMIC_MODIFIER_ROW: AtomicI32 = AtomicI32::new(0);
static ATOMIC_MODIFIER_COL: AtomicI32 = AtomicI32::new(0);
static ATOMIC_MODIFIER_BOOL: AtomicBool = AtomicBool::new(false);

fn main() -> anyhow::Result<()> {
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    /* initialize matrix */
    let mut keyboard_left_side = KeyboardSide::new();

    /* Run the tasks in parallel */
    block_on(async {
        select3(
            ble_transmit(),
            key_matrix(&mut keyboard_left_side.key_matrix),
            modifier_matrix(&mut keyboard_left_side.modifier_matrix),
        )
        .await;
    });

    Ok(())
}

async fn ble_transmit() -> ! {
    /* initialize BLE */
    let mut keyboard = Keyboard::new().unwrap();

    println!("BLE Initialized...");

    /* initialize layers */
    let mut layers = Layers::new();
    layers.initialie_base_layer();
    layers.initialie_modifier_layer();

    let mut key: HidMapings = HidMapings::None;
    let mut modifier: HidMapings = HidMapings::None;

    loop {
        /* check if connected */
        if keyboard.connected() {
            /* wait 10 ms */
            delay_ms(1).await;

            /* check if a key has been pressed */
            if ATOMIC_KEY_BOOL.load(Ordering::Relaxed) {
                /* get valid key */
                if let Some(valid_key) = layers.base_layer.get(&(
                    ATOMIC_KEY_ROW.load(Ordering::Relaxed),
                    ATOMIC_KEY_COL.load(Ordering::Relaxed),
                )) {
                    /* set key */
                    key = *valid_key;
                } else {
                    key = HidMapings::None;
                }

                /* reset bool */
                ATOMIC_KEY_BOOL.store(false, Ordering::Relaxed);
            }

            if ATOMIC_MODIFIER_BOOL.load(Ordering::Relaxed) {
                /* get valid modifier key */
                if let Some(valid_modifier_key) = layers.modifier_layer.get(&(
                    ATOMIC_MODIFIER_ROW.load(Ordering::Relaxed),
                    ATOMIC_MODIFIER_COL.load(Ordering::Relaxed),
                )) {
                    /* set modifier */
                    modifier = *valid_modifier_key;
                } else {
                    modifier = HidMapings::None;
                }
                /* set bool */
                ATOMIC_MODIFIER_BOOL.store(false, Ordering::Relaxed);
            }

            if (key != HidMapings::None) || (modifier != HidMapings::None) {
                /* send press key */
                keyboard.press(key, modifier);
                keyboard.release();

                /* reset key and modifier */
                key = HidMapings::None;
                modifier = HidMapings::None;
            }
        }

        delay_ms(1).await;
    }
}
async fn key_matrix(matrix: &mut KeyMatrix<'_>) -> ! {
    let mut store_values_flag: bool = true;

    loop {
        /* check rows and cols */
        for row in matrix.rows.iter_mut() {
            /* set row to high */
            row.set_high().unwrap();

            /* check if a col is high */
            for col in matrix.cols.iter_mut() {
                /* if a col is high */
                while col.is_high() {
                    if store_values_flag {
                        ATOMIC_KEY_ROW.store(row.pin(), Ordering::Relaxed);
                        ATOMIC_KEY_COL.store(col.pin(), Ordering::Relaxed);

                        /* set bool */
                        ATOMIC_KEY_BOOL.store(true, Ordering::Relaxed);
                        store_values_flag = false;
                    }

                    delay_ms(1).await;
                }
                store_values_flag = true;
            }

            /* set row to low */
            row.set_low().unwrap();

            /* Wait 1 ms */
            delay_ms(1).await;
        }
    }
}

async fn modifier_matrix(matrix: &mut ModifierMatrix<'_>) -> ! {
    let mut store_values_flag: bool = true;

    /* set modifier row to always high */
    matrix.rows[0].set_high().unwrap();

    loop {
        /* check if a col is high */
        for col in matrix.cols.iter_mut() {
            /* if a col is high */
            while col.is_high() {
                if store_values_flag {
                    ATOMIC_MODIFIER_ROW.store(matrix.rows[0].pin(), Ordering::Relaxed);
                    ATOMIC_MODIFIER_COL.store(col.pin(), Ordering::Relaxed);

                    /* set bool */
                    ATOMIC_MODIFIER_BOOL.store(true, Ordering::Relaxed);
                    store_values_flag = false;
                }
                delay_ms(1).await;
            }
            store_values_flag = true;
        }
        /* Wait 1 ms */
        delay_ms(1).await;
    }
}
