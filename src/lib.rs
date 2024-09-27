/*
*********************************************************************************************
BASE LAYER:

PINS|  2  |  3  |  10 |  6  |  7  |  4  |           PINS|  2  |  3  |  10 |  6  |  7  |  11 |
  0 |_ESC_|__1__|__2__|__3__|__4__|__5__|             0 |__6__|__7__|__8__|__9__|__0__|__-__|
  1 |_TAB_|__q__|__w__|__e__|__r__|__t__|             1 |__y__|__u__|__i__|__o__|__p__|__[__|
 12 |_BSP_|__a__|__s__|__d__|__f__|__g__|             12|__h__|__j__|__k__|__l__|__;__|__]__|
 18 |_LYR_|__z__|__x__|__c__|__v__|__b__|             18|__n__|__m__|__,__|__.__|__/__|__\__|
 19 |_____|_____|_____|_CTL_|_SFT_|_SPC_|             19|_ENT_|_CTL_|_LYR_|_____|_____|_____|

*********************************************************************************************
SHIFT LAYER:

PINS|  2  |  3  |  10 |  6  |  7  |  4  |           PINS|  2  |  3  |  10 |  6  |  7  |  11 |
  0 |_ESC_|__!__|__@__|__#__|__$__|__%__|             0 |__^__|__&__|__*__|__(__|__)__|_ _ _|
  1 |_TAB_|__Q__|__W__|__E__|__R__|__T__|             1 |__Y__|__U__|__I__|__O__|__P__|__{__|
 12 |_CAP_|__A__|__S__|__D__|__F__|__G__|             12|__H__|__J__|__K__|__L__|__:__|__}__|
 18 |_____|__Z__|__X__|__C__|__V__|__B__|             18|__N__|__M__|__<__|__>__|__?__|__|__|
 19 |_____|_____|_LYR_|_CTL_|_SFT_|_ENT_|             19|_SPC_|_BSP_|_CTL_|_LYR_|_____|_____|

 *********************************************************************************************
UPPER LAYER:

PINS|  2  |  3  |  10 |  6  |  7  |  4  |           PINS|  2  |  3  |  10 |  6  |  7  |  11 |
  0 |_ESC_|_____|_____|_____|_____|_____|             0 |_____|_____|_____|_____|_____|_____|
  1 |_TAB_|_____|__UP_|_____|_____|_____|             1 |_____|_____|_____|_____|_____|_____|
 12 |_CAP_|_LFT_|_DWN_|_RGT_|_____|_____|             12|_____|_____|_____|_____|_____|_____|
 18 |_____|_____|_____|_____|_____|_____|             18|_____|_____|_____|_____|_____|_____|
 19 |_____|_____|_LYR_|_CTL_|_SFT_|_ENT_|             19|_SPC_|_BSP_|_CTL_|_LYR_|_____|_____|

*/
use crate::enums::*;
use esp_idf_svc::hal::gpio::*;
use esp_idf_svc::hal::peripherals::Peripherals;
use std::collections::HashMap;

pub mod ble_keyboard;
pub mod enums;

pub const ROWS: usize = 5;
pub const COLS: usize = 6;

pub struct KeyMatrix<'a> {
    pub rows: [PinDriver<'a, AnyOutputPin, Output>; ROWS],
    pub cols: [PinDriver<'a, AnyInputPin, Input>; COLS],
}

pub struct Layers {
    pub base_layer: HashMap<(i32, i32), HidMapings>,
}

pub struct KeyboardSide<'a> {
    pub layer: Layers,
    pub key_matrix: KeyMatrix<'a>,
}

impl KeyboardSide<'_> {
    pub fn new() -> KeyboardSide<'static> {
        let peripherals = Peripherals::take().unwrap();

        KeyboardSide {
            layer: Layers {
                base_layer: HashMap::new(),
            },
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
        }
    }

    pub fn initialize_base_layer(&mut self) {
        self.base_layer.insert((0, 2), HidMapings::Escape); // ESC
        self.base_layer.insert((0, 3), HidMapings::Num1); // 1
        self.base_layer.insert((0, 10), HidMapings::Num2); // 2
        self.base_layer.insert((0, 6), HidMapings::Num3); // 3
        self.base_layer.insert((0, 7), HidMapings::Num4); // 4
        self.base_layer.insert((0, 4), HidMapings::Num5); // 5

        self.base_layer.insert((1, 2), HidMapings::Tab); // TAB
        self.base_layer.insert((1, 3), HidMapings::Q); // q
        self.base_layer.insert((1, 10), HidMapings::W); // w
        self.base_layer.insert((1, 6), HidMapings::E); // e
        self.base_layer.insert((1, 7), HidMapings::R); // r
        self.base_layer.insert((1, 4), HidMapings::T); // t

        self.base_layer.insert((12, 2), HidMapings::Capslock); // BACKSPACE
        self.base_layer.insert((12, 3), HidMapings::A); // a
        self.base_layer.insert((12, 10), HidMapings::S); // s
        self.base_layer.insert((12, 6), HidMapings::D); // d
        self.base_layer.insert((12, 7), HidMapings::F); // f
        self.base_layer.insert((12, 4), HidMapings::G); // g

        self.base_layer.insert((18, 2), HidMapings::None); // LAYER
        self.base_layer.insert((18, 3), HidMapings::Z); // z
        self.base_layer.insert((18, 10), HidMapings::X); // x
        self.base_layer.insert((18, 6), HidMapings::C); // c
        self.base_layer.insert((18, 7), HidMapings::V); // v
        self.base_layer.insert((18, 4), HidMapings::B); // b

        self.base_layer.insert((19, 2), HidMapings::None); //
        self.base_layer.insert((19, 3), HidMapings::None); //
        self.base_layer.insert((19, 10), HidMapings::None); //
        self.base_layer.insert((19, 6), HidMapings::Control); // CONTROL
        self.base_layer.insert((19, 7), HidMapings::Shift); // SHIFT
        self.base_layer.insert((19, 4), HidMapings::Space); // SPACE
    }
}
