// originally: https://github.com/T-vK/ESP32-BLE-Keyboard
#![allow(dead_code)]

use crate::delay::{delay_ms, delay_us};
use crate::layers::Layers;

use crate::DEBOUNCE_DELAY;
use crate::KEYBOARD_LEFT_SIDE;
use embassy_time::Instant;
use esp32_nimble::{
    enums::*, hid::*, utilities::mutex::Mutex, BLEAdvertisementData, BLECharacteristic, BLEDevice,
    BLEHIDDevice, BLEServer,
};
use esp_idf_sys::{
    esp_ble_power_type_t_ESP_BLE_PWR_TYPE_CONN_HDL0, esp_power_level_t_ESP_PWR_LVL_N24,
};
use std::collections::HashMap;
use std::sync::{Arc, Mutex as stdMutex};
const KEYBOARD_ID: u8 = 0x01;
const MEDIA_KEYS_ID: u8 = 0x02;

const HID_REPORT_DISCRIPTOR: &[u8] = hid!(
    (USAGE_PAGE, 0x01), // USAGE_PAGE (Generic Desktop Ctrls)
    (USAGE, 0x06),      // USAGE (Keyboard)
    (COLLECTION, 0x01), // COLLECTION (Application)
    // ------------------------------------------------- Keyboard
    (REPORT_ID, KEYBOARD_ID), //   REPORT_ID (1)
    (USAGE_PAGE, 0x07),       //   USAGE_PAGE (Kbrd/Keypad)
    (USAGE_MINIMUM, 0xE0),    //   USAGE_MINIMUM (0xE0)
    (USAGE_MAXIMUM, 0xE7),    //   USAGE_MAXIMUM (0xE7)
    (LOGICAL_MINIMUM, 0x00),  //   LOGICAL_MINIMUM (0)
    (LOGICAL_MAXIMUM, 0x01),  //   Logical Maximum (1)
    (REPORT_SIZE, 0x01),      //   REPORT_SIZE (1)
    (REPORT_COUNT, 0x08),     //   REPORT_COUNT (8)
    (HIDINPUT, 0x02), //   INPUT (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position)
    (REPORT_COUNT, 0x01), //   REPORT_COUNT (1) ; 1 byte (Reserved)
    (REPORT_SIZE, 0x08), //   REPORT_SIZE (8)
    (HIDINPUT, 0x01), //   INPUT (Const,Array,Abs,No Wrap,Linear,Preferred State,No Null Position)
    (REPORT_COUNT, 0x05), //   REPORT_COUNT (5) ; 5 bits (Num lock, Caps lock, Scroll lock, Compose, Kana)
    (REPORT_SIZE, 0x01),  //   REPORT_SIZE (1)
    (USAGE_PAGE, 0x08),   //   USAGE_PAGE (LEDs)
    (USAGE_MINIMUM, 0x01), //   USAGE_MINIMUM (0x01) ; Num Lock
    (USAGE_MAXIMUM, 0x05), //   USAGE_MAXIMUM (0x05) ; Kana
    (HIDOUTPUT, 0x02), //   OUTPUT (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
    (REPORT_COUNT, 0x01), //   REPORT_COUNT (1) ; 3 bits (Padding)
    (REPORT_SIZE, 0x03), //   REPORT_SIZE (3)
    (HIDOUTPUT, 0x01), //   OUTPUT (Const,Array,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
    (REPORT_COUNT, 0x06), //   REPORT_COUNT (6) ; 6 bytes (Keys)
    (REPORT_SIZE, 0x08), //   REPORT_SIZE(8)
    (LOGICAL_MINIMUM, 0x00), //   LOGICAL_MINIMUM(0)
    (LOGICAL_MAXIMUM, 0x65), //   LOGICAL_MAXIMUM(0x65) ; 101 keys
    (USAGE_PAGE, 0x07), //   USAGE_PAGE (Kbrd/Keypad)
    (USAGE_MINIMUM, 0x00), //   USAGE_MINIMUM (0)
    (USAGE_MAXIMUM, 0x65), //   USAGE_MAXIMUM (0x65)
    (HIDINPUT, 0x00),  //   INPUT (Data,Array,Abs,No Wrap,Linear,Preferred State,No Null Position)
    (END_COLLECTION),  // END_COLLECTION
    // ------------------------------------------------- Media Keys
    (USAGE_PAGE, 0x0C),         // USAGE_PAGE (Consumer)
    (USAGE, 0x01),              // USAGE (Consumer Control)
    (COLLECTION, 0x01),         // COLLECTION (Application)
    (REPORT_ID, MEDIA_KEYS_ID), //   REPORT_ID (3)
    (USAGE_PAGE, 0x0C),         //   USAGE_PAGE (Consumer)
    (LOGICAL_MINIMUM, 0x00),    //   LOGICAL_MINIMUM (0)
    (LOGICAL_MAXIMUM, 0x01),    //   LOGICAL_MAXIMUM (1)
    (REPORT_SIZE, 0x01),        //   REPORT_SIZE (1)
    (REPORT_COUNT, 0x10),       //   REPORT_COUNT (16)
    (USAGE, 0xB5),              //   USAGE (Scan Next Track)     ; bit 0: 1
    (USAGE, 0xB6),              //   USAGE (Scan Previous Track) ; bit 1: 2
    (USAGE, 0xB7),              //   USAGE (Stop)                ; bit 2: 4
    (USAGE, 0xCD),              //   USAGE (Play/Pause)          ; bit 3: 8
    (USAGE, 0xE2),              //   USAGE (Mute)                ; bit 4: 16
    (USAGE, 0xE9),              //   USAGE (Volume Increment)    ; bit 5: 32
    (USAGE, 0xEA),              //   USAGE (Volume Decrement)    ; bit 6: 64
    (USAGE, 0x23, 0x02),        //   Usage (WWW Home)            ; bit 7: 128
    (USAGE, 0x94, 0x01),        //   Usage (My Computer) ; bit 0: 1
    (USAGE, 0x92, 0x01),        //   Usage (Calculator)  ; bit 1: 2
    (USAGE, 0x2A, 0x02),        //   Usage (WWW fav)     ; bit 2: 4
    (USAGE, 0x21, 0x02),        //   Usage (WWW search)  ; bit 3: 8
    (USAGE, 0x26, 0x02),        //   Usage (WWW stop)    ; bit 4: 16
    (USAGE, 0x24, 0x02),        //   Usage (WWW back)    ; bit 5: 32
    (USAGE, 0x83, 0x01),        //   Usage (Media sel)   ; bit 6: 64
    (USAGE, 0x8A, 0x01),        //   Usage (Mail)        ; bit 7: 128
    (HIDINPUT, 0x02), // INPUT (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position)
    (END_COLLECTION), // END_COLLECTION
);

