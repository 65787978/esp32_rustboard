use crate::debounce::KEY_PRESSED;
use crate::delay::*;
use crate::{config::config::*, debounce::Debounce};
use embassy_time::Instant;
use esp_idf_svc::hal::gpio::*;
use esp_idf_svc::hal::peripherals::Peripherals;
use esp_idf_sys::{
    self as _, gpio_int_type_t_GPIO_INTR_HIGH_LEVEL, gpio_num_t_GPIO_NUM_10,
    gpio_num_t_GPIO_NUM_20, gpio_num_t_GPIO_NUM_6, gpio_num_t_GPIO_NUM_7,
};
use heapless::FnvIndexMap;
use spin::Mutex;

#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug)]
pub struct Key {
    pub row: i8,
    pub col: i8,
}

impl Key {
    fn new(row: i8, col: i8) -> Key {
        Key { row, col }
    }
}
pub struct PinMatrix<'a> {
    pub rows: [PinDriver<'a, AnyOutputPin, Output>; ROWS],
    pub cols: [PinDriver<'a, AnyIOPin, Input>; COLS],
    pub enter_sleep_delay: Instant,
    pub sleep_delay_key_pressed: bool,
}

impl PinMatrix<'_> {
    pub fn new() -> PinMatrix<'static> {
        let peripherals = Peripherals::take().expect("Not able to init peripherals.");

        PinMatrix {
            rows: [
                PinDriver::output(peripherals.pins.gpio0.downgrade_output())
                    .expect("Not able to set port as output."),
                PinDriver::output(peripherals.pins.gpio1.downgrade_output())
                    .expect("Not able to set port as output."),
                PinDriver::output(peripherals.pins.gpio2.downgrade_output())
                    .expect("Not able to set port as output."),
                PinDriver::output(peripherals.pins.gpio3.downgrade_output())
                    .expect("Not able to set port as output."),
            ],
            cols: [
                PinDriver::input(peripherals.pins.gpio21.downgrade())
                    .expect("Not able to set port as input."),
                PinDriver::input(peripherals.pins.gpio20.downgrade())
                    .expect("Not able to set port as input."),
                PinDriver::input(peripherals.pins.gpio10.downgrade())
                    .expect("Not able to set port as input."),
                PinDriver::input(peripherals.pins.gpio7.downgrade())
                    .expect("Not able to set port as input."),
                PinDriver::input(peripherals.pins.gpio6.downgrade())
                    .expect("Not able to set port as input."),
                PinDriver::input(peripherals.pins.gpio5.downgrade())
                    .expect("Not able to set port as input."),
            ],
            enter_sleep_delay: Instant::now() + SLEEP_DELAY_INIT,
            sleep_delay_key_pressed: false,
        }
    }

    fn set_cols_interrupt(&mut self) {
        for col in self.cols.iter_mut() {
            col.set_pull(Pull::Down).unwrap();
            col.set_interrupt_type(InterruptType::AnyEdge)
                .expect("Not able to set interrupt type.");
        }
    }

    fn set_enable_interrupts(&mut self) {
        for col in self.cols.iter_mut() {
            col.enable_interrupt()
                .expect("Not able to enable interrput.")
        }
    }

    fn reset_sleep_delay(&mut self) {
        self.enter_sleep_delay = Instant::now() + SLEEP_DELAY;
    }

    fn enter_sleep_mode(&mut self) {
        /* enable interrupts */
        self.set_enable_interrupts();

        /* set the home row to high */
        self.rows[1].set_high().unwrap();

        /* enter sleep mode */
        unsafe {
            /* set gpios that can wake up the chip */
            esp_idf_sys::gpio_wakeup_enable(
                gpio_num_t_GPIO_NUM_20,
                gpio_int_type_t_GPIO_INTR_HIGH_LEVEL,
            );
            esp_idf_sys::gpio_wakeup_enable(
                gpio_num_t_GPIO_NUM_10,
                gpio_int_type_t_GPIO_INTR_HIGH_LEVEL,
            );
            esp_idf_sys::gpio_wakeup_enable(
                gpio_num_t_GPIO_NUM_7,
                gpio_int_type_t_GPIO_INTR_HIGH_LEVEL,
            );
            esp_idf_sys::gpio_wakeup_enable(
                gpio_num_t_GPIO_NUM_6,
                gpio_int_type_t_GPIO_INTR_HIGH_LEVEL,
            );

            esp_idf_sys::esp_sleep_enable_gpio_switch(false);

            esp_idf_sys::esp_sleep_enable_gpio_wakeup();

            log::info!("Entering sleep...");

            /* enter sleep */
            esp_idf_sys::esp_light_sleep_start();

            log::info!("Woke up...");

            /* reset sleep delay */
            self.reset_sleep_delay();

            /* restart the cpu, so we have faster ble connection after sleep */
            esp_idf_sys::esp_restart();
        }
    }
}

pub async fn scan_grid(
    keys_pressed: &Mutex<FnvIndexMap<Key, Debounce, PRESSED_KEYS_INDEXMAP_SIZE>>,
) -> ! {
    /* construct the matrix */
    let mut matrix = PinMatrix::new();

    /* initialize interrupt */
    matrix.set_cols_interrupt();

    /* initialize counts */
    let mut count = Key::new(0, 0);

    loop {
        if Instant::now() >= matrix.enter_sleep_delay {
            matrix.enter_sleep_mode();
        } else {
            /* check rows and cols */
            for row in matrix.rows.iter_mut() {
                /* set row to high */
                row.set_high().unwrap();

                /* delay so pin can propagate */
                delay_us(50).await;

                /* check if a col is high */
                for col in matrix.cols.iter() {
                    /* check if a col is set to high (key pressed) */
                    if col.is_high() {
                        /* lock the hashmap */
                        if let Some(mut keys_pressed) = keys_pressed.try_lock() {
                            /* Inserts a key-value pair into the map.
                             * If an equivalent key already exists in the map: the key remains and retains in its place in the order, its corresponding value is updated with value and the older value is returned inside Some(_).
                             * If no equivalent key existed in the map: the new key-value pair is inserted, last in order, and None is returned.
                             */
                            keys_pressed
                                .insert(
                                    Key {
                                        row: count.row,
                                        col: count.col,
                                    },
                                    Debounce {
                                        key_pressed_time: Instant::now(),
                                        key_state: KEY_PRESSED,
                                    },
                                )
                                .expect("Error setting new key in the hashmap");

                            log::info!("Pressed keys stored! X:{}, Y:{}", count.row, count.col);

                            /* reset sleep delay if a key is pressed */
                            matrix.sleep_delay_key_pressed = true;
                        }
                    }
                    /* increment col */
                    count.col += 1;
                }
                /* set row to low */
                row.set_low().unwrap();

                /* increment row */
                count.row += 1;

                /* reset col count */
                count.col = 0;
            }

            /* reset row count */
            count.row = 0;

            /* if a key has been pressed */
            if matrix.sleep_delay_key_pressed {
                /* reset key_pressed */
                matrix.sleep_delay_key_pressed = false;

                /* reset sleep delay */
                matrix.reset_sleep_delay();
            }
        }
    }
}
