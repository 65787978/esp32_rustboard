/*
to flash:
espflash flash ../target/riscv32imc-esp-espidf/debug/esp32-rust-split-keyboard --monitor
*/
use chrono::Utc;
use esp32_rust_split_keyboard::{ColPins, KeyboardLeftSide, RowPins};
use esp_idf_hal::delay::FreeRtos;
// use esp_idf_hal::gpio::*;
// use esp_idf_hal::peripherals::Peripherals;
use esp_idf_svc::sys::link_patches;
use std::vec;

// enum Rows<'a> {
//     Row0(PinDriver<'a, Gpio2, Input>),
//     Row1(PinDriver<'a, Gpio3, Input>),
//     Row2(PinDriver<'a, Gpio10, Input>),
//     Row3(PinDriver<'a, Gpio6, Input>),
//     Row4(PinDriver<'a, Gpio7, Input>),
// }
// enum Cols<'a> {
//     Col0(PinDriver<'a, Gpio0, Input>),
//     Col1(PinDriver<'a, Gpio1, Input>),
//     Col2(PinDriver<'a, Gpio12, Input>),
//     Col3(PinDriver<'a, Gpio18, Input>),
//     Col4(PinDriver<'a, Gpio19, Input>),
//     Col5(PinDriver<'a, Gpio13, Input>),
// }

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Hello, world!");

    let rows = vec![
        RowPins::Row0,
        RowPins::Row1,
        RowPins::Row2,
        RowPins::Row3,
        RowPins::Row4,
    ];

    let cols = vec![
        ColPins::Col0,
        ColPins::Col1,
        ColPins::Col2,
        ColPins::Col3,
        ColPins::Col4,
        ColPins::Col5,
    ];

    let mut keyboard = KeyboardLeftSide::new();
    keyboard.initialize_hashmap();

    let mut key_1_pressed: Option<(i32, i32)> = None;
    let mut key_2_pressed: Option<(i32, i32)> = None;
    let mut key_3_pressed: Option<(i32, i32)> = None;

    let mut prev_key_pressed = -1;

    loop {
        let start_timestamp = Utc::now().timestamp_millis();

        for row in &rows {
            match row.is_high().0 {
                true => {
                    if let None = key_1_pressed {
                        key_1_pressed.unwrap().0 = row.is_high().1;
                        prev_key_pressed = row.is_high().1;
                    } else if key_1_pressed.unwrap().0 != prev_key_pressed {
                        key_2_pressed.unwrap().0 = row.is_high().1;
                        prev_key_pressed = row.is_high().1;
                    } else if key_2_pressed.unwrap().0 != prev_key_pressed {
                        key_3_pressed.unwrap().0 = row.is_high().1;
                    }
                }
                false => {}
            }
        }

        for col in &cols {
            match col.is_high().0 {
                true => {
                    if let None = key_1_pressed {
                        key_1_pressed.unwrap().1 = col.is_high().1;
                        prev_key_pressed = col.is_high().1;
                    } else if key_1_pressed.unwrap().0 != prev_key_pressed {
                        key_2_pressed.unwrap().1 = col.is_high().1;
                        prev_key_pressed = col.is_high().1;
                    } else if key_2_pressed.unwrap().0 != prev_key_pressed {
                        key_3_pressed.unwrap().1 = col.is_high().1;
                    }
                }
                false => {}
            }
        }

        if let Some(key_1_valid) = keyboard.key.get(&key_1_pressed.unwrap()) {
            let mut keys_pressed: (&str, &str, &str) =
                ("not_pressed", "not_pressed", "not_pressed");

            keys_pressed.0 = *key_1_valid;

            if let Some(key_2_valid) = keyboard.key.get(&key_2_pressed.unwrap()) {
                keys_pressed.1 = *key_2_valid;

                if let Some(key_3_valid) = keyboard.key.get(&key_3_pressed.unwrap()) {
                    keys_pressed.2 = *key_3_valid;
                }
            }

            log::info!("Keys pressed: {:?}", keys_pressed);

            key_1_pressed = None;
            key_2_pressed = None;
            key_3_pressed = None;
        }

        let end_timestamp = Utc::now().timestamp_millis();
        log::info!("Total time taken: {}ms", end_timestamp - start_timestamp);

        FreeRtos::delay_ms(100);
    }

    // let peripherals = Peripherals::take().expect("msg");

    // let row_0: PinDriver<Gpio2, Input> = PinDriver::input(peripherals.pins.gpio2).expect("msg");
    // let row_1: PinDriver<Gpio3, Input> = PinDriver::input(peripherals.pins.gpio3).expect("msg");
    // let row_2: PinDriver<Gpio10, Input> = PinDriver::input(peripherals.pins.gpio10).expect("msg");
    // let row_3: PinDriver<Gpio6, Input> = PinDriver::input(peripherals.pins.gpio6).expect("msg");
    // let row_4: PinDriver<Gpio7, Input> = PinDriver::input(peripherals.pins.gpio7).expect("msg");

    // let col_0: PinDriver<Gpio0, Input> = PinDriver::input(peripherals.pins.gpio0).expect("msg");
    // let col_1: PinDriver<Gpio1, Input> = PinDriver::input(peripherals.pins.gpio1).expect("msg");
    // let col_2: PinDriver<Gpio12, Input> = PinDriver::input(peripherals.pins.gpio12).expect("msg");
    // let col_3: PinDriver<Gpio18, Input> = PinDriver::input(peripherals.pins.gpio18).expect("msg");
    // let col_4: PinDriver<Gpio19, Input> = PinDriver::input(peripherals.pins.gpio19).expect("msg");
    // let col_5: PinDriver<Gpio13, Input> = PinDriver::input(peripherals.pins.gpio13).expect("msg");

    // let row_vec = vec![
    //     Rows::Row0(row_0),
    //     Rows::Row1(row_1),
    //     Rows::Row2(row_2),
    //     Rows::Row3(row_3),
    //     Rows::Row4(row_4),
    // ];

    // let col_vec = vec![
    //     Cols::Col0(col_0),
    //     Cols::Col1(col_1),
    //     Cols::Col2(col_2),
    //     Cols::Col3(col_3),
    //     Cols::Col4(col_4),
    //     Cols::Col5(col_5),
    // ];

    // let mut keyboard = KeyboardLeftSide::new();

    // keyboard.initialize_hashmap();

    // let mut key_1_pressed: Option<(i32, i32)> = None;
    // let mut key_2_pressed: Option<(i32, i32)> = None;
    // let mut key_3_pressed: Option<(i32, i32)> = None;

    // loop {
    //     let start_timestamp = Utc::now().timestamp_millis();

    //     for _i in 0..3 {
    //         for row in &row_vec {
    //             match row {
    //                 Rows::Row0(key) => {
    //                     if key.is_high() {
    //                         if key_1_pressed.is_none() {
    //                             key_1_pressed.unwrap().0 = 2;
    //                         } else if key_2_pressed.is_none() && key_1_pressed.unwrap().0 != 2 {
    //                             key_2_pressed.unwrap().0 = 2;
    //                         } else if key_3_pressed.is_none()
    //                             && key_1_pressed.unwrap().0 != 2
    //                             && key_2_pressed.unwrap().0 != 2
    //                         {
    //                             key_3_pressed.unwrap().0 = 2;
    //                         }
    //                     }
    //                 }
    //                 Rows::Row1(key) => {
    //                     if key.is_high() {
    //                         if key_1_pressed.is_none() {
    //                             key_1_pressed.unwrap().0 = 3;
    //                         } else if key_2_pressed.is_none() && key_1_pressed.unwrap().0 != 3 {
    //                             key_2_pressed.unwrap().0 = 3;
    //                         } else if key_3_pressed.is_none()
    //                             && key_1_pressed.unwrap().0 != 3
    //                             && key_2_pressed.unwrap().0 != 3
    //                         {
    //                             key_3_pressed.unwrap().0 = 3;
    //                         }
    //                     }
    //                 }
    //                 Rows::Row2(key) => {
    //                     if key.is_high() {
    //                         if key_1_pressed.is_none() {
    //                             key_1_pressed.unwrap().0 = 10;
    //                         } else if key_2_pressed.is_none() && key_1_pressed.unwrap().0 != 10 {
    //                             key_2_pressed.unwrap().0 = 10;
    //                         } else if key_3_pressed.is_none()
    //                             && key_1_pressed.unwrap().0 != 10
    //                             && key_2_pressed.unwrap().0 != 10
    //                         {
    //                             key_3_pressed.unwrap().0 = 10;
    //                         }
    //                     }
    //                 }
    //                 Rows::Row3(key) => {
    //                     if key.is_high() {
    //                         if key_1_pressed.is_none() {
    //                             key_1_pressed.unwrap().0 = 6;
    //                         } else if key_2_pressed.is_none() && key_1_pressed.unwrap().0 != 6 {
    //                             key_2_pressed.unwrap().0 = 6;
    //                         } else if key_3_pressed.is_none()
    //                             && key_1_pressed.unwrap().0 != 6
    //                             && key_2_pressed.unwrap().0 != 6
    //                         {
    //                             key_3_pressed.unwrap().0 = 6;
    //                         }
    //                     }
    //                 }
    //                 Rows::Row4(key) => {
    //                     if key.is_high() {
    //                         if key_1_pressed.is_none() {
    //                             key_1_pressed.unwrap().0 = 7;
    //                         } else if key_2_pressed.is_none() && key_1_pressed.unwrap().0 != 7 {
    //                             key_2_pressed.unwrap().0 = 7;
    //                         } else if key_3_pressed.is_none()
    //                             && key_1_pressed.unwrap().0 != 7
    //                             && key_2_pressed.unwrap().0 != 7
    //                         {
    //                             key_3_pressed.unwrap().0 = 7;
    //                         }
    //                     }
    //                 }
    //             }
    //         }

    //         for col in &col_vec {
    //             match col {
    //                 Cols::Col0(key) => {
    //                     if key.is_high() {
    //                         if key_1_pressed.is_none() {
    //                             key_1_pressed.unwrap().1 = 0;
    //                         } else if key_2_pressed.is_none() && key_1_pressed.unwrap().1 != 0 {
    //                             key_2_pressed.unwrap().1 = 0;
    //                         } else if key_3_pressed.is_none()
    //                             && key_1_pressed.unwrap().1 != 0
    //                             && key_2_pressed.unwrap().1 != 0
    //                         {
    //                             key_3_pressed.unwrap().1 = 0;
    //                         }
    //                     }
    //                 }
    //                 Cols::Col1(key) => {
    //                     if key.is_high() {
    //                         if key_1_pressed.is_none() {
    //                             key_1_pressed.unwrap().1 = 1;
    //                         } else if key_2_pressed.is_none() && key_1_pressed.unwrap().1 != 1 {
    //                             key_2_pressed.unwrap().1 = 1;
    //                         } else if key_3_pressed.is_none()
    //                             && key_1_pressed.unwrap().1 != 1
    //                             && key_2_pressed.unwrap().1 != 1
    //                         {
    //                             key_3_pressed.unwrap().1 = 1;
    //                         }
    //                     }
    //                 }
    //                 Cols::Col2(key) => {
    //                     if key.is_high() {
    //                         if key_1_pressed.is_none() {
    //                             key_1_pressed.unwrap().1 = 12;
    //                         } else if key_2_pressed.is_none() && key_1_pressed.unwrap().1 != 12 {
    //                             key_2_pressed.unwrap().1 = 12;
    //                         } else if key_3_pressed.is_none()
    //                             && key_1_pressed.unwrap().1 != 12
    //                             && key_2_pressed.unwrap().1 != 12
    //                         {
    //                             key_3_pressed.unwrap().1 = 12;
    //                         }
    //                     }
    //                 }
    //                 Cols::Col3(key) => {
    //                     if key.is_high() {
    //                         if key_1_pressed.is_none() {
    //                             key_1_pressed.unwrap().1 = 18;
    //                         } else if key_2_pressed.is_none() && key_1_pressed.unwrap().1 != 18 {
    //                             key_2_pressed.unwrap().1 = 18;
    //                         } else if key_3_pressed.is_none()
    //                             && key_1_pressed.unwrap().1 != 18
    //                             && key_2_pressed.unwrap().1 != 18
    //                         {
    //                             key_3_pressed.unwrap().1 = 18;
    //                         }
    //                     }
    //                 }
    //                 Cols::Col4(key) => {
    //                     if key.is_high() {
    //                         if key_1_pressed.is_none() {
    //                             key_1_pressed.unwrap().1 = 19;
    //                         } else if key_2_pressed.is_none() && key_1_pressed.unwrap().1 != 19 {
    //                             key_2_pressed.unwrap().1 = 19;
    //                         } else if key_3_pressed.is_none()
    //                             && key_1_pressed.unwrap().1 != 19
    //                             && key_2_pressed.unwrap().1 != 19
    //                         {
    //                             key_3_pressed.unwrap().1 = 19;
    //                         }
    //                     }
    //                 }
    //                 Cols::Col5(key) => {
    //                     if key.is_high() {
    //                         if key_1_pressed.is_none() {
    //                             key_1_pressed.unwrap().1 = 13;
    //                         } else if key_2_pressed.is_none() && key_1_pressed.unwrap().1 != 13 {
    //                             key_2_pressed.unwrap().1 = 13;
    //                         } else if key_3_pressed.is_none()
    //                             && key_1_pressed.unwrap().1 != 13
    //                             && key_2_pressed.unwrap().1 != 13
    //                         {
    //                             key_3_pressed.unwrap().1 = 13;
    //                         }
    //                     }
    //                 }
    //             }
    //         }

    //         if let Some(key_1_valid) = keyboard.key.get(&key_1_pressed.unwrap()) {
    //             let mut keys_pressed: (&str, &str, &str) =
    //                 ("not_pressed", "not_pressed", "not_pressed");

    //             keys_pressed.0 = *key_1_valid;

    //             if let Some(key_2_valid) = keyboard.key.get(&key_2_pressed.unwrap()) {
    //                 keys_pressed.1 = *key_2_valid;

    //                 if let Some(key_3_valid) = keyboard.key.get(&key_3_pressed.unwrap()) {
    //                     keys_pressed.2 = *key_3_valid;
    //                 }
    //             }

    //             log::info!("Keys pressed: {:?}", keys_pressed);

    //             key_1_pressed = None;
    //             key_2_pressed = None;
    //             key_3_pressed = None;
    //         }
    //     }

    //     let end_timestamp = Utc::now().timestamp_millis();
    //     log::info!("Total time taken: {}ms", end_timestamp - start_timestamp);

    //     FreeRtos::delay_ms(100);
    // }
}