pub const SHIFT: u8 = 0x02;
pub const ASCII_MAP: &[u8] = &[];

#[repr(packed)]
struct KeyReport {
    modifiers: u8,
    reserved: u8,
    keys: [u8; 6],
}

pub struct BleKeyboard {
    server: &'static mut BLEServer,
    input_keyboard: Arc<Mutex<BLECharacteristic>>,
    output_keyboard: Arc<Mutex<BLECharacteristic>>,
    input_media_keys: Arc<Mutex<BLECharacteristic>>,
    key_report: KeyReport,
    key_count: usize,
}

impl BleKeyboard {
    pub fn new() -> anyhow::Result<Self> {
        let device = BLEDevice::take();
        device
            .security()
            .set_auth(AuthReq::all())
            .set_io_cap(SecurityIOCap::NoInputNoOutput)
            .resolve_rpa();

        let server = device.get_server();
        let mut hid = BLEHIDDevice::new(server);

        let input_keyboard = hid.input_report(KEYBOARD_ID);
        let output_keyboard = hid.output_report(KEYBOARD_ID);
        let input_media_keys = hid.input_report(MEDIA_KEYS_ID);

        hid.manufacturer("Espressif");
        hid.pnp(0x02, 0x05ac, 0x820a, 0x0210);
        hid.hid_info(0x00, 0x01);

        hid.report_map(HID_REPORT_DISCRIPTOR);

        hid.set_battery_level(100);

        let name: &str;

        if KEYBOARD_LEFT_SIDE {
            name = "RUSTBOARD_LEFT";
        } else {
            name = "RUSTBOARD_RIGHT";
        }

        let ble_advertising = device.get_advertising();
        ble_advertising.lock().scan_response(false).set_data(
            BLEAdvertisementData::new()
                .name(name)
                .appearance(0x03C1)
                .add_service_uuid(hid.hid_service().lock().uuid()),
        )?;
        ble_advertising.lock().start()?;

        Ok(Self {
            server,
            input_keyboard,
            output_keyboard,
            input_media_keys,
            key_report: KeyReport {
                modifiers: 0,
                reserved: 0,
                keys: [0; 6],
            },
            key_count: 0,
        })
    }

