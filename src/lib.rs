/*
*********************************************************************************************
BASE LAYER:

PINS|  2  |  3  |  10 |  6  |  7  |  4  |           PINS|  2  |  3  |  10 |  6  |  7  |  11 |
  0 |_ESC_|__1__|__2__|__3__|__4__|__5__|             0 |__6__|__7__|__8__|__9__|__0__|__-__|
  1 |_TAB_|__q__|__w__|__e__|__r__|__t__|             1 |__y__|__u__|__i__|__o__|__p__|__[__|
 12 |_CAP_|__a__|__s__|__d__|__f__|__g__|             12|__h__|__j__|__k__|__l__|__;__|__]__|
 18 |_____|__z__|__x__|__c__|__v__|__b__|             18|__n__|__m__|__,__|__.__|__/__|__\__|
 19 |_____|_____|_LYR_|_CTL_|_SFT_|_ENT_|             19|_SPC_|_BSP_|_CTL_|_LYR_|_____|_____|

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
use esp_idf_hal::gpio::*;
// use esp_idf_hal::peripherals::Peripherals;
use esp_idf_svc::hal::delay::FreeRtos;
use std::collections::HashMap;

pub const DELAY_DEFAULT: u32 = 20;
pub const DELAY_SAME_KEY: u32 = 60;

pub const PIN_INACTIVE: i32 = -1;

#[derive(Clone, Default, Debug)]
pub struct KeyboardLeftSide {
    pub base_layer: HashMap<(i32, i32), &'static str>,
    pub shift_layer: HashMap<(i32, i32), &'static str>,
    pub upper_layer: HashMap<(i32, i32), &'static str>,
}

impl KeyboardLeftSide {
    pub fn new() -> KeyboardLeftSide {
        KeyboardLeftSide {
            base_layer: HashMap::new(),
            shift_layer: HashMap::new(),
            upper_layer: HashMap::new(),
        }
    }

    pub fn initialize_layers(&mut self) {
        self.initialie_base_layer();
        self.initialie_shift_layer();
        self.initialie_upper_layer();
    }

    fn initialie_base_layer(&mut self) {
        self.base_layer.insert((0, 2), "ESC");
        self.base_layer.insert((0, 3), "1");
        self.base_layer.insert((0, 10), "2");
        self.base_layer.insert((0, 6), "3");
        self.base_layer.insert((0, 7), "4");
        self.base_layer.insert((0, 4), "5");

        self.base_layer.insert((1, 2), "TAB");
        self.base_layer.insert((1, 3), "q");
        self.base_layer.insert((1, 10), "w");
        self.base_layer.insert((1, 6), "e");
        self.base_layer.insert((1, 7), "r");
        self.base_layer.insert((1, 4), "t");

        self.base_layer.insert((12, 2), "CAP");
        self.base_layer.insert((12, 3), "a");
        self.base_layer.insert((12, 10), "s");
        self.base_layer.insert((12, 6), "d");
        self.base_layer.insert((12, 7), "f");
        self.base_layer.insert((12, 4), "g");

        self.base_layer.insert((18, 2), "");
        self.base_layer.insert((18, 3), "z");
        self.base_layer.insert((18, 10), "x");
        self.base_layer.insert((18, 6), "c");
        self.base_layer.insert((18, 7), "v");
        self.base_layer.insert((18, 4), "b");

        self.base_layer.insert((19, 2), "");
        self.base_layer.insert((19, 3), "");
        self.base_layer.insert((19, 10), "LYR");
        self.base_layer.insert((19, 6), "CTL");
        self.base_layer.insert((19, 7), "SFT");
        self.base_layer.insert((19, 4), "ENT");
    }

    fn initialie_shift_layer(&mut self) {
        self.shift_layer.insert((0, 2), "ESC");
        self.shift_layer.insert((0, 3), "!");
        self.shift_layer.insert((0, 10), "@");
        self.shift_layer.insert((0, 6), "#");
        self.shift_layer.insert((0, 7), "$");
        self.shift_layer.insert((0, 4), "%");

        self.shift_layer.insert((1, 2), "TAB");
        self.shift_layer.insert((1, 3), "Q");
        self.shift_layer.insert((1, 10), "W");
        self.shift_layer.insert((1, 6), "E");
        self.shift_layer.insert((1, 7), "R");
        self.shift_layer.insert((1, 4), "T");

        self.shift_layer.insert((12, 2), "CAP");
        self.shift_layer.insert((12, 3), "A");
        self.shift_layer.insert((12, 10), "S");
        self.shift_layer.insert((12, 6), "D");
        self.shift_layer.insert((12, 7), "F");
        self.shift_layer.insert((12, 4), "G");

        self.shift_layer.insert((18, 2), "");
        self.shift_layer.insert((18, 3), "Z");
        self.shift_layer.insert((18, 10), "X");
        self.shift_layer.insert((18, 6), "C");
        self.shift_layer.insert((18, 7), "V");
        self.shift_layer.insert((18, 4), "B");

        self.shift_layer.insert((19, 2), "");
        self.shift_layer.insert((19, 3), "");
        self.shift_layer.insert((19, 10), "LYR");
        self.shift_layer.insert((19, 6), "CTL");
        self.shift_layer.insert((19, 7), "SFT");
        self.shift_layer.insert((19, 4), "ENT");
    }

    fn initialie_upper_layer(&mut self) {
        self.upper_layer.insert((0, 2), "ESC");
        self.upper_layer.insert((0, 3), "-");
        self.upper_layer.insert((0, 10), "-");
        self.upper_layer.insert((0, 6), "-");
        self.upper_layer.insert((0, 7), "-");
        self.upper_layer.insert((0, 4), "-");

        self.upper_layer.insert((1, 2), "TAB");
        self.upper_layer.insert((1, 3), "-");
        self.upper_layer.insert((1, 10), "UP");
        self.upper_layer.insert((1, 6), "-");
        self.upper_layer.insert((1, 7), "-");
        self.upper_layer.insert((1, 4), "-");

        self.upper_layer.insert((12, 2), "CAP");
        self.upper_layer.insert((12, 3), "LFT");
        self.upper_layer.insert((12, 10), "DWN");
        self.upper_layer.insert((12, 6), "RGT");
        self.upper_layer.insert((12, 7), "-");
        self.upper_layer.insert((12, 4), "-");

        self.upper_layer.insert((18, 2), "-");
        self.upper_layer.insert((18, 3), "-");
        self.upper_layer.insert((18, 10), "-");
        self.upper_layer.insert((18, 6), "-");
        self.upper_layer.insert((18, 7), "-");
        self.upper_layer.insert((18, 4), "-");

        self.upper_layer.insert((19, 2), "-");
        self.upper_layer.insert((19, 3), "-");
        self.upper_layer.insert((19, 10), "LYR");
        self.upper_layer.insert((19, 6), "CTL");
        self.upper_layer.insert((19, 7), "SFT");
        self.upper_layer.insert((19, 4), "ENT");
    }
}

pub enum Layer {
    Base,
    Shift,
    Upper,
}

pub fn check_pins(
    pins_active: &mut (i32, i32),
    gpio2: &PinDriver<Gpio2, Input>,
    gpio3: &PinDriver<Gpio3, Input>,
    gpio10: &PinDriver<Gpio10, Input>,
    gpio6: &PinDriver<Gpio6, Input>,
    gpio7: &PinDriver<Gpio7, Input>,
    gpio4: &PinDriver<Gpio4, Input>,
) {
    while gpio2.is_high() {
        pins_active.1 = gpio2.pin();
        // log::info!("{}, {}", pins_active.0, pins_active.1);
        FreeRtos::delay_ms(DELAY_SAME_KEY);
        break;
    }
    while gpio3.is_high() {
        pins_active.1 = gpio3.pin();
        // log::info!("{}, {}", pins_active.0, pins_active.1);
        FreeRtos::delay_ms(DELAY_SAME_KEY);
        break;
    }
    while gpio10.is_high() {
        pins_active.1 = gpio10.pin();
        // log::info!("{}, {}", pins_active.0, pins_active.1);
        FreeRtos::delay_ms(DELAY_SAME_KEY);
        break;
    }
    while gpio6.is_high() {
        pins_active.1 = gpio6.pin();
        // log::info!("{}, {}", pins_active.0, pins_active.1);
        FreeRtos::delay_ms(DELAY_SAME_KEY);
        break;
    }
    while gpio7.is_high() {
        pins_active.1 = gpio7.pin();
        // log::info!("{}, {}", pins_active.0, pins_active.1);
        FreeRtos::delay_ms(DELAY_SAME_KEY);
        break;
    }
    while gpio4.is_high() {
        pins_active.1 = gpio4.pin();
        // log::info!("{}, {}", pins_active.0, pins_active.1);
        FreeRtos::delay_ms(DELAY_SAME_KEY);
        break;
    }
}

// #[derive(Clone, Copy, Debug)]
// enum Pins {
//     Gpio0 = 0,
//     Gpio1 = 1,
//     Gpio2 = 2,
//     Gpio3 = 3,
//     Gpio6 = 6,
//     Gpio7 = 7,
//     Gpio10 = 10,
//     Gpio12 = 12,
//     Gpio13 = 13,
//     Gpio18 = 18,
//     Gpio19 = 19,
// }

// impl Pins {
//     pub fn as_i32(&self) -> i32 {
//         self.clone() as i32
//     }
// }

// pub enum RowPins {
//     Row0,
//     Row1,
//     Row2,
//     Row3,
//     Row4,
// }

// impl RowPins {
//     pub fn is_high(&self) -> (bool, i32) {
//         let peripherals = Peripherals::take().expect("msg");

//         match self {
//             RowPins::Row0 => (
//                 PinDriver::input(peripherals.pins.gpio2)
//                     .expect("msg")
//                     .is_high(),
//                 Pins::Gpio2.as_i32(),
//             ),
//             RowPins::Row1 => (
//                 PinDriver::input(peripherals.pins.gpio3)
//                     .expect("msg")
//                     .is_high(),
//                 Pins::Gpio3.as_i32(),
//             ),
//             RowPins::Row2 => (
//                 PinDriver::input(peripherals.pins.gpio10)
//                     .expect("msg")
//                     .is_high(),
//                 Pins::Gpio10.as_i32(),
//             ),
//             RowPins::Row3 => (
//                 PinDriver::input(peripherals.pins.gpio6)
//                     .expect("msg")
//                     .is_high(),
//                 Pins::Gpio6.as_i32(),
//             ),
//             RowPins::Row4 => (
//                 PinDriver::input(peripherals.pins.gpio7)
//                     .expect("msg")
//                     .is_high(),
//                 Pins::Gpio7.as_i32(),
//             ),
//         }
//     }
// }

// pub enum ColPins {
//     Col0,
//     Col1,
//     Col2,
//     Col3,
//     Col4,
//     Col5,
// }

// impl ColPins {
//     pub fn is_high(&self) -> (bool, i32) {
//         let peripherals = Peripherals::take().expect("msg");

//         match self {
//             ColPins::Col0 => (
//                 PinDriver::input(peripherals.pins.gpio0)
//                     .expect("msg")
//                     .is_high(),
//                 Pins::Gpio0.as_i32(),
//             ),
//             ColPins::Col1 => (
//                 PinDriver::input(peripherals.pins.gpio1)
//                     .expect("msg")
//                     .is_high(),
//                 Pins::Gpio1.as_i32(),
//             ),
//             ColPins::Col2 => (
//                 PinDriver::input(peripherals.pins.gpio12)
//                     .expect("msg")
//                     .is_high(),
//                 Pins::Gpio12.as_i32(),
//             ),
//             ColPins::Col3 => (
//                 PinDriver::input(peripherals.pins.gpio18)
//                     .expect("msg")
//                     .is_high(),
//                 Pins::Gpio18.as_i32(),
//             ),
//             ColPins::Col4 => (
//                 PinDriver::input(peripherals.pins.gpio19)
//                     .expect("msg")
//                     .is_high(),
//                 Pins::Gpio19.as_i32(),
//             ),
//             ColPins::Col5 => (
//                 PinDriver::input(peripherals.pins.gpio13)
//                     .expect("msg")
//                     .is_high(),
//                 Pins::Gpio13.as_i32(),
//             ),
//         }
//     }
// }
