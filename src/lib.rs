/*
*********************************************************************************************
BASE LAYER:

X \ Y|  0  |  1  |  2  |  3  |  4  |  5  |           X \ Y|  0  |  1  |  2  |  3  |  4  |  5  |
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
use esp_idf_sys::{
    self as _, gpio_int_type_t_GPIO_INTR_LOW_LEVEL, gpio_num_t_GPIO_NUM_10, gpio_num_t_GPIO_NUM_2,
    gpio_num_t_GPIO_NUM_3, gpio_num_t_GPIO_NUM_4, gpio_num_t_GPIO_NUM_6, gpio_num_t_GPIO_NUM_7,
};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub mod ble_keyboard;
pub mod enums;

pub mod delay {
    use embassy_time::{Duration, Timer};

    pub async fn delay_ms(delay: u64) {
        let duration = Duration::from_millis(delay);
        Timer::after(duration).await;
    }

    pub async fn delay_us(delay: u64) {
        let duration = Duration::from_millis(delay);
        Timer::after(duration).await;
    }
}

pub const ROWS: usize = 5;
pub const COLS: usize = 6;
pub const LAYER_KEY: (i8, i8) = (3, 0);
pub const DEBOUNCE_DELAY: Duration = Duration::from_millis(50);
pub const SLEEP_DELAY: Duration = Duration::from_millis(15000);
pub const SLEEP_DELAY_INIT: Duration = Duration::from_millis(30000);

pub struct PinMatrix<'a> {
    pub rows: [PinDriver<'a, AnyOutputPin, Output>; ROWS],
    pub cols: [PinDriver<'a, AnyIOPin, Input>; COLS],
    enter_sleep_delay: Instant,
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
                PinDriver::input(peripherals.pins.gpio2.downgrade()).unwrap(),
                PinDriver::input(peripherals.pins.gpio3.downgrade()).unwrap(),
                PinDriver::input(peripherals.pins.gpio10.downgrade()).unwrap(),
                PinDriver::input(peripherals.pins.gpio6.downgrade()).unwrap(),
                PinDriver::input(peripherals.pins.gpio7.downgrade()).unwrap(),
                PinDriver::input(peripherals.pins.gpio4.downgrade()).unwrap(),
            ],
            enter_sleep_delay: Instant::now() + SLEEP_DELAY_INIT,
        }
    }

    fn set_cols_interrupt(&mut self) {
        for col in self.cols.iter_mut() {
            col.set_pull(Pull::Down).unwrap();
            col.set_interrupt_type(InterruptType::AnyEdge).unwrap();
        }
    }

    fn set_enable_interrupts(&mut self) {
        for col in self.cols.iter_mut() {
            col.enable_interrupt().unwrap();
        }
    }

    fn reset_sleep_delay(&mut self) {
        self.enter_sleep_delay = Instant::now() + SLEEP_DELAY;
    }

    pub async fn scan_grid(&mut self, keys_pressed: &Arc<Mutex<HashMap<(i8, i8), Instant>>>) -> ! {
        /* initialize interrupt */
        self.set_cols_interrupt();

        /* initialize counts */
        let mut row_count: i8 = 0;
        let mut col_count: i8 = 0;

        /* initialize key_pressed */
        let mut key_pressed = false;

        loop {
            if Instant::now() >= self.enter_sleep_delay {
                /* enable interrupts */
                self.set_enable_interrupts();

                /* set the home row to high */
                self.rows[2].set_high().unwrap();

                /* enter sleep mode */
                unsafe {
                    esp_idf_sys::gpio_wakeup_enable(
                        gpio_num_t_GPIO_NUM_2,
                        gpio_int_type_t_GPIO_INTR_LOW_LEVEL,
                    );
                    esp_idf_sys::gpio_wakeup_enable(
                        gpio_num_t_GPIO_NUM_3,
                        gpio_int_type_t_GPIO_INTR_LOW_LEVEL,
                    );
                    esp_idf_sys::gpio_wakeup_enable(
                        gpio_num_t_GPIO_NUM_10,
                        gpio_int_type_t_GPIO_INTR_LOW_LEVEL,
                    );
                    esp_idf_sys::gpio_wakeup_enable(
                        gpio_num_t_GPIO_NUM_6,
                        gpio_int_type_t_GPIO_INTR_LOW_LEVEL,
                    );
                    esp_idf_sys::gpio_wakeup_enable(
                        gpio_num_t_GPIO_NUM_7,
                        gpio_int_type_t_GPIO_INTR_LOW_LEVEL,
                    );
                    esp_idf_sys::gpio_wakeup_enable(
                        gpio_num_t_GPIO_NUM_4,
                        gpio_int_type_t_GPIO_INTR_LOW_LEVEL,
                    );

                    esp_idf_sys::esp_sleep_enable_gpio_wakeup();
                    esp_idf_sys::esp_light_sleep_start();

                    /* reset sleep delay */
                    self.reset_sleep_delay();
                };
            } else {
                /* check rows and cols */
                for row in self.rows.iter_mut() {
                    /* set row to high */
                    row.set_high().unwrap();

                    /* delay so pin can propagate */
                    delay::delay_us(50).await;

                    /* check if a col is high */
                    for col in self.cols.iter() {
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
                                        key_pressed_lock
                                            .insert((row_count, col_count), Instant::now());

                                        log::info!("Pressed keys stored!");
                                    }
                                }
                                Err(_) => {}
                            }

                            /* reset sleep delay if a key is pressed */
                            key_pressed = true;
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

                /* if a key has been pressed */
                if key_pressed {
                    /* reset key_pressed */
                    key_pressed = false;

                    /* reset sleep delay */
                    self.reset_sleep_delay();
                }
            }
        }
    }
}