    pub fn connected(&self) -> bool {
        self.server.connected_count() > 0
    }

    // pub fn press(&mut self) {
    //     // let mut key = char;
    //     // if (key & SHIFT) > 0 {
    //     //     self.key_report.modifiers |= 0x02;
    //     //     key &= !SHIFT;
    //     // }

    //     self.send_report(&self.key_report);
    // }

    pub fn release(&mut self) {
        /* clear the key count */
        self.key_count = 0;

        /* clear the key report */
        self.key_report.modifiers = 0;
        self.key_report.keys.fill(0);

        /* send the report */
        self.send_report();
    }

    fn send_report(&mut self) {
        self.input_keyboard
            .lock()
            .set_from(&self.key_report)
            .notify();
        esp_idf_svc::hal::delay::Ets::delay_ms(5);
    }

    fn set_ble_power_save(&mut self) {
        /* set power save */
        unsafe {
            esp_idf_sys::esp_ble_tx_power_set(
                esp_ble_power_type_t_ESP_BLE_PWR_TYPE_CONN_HDL0,
                esp_power_level_t_ESP_PWR_LVL_N24,
            );
        }
    }

    pub async fn send_key(
        &mut self,
        keys_pressed: &Arc<stdMutex<HashMap<(i8, i8), (Instant, bool)>>>,
    ) -> ! {
        /* initialize layers */
        let mut layers = Layers::new();

        if KEYBOARD_LEFT_SIDE {
            /* For left side of the keyboard */
            layers.initialize_base_layer_left();
            layers.initialize_upper_layer_left();
        } else {
            /* For right side of the keyboard */
            layers.initialize_base_layer_right();
            layers.initialize_upper_layer_right();
        }

        /* initialize set_ble_power_flag */
        let mut set_ble_power_flag = true;

        /* Run the main loop */
        loop {
            if self.connected() {
                /* set ble power save */
                if set_ble_power_flag {
                    /* set power save */
                    self.set_ble_power_save();

                    /* set flag to false */
                    set_ble_power_flag = false;
                }

                /* try to lock the hashmap */
                match keys_pressed.try_lock() {
                    Ok(mut keys_pressed_locked) => {
                        /* store the keys that need to be removed */
                        let mut pressed_keys_to_remove = Vec::new();

                        /* check if there are pressed keys */
                        if !keys_pressed_locked.is_empty() {
                            /* iter trough the pressed keys */
                            for ((row, col), (time_pressed, is_reported)) in
                                keys_pressed_locked.iter_mut()
                            {
                                /* check if the key is already reported */
                                if !*is_reported {
                                    /* check and set the layer */
                                    layers.set_layer(*row, *col);

                                    /* get the pressed key */
                                    if let Some(valid_key) = layers.get(*row, *col) {
                                        /* check and set the modifier */
                                        layers.set_modifier(
                                            valid_key,
                                            &mut self.key_report.modifiers,
                                        );

                                        /* check if the key count is less than 6 */
                                        if self.key_count < 6 {
                                            /* set the key to the buffer */
                                            self.key_report.keys[self.key_count] = *valid_key;

                                            /* increment the key count */
                                            self.key_count += 1;

                                            /* set the key as reported */
                                            *is_reported = true;
                                        } else {
                                            break;
                                        }
                                    }

                                /* if it is reported, and the debounce delay is over, add it to the remove list */
                                } else if Instant::now() >= *time_pressed + DEBOUNCE_DELAY {
                                    pressed_keys_to_remove.push((*row, *col));
                                }
                            }

                            /* remove pressed keys */
                            for key_pressed in pressed_keys_to_remove.iter() {
                                keys_pressed_locked.remove(&key_pressed);
                            }
                        }

                        if self.key_count > 0 {
                            /* send the report */
                            self.send_report();

                            /* release the keys */
                            self.release();
                        }
                    }
                    Err(_) => {}
                }

                delay_us(10).await;
            } else {
                log::info!("Keyboard not connected!");

                /* reset ble power save flag*/
                set_ble_power_flag = true;

                /* sleep for 100ms */
                delay_ms(100).await;
            }
        }
    }
}
