/*
to flash:
espflash flash ../target/riscv32imc-esp-espidf/debug/esp32-rust-split-keyboard --monitor
*/

use crate::ble_keyboard::*;
use anyhow;
use embassy_futures::select::select;
use embassy_time::{Duration, Timer};
use esp32_rust_split_keyboard::*;
use esp_idf_hal::task::block_on;
use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};
static ATOMIC_ROW: AtomicI32 = AtomicI32::new(0);
static ATOMIC_COL: AtomicI32 = AtomicI32::new(0);
static ATOMIC_BOOL: AtomicBool = AtomicBool::new(false);

fn main() -> anyhow::Result<()> {
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    /* Run the tasks in parallel */
    block_on(async {
        select(ble_transmit(), matrix()).await;
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

    loop {
        /* check if connected */
        if keyboard.connected() {
            /* wait 10 ms */
            delay_ms(10).await;

            /* check if a key has been pressed */
            if ATOMIC_BOOL.load(Ordering::Relaxed) {
                /* get valid key */
                if let Some(valid_key) = layers.base_layer.get(&(
                    ATOMIC_ROW.load(Ordering::Relaxed),
                    ATOMIC_COL.load(Ordering::Relaxed),
                )) {
                    /* send press key */
                    println!("Valid_Key = {:?}", *valid_key);
                    keyboard.press(*valid_key);
                    keyboard.release();
                }

                /* reset bool */
                ATOMIC_BOOL.store(false, Ordering::Relaxed);
            }
        }
    }
}

async fn matrix() -> ! {
    /* initialize matrix */
    let mut keyboard_left_side = KeyboardSide::new();

    loop {
        /* check rows and cols */
        for row in keyboard_left_side.key_matrix.rows.iter_mut() {
            /* set row to high */
            row.set_high().unwrap();

            /* check if a col is high */
            for col in keyboard_left_side.key_matrix.cols.iter_mut() {
                /* if a col is high */
                if col.is_high() {
                    ATOMIC_ROW.store(row.pin(), Ordering::Relaxed);
                    ATOMIC_COL.store(col.pin(), Ordering::Relaxed);

                    /* set bool */
                    ATOMIC_BOOL.store(true, Ordering::Relaxed);
                }
            }

            /* set row to low */
            row.set_low().unwrap();

            /* Wait 1 ms */
            delay_ms(1).await;
        }
    }
}

async fn delay_ms(ms: u32) {
    let delay = Duration::from_millis(ms as u64);
    Timer::after(delay).await;
}
