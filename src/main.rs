/*
to flash:
espflash flash ../target/riscv32imc-esp-espidf/debug/esp32-rust-split-keyboard --monitor
*/
use chrono::Utc;
use esp32_rust_split_keyboard::KeyboardLeftSide;
use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::gpio::*;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_svc::sys::link_patches;
use std::vec;

enum Rows<'a> {
    Row0(PinDriver<'a, Gpio2, Input>),
    Row1(PinDriver<'a, Gpio3, Input>),
    Row2(PinDriver<'a, Gpio10, Input>),
    Row3(PinDriver<'a, Gpio6, Input>),
    Row4(PinDriver<'a, Gpio7, Input>),
}
enum Cols<'a> {
    Col0(PinDriver<'a, Gpio0, Input>),
    Col1(PinDriver<'a, Gpio1, Input>),
    Col2(PinDriver<'a, Gpio12, Input>),
    Col3(PinDriver<'a, Gpio18, Input>),
    Col4(PinDriver<'a, Gpio19, Input>),
    Col5(PinDriver<'a, Gpio13, Input>),
}

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Hello, world!");

    let peripherals = Peripherals::take().expect("msg");

    let row_0: PinDriver<Gpio2, Input> = PinDriver::input(peripherals.pins.gpio2).expect("msg");
    let row_1: PinDriver<Gpio3, Input> = PinDriver::input(peripherals.pins.gpio3).expect("msg");
    let row_2: PinDriver<Gpio10, Input> = PinDriver::input(peripherals.pins.gpio10).expect("msg");
    let row_3: PinDriver<Gpio6, Input> = PinDriver::input(peripherals.pins.gpio6).expect("msg");
    let row_4: PinDriver<Gpio7, Input> = PinDriver::input(peripherals.pins.gpio7).expect("msg");

    let col_0: PinDriver<Gpio0, Input> = PinDriver::input(peripherals.pins.gpio0).expect("msg");
    let col_1: PinDriver<Gpio1, Input> = PinDriver::input(peripherals.pins.gpio1).expect("msg");
    let col_2: PinDriver<Gpio12, Input> = PinDriver::input(peripherals.pins.gpio12).expect("msg");
    let col_3: PinDriver<Gpio18, Input> = PinDriver::input(peripherals.pins.gpio18).expect("msg");
    let col_4: PinDriver<Gpio19, Input> = PinDriver::input(peripherals.pins.gpio19).expect("msg");
    let col_5: PinDriver<Gpio13, Input> = PinDriver::input(peripherals.pins.gpio13).expect("msg");

    let row_vec = vec![
        Rows::Row0(row_0),
        Rows::Row1(row_1),
        Rows::Row2(row_2),
        Rows::Row3(row_3),
        Rows::Row4(row_4),
    ];

    let col_vec = vec![
        Cols::Col0(col_0),
        Cols::Col1(col_1),
        Cols::Col2(col_2),
        Cols::Col3(col_3),
        Cols::Col4(col_4),
        Cols::Col5(col_5),
    ];

    let mut keyboard = KeyboardLeftSide::new();

    keyboard.initialize_hashmap();

    let mut key_pressed = (-1, -1);

    loop {
        let start_timestamp = Utc::now().timestamp_millis();

        for row in &row_vec {
            match row {
                Rows::Row0(key) => {
                    if key.is_high() {
                        key_pressed.0 = 2;
                    }
                }
                Rows::Row1(key) => {
                    if key.is_high() {
                        key_pressed.0 = 3;
                    }
                }
                Rows::Row2(key) => {
                    if key.is_high() {
                        key_pressed.0 = 10;
                    }
                }
                Rows::Row3(key) => {
                    if key.is_high() {
                        key_pressed.0 = 6;
                    }
                }
                Rows::Row4(key) => {
                    if key.is_high() {
                        key_pressed.0 = 7;
                    }
                }
            }
        }

        for col in &col_vec {
            match col {
                Cols::Col0(key) => {
                    if key.is_high() {
                        key_pressed.1 = 0;
                    }
                }
                Cols::Col1(key) => {
                    if key.is_high() {
                        key_pressed.1 = 1;
                    }
                }
                Cols::Col2(key) => {
                    if key.is_high() {
                        key_pressed.1 = 12;
                    }
                }
                Cols::Col3(key) => {
                    if key.is_high() {
                        key_pressed.1 = 18;
                    }
                }
                Cols::Col4(key) => {
                    if key.is_high() {
                        key_pressed.1 = 19;
                    }
                }
                Cols::Col5(key) => {
                    if key.is_high() {
                        key_pressed.1 = 13;
                    }
                }
            }
        }

        if let Some(key_valid) = keyboard.key.get(&key_pressed) {
            log::info!("Key pressed: {:?}", *key_valid);

            key_pressed = (-1, -1);
        }

        let end_timestamp = Utc::now().timestamp_millis();
        log::info!("Total time taken: {}ms", end_timestamp - start_timestamp);

        FreeRtos::delay_ms(100);
    }

    // let rows = vec![
    // Rows::Row0(PinDriver::input(peripherals.pins.gpio2).expect("msg")),
    // Rows::Row1(PinDriver::input(peripherals.pins.gpio3).expect("msg")),
    //     Rows::Row2(PinDriver::input(peripherals.pins.gpio10).expect("msg")),
    //     Rows::Row3(PinDriver::input(peripherals.pins.gpio6).expect("msg")),
    //     Rows::Row4(PinDriver::input(peripherals.pins.gpio7).expect("msg")),
    // ];

    // let cols = vec![
    //     Cols::Col0(PinDriver::input(peripherals.pins.gpio0).expect("msg")),
    //     Cols::Col1(PinDriver::input(peripherals.pins.gpio1).expect("msg")),
    //     Cols::Col2(PinDriver::input(peripherals.pins.gpio12).expect("msg")),
    //     Cols::Col3(PinDriver::input(peripherals.pins.gpio18).expect("msg")),
    //     Cols::Col4(PinDriver::input(peripherals.pins.gpio19).expect("msg")),
    //     Cols::Col5(PinDriver::input(peripherals.pins.gpio13).expect("msg")),
    // ];

    // let col_0 = PinDriver::input(peripherals.pins.gpio0).expect("msg");
    // let col_1 = PinDriver::input(peripherals.pins.gpio1).expect("msg");
    // let col_2 = PinDriver::input(peripherals.pins.gpio12).expect("msg");
    // let col_3 = PinDriver::input(peripherals.pins.gpio18).expect("msg");
    // let col_4 = PinDriver::input(peripherals.pins.gpio19).expect("msg");
    // let col_5 = PinDriver::input(peripherals.pins.gpio13).expect("msg");

    // let row_0 = PinDriver::input(peripherals.pins.gpio2).expect("msg");
    // let row_1 = PinDriver::input(peripherals.pins.gpio3).expect("msg");
    // let row_2 = PinDriver::input(peripherals.pins.gpio10).expect("msg");
    // let row_3 = PinDriver::input(peripherals.pins.gpio6).expect("msg");
    // let row_4 = PinDriver::input(peripherals.pins.gpio7).expect("msg");

    // let mut rows = Vec::new();

    // let mut col_vec = vec![col_0, col_1, col_2, col_3, col_4, col_5];
    // let mut row_vec = vec![row_0, row_1, row_2, row_3, row_4];

    // let mut keyboard = KeyboardLeftSide::new();

    // keyboard.initialize_hashmap();

    // loop {
    //     // if row_0.is_high() && col_0.is_high() {
    //     //     log::info!("ESC is pressed: PIN: {:?}", keyboard.key.keys());
    //     // }

    //     FreeRtos::delay_ms(100);
    // }
    // /* Declare keys */
    // let mut keyboard_left_side = KeyboardLeftSide::new();
    // keyboard_left_side.initialize_keys();

    // let mut has_key_been_pressed_this_cycle = false;
    // let mut previous_key_pressed: (u8, u8) = (0, 0);

    // loop {
    //     /* Implement hardware matrix scan */
    //     let key_pressed_matrix: Option<(u8, u8)> = Some((0, 0));

    //     match key_pressed_matrix {
    //         Some(key_pressed) => {
    //             if let Some(key) = keyboard_left_side.key.get_mut(&key_pressed) {
    //                 *key = true;
    //                 previous_key_pressed = key_pressed;
    //                 has_key_been_pressed_this_cycle = true;
    //                 // log::info!("Key pressed: {:?}", key_pressed);

    //                 if key_pin.is_low() {
    //                     log::info!("Key is HIGH")
    //                 } else {
    //                     log::info!("Key is LOW")
    //                 }
    //             }
    //         }

    //         /* If a key is not pressed, set the previous key to false */
    //         None => {
    //             if has_key_been_pressed_this_cycle {
    //                 keyboard_left_side.key.insert(previous_key_pressed, false);
    //                 has_key_been_pressed_this_cycle = false;
    //             }
    //         }
    //     }

    //     /* Sleep for 20 milliseconds before fetching the matrix */
    //     FreeRtos::delay_ms(20);
    // }
}
