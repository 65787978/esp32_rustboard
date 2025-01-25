use crate::ble::BleStatus;
use crate::debounce::KEY_PRESSED;
use crate::delay::*;
use crate::{config::config::*, debounce::Debounce};
use embassy_time::Instant;
use esp_idf_svc::hal::gpio::*;
use esp_idf_svc::hal::peripherals::Peripherals;

#[cfg(feature = "sleep-mode")]
use esp_idf_sys::{
    self as _, esp_bt_controller_disable, gpio_int_type_t_GPIO_INTR_HIGH_LEVEL,
    gpio_num_t_GPIO_NUM_10, gpio_num_t_GPIO_NUM_20, gpio_num_t_GPIO_NUM_6, gpio_num_t_GPIO_NUM_7,
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
    pub rows: [PinDriver<'a, AnyIOPin, Output>; ROWS],
    pub cols: [PinDriver<'a, AnyIOPin, Input>; COLS],
    #[cfg(feature = "sleep-mode")]
    pub enter_sleep_delay: Instant,
}

impl PinMatrix<'_> {
    pub fn new() -> PinMatrix<'static> {
        let peripherals = Peripherals::take().expect("Not able to init peripherals.");

        PinMatrix {
            rows: [
                PinDriver::output(peripherals.pins.gpio0.downgrade())
                    .expect("Not able to set port as output."),
                PinDriver::output(peripherals.pins.gpio1.downgrade())
                    .expect("Not able to set port as output."),
                PinDriver::output(peripherals.pins.gpio2.downgrade())
                    .expect("Not able to set port as output."),
                PinDriver::output(peripherals.pins.gpio3.downgrade())
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
            #[cfg(feature = "sleep-mode")]
            enter_sleep_delay: Instant::now() + SLEEP_DELAY_INIT,
        }
    }

    fn set_cols_interrupt(&mut self) {
        for col in self.cols.iter_mut() {
            col.set_pull(Pull::Down).unwrap();
            col.set_interrupt_type(InterruptType::AnyEdge)
                .expect("Not able to set interrupt type.");
        }
    }

    #[cfg(feature = "sleep-mode")]
    fn set_light_sleep_enable_interrupts(&mut self) {
        for col in self.cols.iter_mut() {
            col.enable_interrupt()
                .expect("Not able to enable interrput.")
        }
    }

    #[cfg(feature = "sleep-mode")]
    fn set_light_sleep_gpio_wakeup_enable(&mut self) {
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
        }
    }

    #[cfg(feature = "sleep-mode")]
    fn enter_light_sleep_mode(&mut self) {
        /* enable interrupts */
        self.set_light_sleep_enable_interrupts();

        /* set the home row to high */
        self.rows[1].set_high().unwrap();

        /* set gpio wakeup enable interrup */
        self.set_light_sleep_gpio_wakeup_enable();

        /* enter sleep mode */
        unsafe {
            /* disable bt before entering sleep */
            esp_bt_controller_disable();

            esp_idf_sys::esp_sleep_enable_gpio_switch(false);

            esp_idf_sys::esp_sleep_enable_gpio_wakeup();

            #[cfg(feature = "debug")]
            log::info!("Entering sleep...");

            /* enter sleep */
            esp_idf_sys::esp_light_sleep_start();

            #[cfg(feature = "debug")]
            log::info!("Woke up...");

            /* restart the cpu, so we have faster ble connection after sleep */
            esp_idf_sys::esp_restart();
        }
    }
}

fn store_key(
    keys_pressed: &Mutex<FnvIndexMap<Key, Debounce, PRESSED_KEYS_INDEXMAP_SIZE>>,
    key: &Key,
) -> Option<()> {
    /* lock the hashmap */
    if let Some(mut keys_pressed) = keys_pressed.try_lock() {
        /* Inserts a key-value pair into the map.
         * If an equivalent key already exists in the map: the key remains and retains in its place in the order, its corresponding value is updated with value and the older value is returned inside Some(_).
         * If no equivalent key existed in the map: the new key-value pair is inserted, last in order, and None is returned.
         */
        keys_pressed
            .insert(
                Key {
                    row: key.row,
                    col: key.col,
                },
                Debounce {
                    key_pressed_time: Instant::now(),
                    key_state: KEY_PRESSED,
                },
            )
            .expect("Error setting new key in the hashmap");

        #[cfg(feature = "debug")]
        log::info!("Pressed keys stored! X:{}, Y:{}", key.row, key.col);

        /* return true to reset the sleep delay */
        Some(())
    } else {
        /* else return false */
        None
    }
}

pub async fn scan_grid(
    keys_pressed: &Mutex<FnvIndexMap<Key, Debounce, PRESSED_KEYS_INDEXMAP_SIZE>>,
    ble_status: &Mutex<BleStatus>,
) -> ! {
    /* construct the matrix */
    let mut matrix = PinMatrix::new();

    /* initialize interrupt */
    matrix.set_cols_interrupt();

    /* initialize counts */
    let mut count = Key::new(0, 0);

    /* local ble status variable */
    let mut ble_status_local: BleStatus = BleStatus::NotConnected;

    loop {
        #[cfg(feature = "sleep-mode")]
        if Instant::now() >= matrix.enter_sleep_delay {
            matrix.enter_light_sleep_mode();
        }

        /* check and store the ble status, then release the lock */
        if let Some(ble_status) = ble_status.try_lock() {
            ble_status_local = *ble_status;
        }

        /* if a connection is established, run the key matrix */
        match ble_status_local {
            BleStatus::Connected => {
                /* check rows and cols */
                for row in matrix.rows.iter_mut() {
                    /* set row to high */
                    row.set_high().unwrap();

                    /* delay so pin can propagate */
                    delay_us(100).await;

                    /* check if a col is high */
                    for col in matrix.cols.iter() {
                        /* check if a col is set to high (key pressed) */
                        if col.is_high() {
                            /* store the key */
                            #[cfg(feature = "sleep-mode")]
                            match store_key(keys_pressed, &count) {
                                Some(()) => {
                                    matrix.enter_sleep_delay = Instant::now() + SLEEP_DELAY;
                                }
                                None => { /* do nothing */ }
                            }

                            #[cfg(not(feature = "sleep-mode"))]
                            store_key(keys_pressed, &count).unwrap();
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
            }
            BleStatus::NotConnected => {
                /* wait till there is a connection */
                /* sleep for 100ms */
                delay_ms(100).await;
            }
        }
    }
}
