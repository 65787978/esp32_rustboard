/*
to flash:
espflash flash ../target/riscv32imc-esp-espidf/debug/esp32-rust-split-keyboard --monitor
*/
use esp32_rust_split_keyboard::*;
use esp_idf_svc::hal::delay::FreeRtos;
use esp_idf_svc::hal::gpio::*;
use esp_idf_svc::hal::peripherals::Peripherals;

fn main() {
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Hello, world!");

    let peripherals = Peripherals::take().unwrap();

    /* rows */
    let gpio0 = &mut PinDriver::output(peripherals.pins.gpio0).unwrap();
    let gpio1 = &mut PinDriver::output(peripherals.pins.gpio1).unwrap();
    let gpio12 = &mut PinDriver::output(peripherals.pins.gpio12).unwrap();
    let gpio18 = &mut PinDriver::output(peripherals.pins.gpio18).unwrap();
    let gpio19 = &mut PinDriver::output(peripherals.pins.gpio19).unwrap();

    /* cols */
    let gpio2 = &mut PinDriver::input(peripherals.pins.gpio2).unwrap();
    let gpio3 = &mut PinDriver::input(peripherals.pins.gpio3).unwrap();
    let gpio10 = &mut PinDriver::input(peripherals.pins.gpio10).unwrap();
    let gpio6 = &mut PinDriver::input(peripherals.pins.gpio6).unwrap();
    let gpio7 = &mut PinDriver::input(peripherals.pins.gpio7).unwrap();
    let gpio4 = &mut PinDriver::input(peripherals.pins.gpio4).unwrap();

    let mut row_pin_active: u32 = 0;

    let mut keyboard_left = KeyboardLeftSide::new();
    keyboard_left.initialize_layers();

    let mut pins_active = (PIN_INACTIVE, PIN_INACTIVE);

    let delay = DELAY_DEFAULT;

    let mut layer;

    loop {
        match row_pin_active {
            0 => {
                gpio0.set_high().unwrap();
                pins_active.0 = gpio0.pin()
            }
            1 => {
                gpio1.set_high().unwrap();
                pins_active.0 = gpio1.pin()
            }
            2 => {
                gpio12.set_high().unwrap();
                pins_active.0 = gpio12.pin()
            }
            3 => {
                gpio18.set_high().unwrap();
                pins_active.0 = gpio18.pin()
            }
            4 => {
                gpio19.set_high().unwrap();
                pins_active.0 = gpio19.pin()
            }
            _ => {}
        }

        /* Shift pressed */
        if gpio7.is_high() && gpio19.is_set_high() {
            layer = Layer::Shift;
        }
        /* Upper layer pressed */
        else if gpio10.is_high() && gpio19.is_set_high() {
            layer = Layer::Upper;
        }
        /* Noting is pressed */
        else {
            layer = Layer::Base;
        }

        check_pins(&mut pins_active, gpio2, gpio3, gpio10, gpio6, gpio7, gpio4);

        match layer {
            Layer::Base => {
                /* Check if the pins pressed have a valid combination in the hashmap */
                if let Some(valid_key) = keyboard_left.base_layer.get(&pins_active) {
                    /* If the previos key is same as the active key */

                    log::info!("{}", *valid_key);
                }
            }
            Layer::Shift => {
                /* Check if the pins pressed have a valid combination in the hashmap */
                if let Some(valid_key) = keyboard_left.shift_layer.get(&pins_active) {
                    /* If the previos key is same as the active key */

                    log::info!("{}", *valid_key);
                }
            }
            Layer::Upper => {
                /* Check if the pins pressed have a valid combination in the hashmap */
                if let Some(valid_key) = keyboard_left.upper_layer.get(&pins_active) {
                    /* If the previos key is same as the active key */

                    log::info!("{}", *valid_key);
                }
            }
        }

        match row_pin_active {
            0 => gpio0.set_low().unwrap(),
            1 => gpio1.set_low().unwrap(),
            2 => gpio12.set_low().unwrap(),
            3 => gpio18.set_low().unwrap(),
            4 => gpio19.set_low().unwrap(),
            _ => {}
        }

        row_pin_active = (row_pin_active + 1) % 5;

        /* reset pins_active */
        pins_active = (PIN_INACTIVE, PIN_INACTIVE);

        FreeRtos::delay_ms(delay);
    }
}
