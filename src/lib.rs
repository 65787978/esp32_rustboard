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
use embassy_time::{Duration, Instant};
use esp_idf_svc::hal::gpio::*;
use esp_idf_svc::hal::peripherals::Peripherals;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub mod ble_keyboard;
pub mod enums;

pub mod delay {
    use embassy_time::{Duration, Instant, Timer};

    pub async fn delay_ms(delay: u64) {
        let duration = Duration::from_millis(delay);
        Timer::after(duration).await;
    }

    pub async fn delay_us(delay: u64) {
        let duration = Duration::from_millis(delay);
        Timer::after(duration).await;
    }

    pub fn time_now() -> Instant {
        Instant::now()
    }
}

pub const ROWS: usize = 5;
pub const COLS: usize = 6;
pub const LAYER_KEY: (i8, i8) = (3, 0);
pub const DEBOUNCE_DELAY: Duration = Duration::from_millis(50);

pub struct PinMatrix<'a> {
    pub rows: [PinDriver<'a, AnyOutputPin, Output>; ROWS],
    pub cols: [PinDriver<'a, AnyInputPin, Input>; COLS],
}

impl PinMatrix<'_> {
    pub fn new() -> PinMatrix<'static> {
        let peripherals = Peripherals::take().unwrap();

        PinMatrix {
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
        }
    }

    pub async fn scan_grid(&mut self, keys_pressed: &Arc<Mutex<HashMap<(i8, i8), Instant>>>) -> ! {
        /* initialize counts */
        let mut row_count: i8 = 0;
        let mut col_count: i8 = 0;

        loop {
            /* check rows and cols */
            for row in self.rows.iter_mut() {
                /* set row to high */
                row.set_high().unwrap();

                /* delay so pin can propagate */
                delay::delay_us(10).await;

                /* check if a col is high */
                for col in self.cols.iter_mut() {
                    /* if a col is high */
                    if col.is_high() {
                        log::info!("ArcMutex not yet locked");
                        /* lock the hashmap */
                        match keys_pressed.try_lock() {
                            Ok(mut key_pressed_lock) => {
                                log::info!("ArcMutex Locked");

                                /* check if the key has been pressed already*/
                                if !key_pressed_lock.contains_key(&(row_count, col_count)) {
                                    /* store pressed keys */
                                    key_pressed_lock.insert((row_count, col_count), Instant::now());

                                    log::info!("Pressed keys stored!");
                                }
                            }
                            Err(_) => {}
                        }
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
            /* reset row count */
            row_count = 0;
        }
    }
}

pub enum Layer {
    Base,
    Upper,
}
pub struct Layers {
    pub base: HashMap<(i8, i8), HidMapings>,
    pub upper: HashMap<(i8, i8), HidMapings>,
    pub state: Layer,
}

impl Layers {
    pub fn new() -> Self {
        Layers {
            base: HashMap::new(),
            upper: HashMap::new(),
            state: Layer::Base,
        }
    }
    pub fn initialize_base_layer(&mut self) {
        self.base.insert((0, 0), HidMapings::Escape); // ESC
        self.base.insert((0, 1), HidMapings::Num1); // 1
        self.base.insert((0, 2), HidMapings::Num2); // 2
        self.base.insert((0, 3), HidMapings::Num3); // 3
        self.base.insert((0, 4), HidMapings::Num4); // 4
        self.base.insert((0, 5), HidMapings::Num5); // 5

        self.base.insert((1, 0), HidMapings::Tab); // TAB
        self.base.insert((1, 1), HidMapings::Q); // q
        self.base.insert((1, 2), HidMapings::W); // w
        self.base.insert((1, 3), HidMapings::E); // e
        self.base.insert((1, 4), HidMapings::R); // r
        self.base.insert((1, 5), HidMapings::T); // t

        self.base.insert((2, 0), HidMapings::Capslock); // BACKSPACE
        self.base.insert((2, 1), HidMapings::A); // a
        self.base.insert((2, 2), HidMapings::S); // s
        self.base.insert((2, 3), HidMapings::D); // d
        self.base.insert((2, 4), HidMapings::F); // f
        self.base.insert((2, 5), HidMapings::G); // g

        self.base.insert((3, 0), HidMapings::None); // LAYER
        self.base.insert((3, 1), HidMapings::Z); // z
        self.base.insert((3, 2), HidMapings::X); // x
        self.base.insert((3, 3), HidMapings::C); // c
        self.base.insert((3, 4), HidMapings::V); // v
        self.base.insert((3, 5), HidMapings::B); // b

        self.base.insert((4, 0), HidMapings::None); //
        self.base.insert((4, 1), HidMapings::None); //
        self.base.insert((4, 2), HidMapings::None); //
        self.base.insert((4, 3), HidMapings::Control); // CONTROL
        self.base.insert((4, 4), HidMapings::Shift); // SHIFT
        self.base.insert((4, 5), HidMapings::Space); // SPACE
    }
}