pub enum Layer {
    Base,
    Upper,
}
pub struct Layers {
    pub base: HashMap<(i8, i8), HidKeys>,
    pub upper: HashMap<(i8, i8), HidKeys>,
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
        self.base.insert((0, 0), HidKeys::Escape); // ESC
        self.base.insert((0, 1), HidKeys::Num1); // 1
        self.base.insert((0, 2), HidKeys::Num2); // 2
        self.base.insert((0, 3), HidKeys::Num3); // 3
        self.base.insert((0, 4), HidKeys::Num4); // 4
        self.base.insert((0, 5), HidKeys::Num5); // 5

        self.base.insert((1, 0), HidKeys::Tab); // TAB
        self.base.insert((1, 1), HidKeys::Q); // q
        self.base.insert((1, 2), HidKeys::W); // w
        self.base.insert((1, 3), HidKeys::E); // e
        self.base.insert((1, 4), HidKeys::R); // r
        self.base.insert((1, 5), HidKeys::T); // t

        self.base.insert((2, 0), HidKeys::Capslock); // BACKSPACE
        self.base.insert((2, 1), HidKeys::A); // a
        self.base.insert((2, 2), HidKeys::S); // s
        self.base.insert((2, 3), HidKeys::D); // d
        self.base.insert((2, 4), HidKeys::F); // f
        self.base.insert((2, 5), HidKeys::G); // g

        self.base.insert((3, 0), HidKeys::None); // LAYER
        self.base.insert((3, 1), HidKeys::Z); // z
        self.base.insert((3, 2), HidKeys::X); // x
        self.base.insert((3, 3), HidKeys::C); // c
        self.base.insert((3, 4), HidKeys::V); // v
        self.base.insert((3, 5), HidKeys::B); // b

        self.base.insert((4, 0), HidKeys::None); //
        self.base.insert((4, 1), HidKeys::None); //
        self.base.insert((4, 2), HidKeys::None); //
        self.base.insert((4, 3), HidKeys::Control); // CONTROL
        self.base.insert((4, 4), HidKeys::Shift); // SHIFT
        self.base.insert((4, 5), HidKeys::Space); // SPACE
    }
}
