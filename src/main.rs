/*
to flash:
espflash flash ./target/riscv32imc-esp-espidf/release/esp32-rust-split-keyboard --monitor
*/

use crate::ble_keyboard::*;
use anyhow;
use embassy_futures::select::select3;
use embassy_time::{Duration, Timer};
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

    let mut key: u8 = 0;
    loop {
        /* check if connected */
        if keyboard.connected() {
            /* wait 10 ms */
            delay_ms(10).await;

            /* check if a key has been pressed */
            if ATOMIC_KEY_BOOL.load(Ordering::Relaxed) {
                /* get valid key */
                if let Some(valid_key) = layers.base_layer.get(&(
                    ATOMIC_KEY_ROW.load(Ordering::Relaxed),
                    ATOMIC_KEY_COL.load(Ordering::Relaxed),
                )) {
                    key = *valid_key;
                }
            }

            if ATOMIC_MODIFIER_BOOL.load(Ordering::Relaxed) {
                /* get valid modifier key */
                if let Some(valid_modifier_key) = layers.modifier_layer.get(&(
                    ATOMIC_MODIFIER_ROW.load(Ordering::Relaxed),
                    ATOMIC_MODIFIER_COL.load(Ordering::Relaxed),
                )) {
                    match *valid_modifier_key {
                        1 => keyboard.key_report.modifiers |= 0x01,

                        2 => keyboard.key_report.modifiers |= 0x02,

                        44 => key = HidMapings::Space as u8,

                        _ => {}
                    }
                }

                /* reset bool */
                ATOMIC_MODIFIER_BOOL.store(false, Ordering::Relaxed);
            }

            /* send press key */
            keyboard.press(key);
            keyboard.release();

            /* reset bool */
            ATOMIC_KEY_BOOL.store(false, Ordering::Relaxed);
        }
    }
}

async fn key_matrix(matrix: &mut KeyMatrix<'_>) -> ! {
    loop {
        /* check rows and cols */
        for row in matrix.rows.iter_mut() {
            /* set row to high */
            row.set_high().unwrap();

            /* check if a col is high */
            for col in matrix.cols.iter_mut() {
                /* if a col is high */
                while col.is_high() {
                    ATOMIC_KEY_ROW.store(row.pin(), Ordering::Relaxed);
                    ATOMIC_KEY_COL.store(col.pin(), Ordering::Relaxed);

                    /* set bool */
                    ATOMIC_KEY_BOOL.store(true, Ordering::Relaxed);
                }
            }

            /* set row to low */
            row.set_low().unwrap();

            /* Wait 1 ms */
            delay_ms(1).await;
        }
    }
}

async fn modifier_matrix(matrix: &mut ModifierMatrix<'_>) -> ! {
    /* set modifier row to always high */
    matrix.rows[0].set_high().unwrap();

    loop {
        /* check if a col is high */
        for col in matrix.cols.iter_mut() {
            /* if a col is high */
            while col.is_high() {
                ATOMIC_MODIFIER_ROW.store(matrix.rows[0].pin(), Ordering::Relaxed);
                ATOMIC_MODIFIER_COL.store(col.pin(), Ordering::Relaxed);

                /* set bool */
                ATOMIC_MODIFIER_BOOL.store(true, Ordering::Relaxed);
            }
        }

        /* Wait 1 ms */
        delay_ms(1).await;
    }
}
async fn delay_ms(ms: u32) {
    let delay = Duration::from_millis(ms as u64);
    Timer::after(delay).await;
}
