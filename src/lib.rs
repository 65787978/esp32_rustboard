/*
*********************************************************************************************
BASE LAYER:

X \ Y|  0  |  1  |  2  |  3  |  4  |  5  |           X \ Y|  0  |  1  |  2  |  3  |  4  |  5 |
   0 |_ESC_|__1__|__2__|__3__|__4__|__5__|              0 |__6__|__7__|__8__|__9__|__0__|__-__|
   1 |_TAB_|__q__|__w__|__e__|__r__|__t__|              1 |__y__|__u__|__i__|__o__|__p__|__[__|
   2 |_BSP_|__a__|__s__|__d__|__f__|__g__|              2 |__h__|__j__|__k__|__l__|__;__|__]__|
   3 |_LYR_|__z__|__x__|__c__|__v__|__b__|              3 |__n__|__m__|__,__|__.__|__/__|__\__|
   4 |_____|_____|_____|_CTL_|_SFT_|_SPC_|              4 |_ENT_|_CTL_|_LYR_|_____|_____|_____|

*/
use crate::enums::*;
use esp_idf_svc::hal::gpio::*;
use esp_idf_svc::hal::peripherals::Peripherals;
use std::collections::HashMap;

pub mod ble_keyboard;
pub mod enums;

pub mod delay {
    use embassy_time::{Duration, Instant};

    pub fn delay_us(us: u64) {
        let duration = Duration::from_micros(us);
        let end_time = Instant::now() + duration;

        while Instant::now() < end_time {}
    }
}

pub const ROWS: usize = 5;
pub const COLS: usize = 6;
pub const LAYER_KEY: (i8, i8) = (3, 0);

pub struct KeyMatrix<'a> {
    pub rows: [PinDriver<'a, AnyOutputPin, Output>; ROWS],
    pub cols: [PinDriver<'a, AnyInputPin, Input>; COLS],
}

pub enum Layer {
    Base,
    Upper,
}

pub struct KeyboardLeftSide<'a> {
    pub base_layer: HashMap<(i8, i8), HidMapings>,
    pub key_matrix: KeyMatrix<'a>,
    pub layer: Layer,
}

impl KeyboardLeftSide<'_> {
    pub fn new() -> KeyboardLeftSide<'static> {
        let peripherals = Peripherals::take().unwrap();

        KeyboardLeftSide {
            base_layer: HashMap::new(),
            key_matrix: KeyMatrix {
                rows: [
                    PinDriver::output(peripherals.pins.gpio0.downgrade_output()).unwrap(),
                    PinDriver::output(peripherals.pins.gpio1.downgrade_output()).unwrap(),
                    PinDriver::output(peripherals.pins.gpio12.downgrade_output()).unwrap(),
                    PinDriver::output(peripherals.pins.gpio18.downgrade_output()).unwrap(),
                    PinDriver::output(peripherals.pins.gpio19.downgrade_output()).unwrap(),
                ],
                cols: [
                    PinDriver::input(peripherals.pins.gpio2.downgrade_input()).unwrap(),
                    PinDriver::input(peripherals.pins.gpio3.downgrade_input()).unwrap(),
                    PinDriver::input(peripherals.pins.gpio10.downgrade_input()).unwrap(),
                    PinDriver::input(peripherals.pins.gpio6.downgrade_input()).unwrap(),
                    PinDriver::input(peripherals.pins.gpio7.downgrade_input()).unwrap(),
                    PinDriver::input(peripherals.pins.gpio4.downgrade_input()).unwrap(),
                ],
            },
            layer: Layer::Base,
        }
    }

    pub fn initialize_base_layer(&mut self) {
        self.base_layer.insert((0, 0), HidMapings::Escape); // ESC
        self.base_layer.insert((0, 1), HidMapings::Num1); // 1
        self.base_layer.insert((0, 2), HidMapings::Num2); // 2
        self.base_layer.insert((0, 3), HidMapings::Num3); // 3
        self.base_layer.insert((0, 4), HidMapings::Num4); // 4
        self.base_layer.insert((0, 5), HidMapings::Num5); // 5

        self.base_layer.insert((1, 0), HidMapings::Tab); // TAB
        self.base_layer.insert((1, 1), HidMapings::Q); // q
        self.base_layer.insert((1, 2), HidMapings::W); // w
        self.base_layer.insert((1, 3), HidMapings::E); // e
        self.base_layer.insert((1, 4), HidMapings::R); // r
        self.base_layer.insert((1, 5), HidMapings::T); // t

        self.base_layer.insert((2, 0), HidMapings::Capslock); // BACKSPACE
        self.base_layer.insert((2, 1), HidMapings::A); // a
        self.base_layer.insert((2, 2), HidMapings::S); // s
        self.base_layer.insert((2, 3), HidMapings::D); // d
        self.base_layer.insert((2, 4), HidMapings::F); // f
        self.base_layer.insert((2, 5), HidMapings::G); // g

        self.base_layer.insert((3, 0), HidMapings::None); // LAYER
        self.base_layer.insert((3, 1), HidMapings::Z); // z
        self.base_layer.insert((3, 2), HidMapings::X); // x
        self.base_layer.insert((3, 3), HidMapings::C); // c
        self.base_layer.insert((3, 4), HidMapings::V); // v
        self.base_layer.insert((3, 5), HidMapings::B); // b

        self.base_layer.insert((4, 0), HidMapings::None); //
        self.base_layer.insert((4, 1), HidMapings::None); //
        self.base_layer.insert((4, 2), HidMapings::None); //
        self.base_layer.insert((4, 3), HidMapings::Control); // CONTROL
        self.base_layer.insert((4, 4), HidMapings::Shift); // SHIFT
        self.base_layer.insert((4, 5), HidMapings::Space); // SPACE
    }
}
