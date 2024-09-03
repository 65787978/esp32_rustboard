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
use crate::enums::*;
use esp_idf_svc::hal::gpio::*;
use esp_idf_svc::hal::peripherals::Peripherals;
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};

static COL_0_FLAG: AtomicBool = AtomicBool::new(false);
static COL_1_FLAG: AtomicBool = AtomicBool::new(false);
static COL_2_FLAG: AtomicBool = AtomicBool::new(false);
static COL_3_FLAG: AtomicBool = AtomicBool::new(false);
static COL_4_FLAG: AtomicBool = AtomicBool::new(false);
static COL_5_FLAG: AtomicBool = AtomicBool::new(false);

pub static REPORT_DELAY: u32 = 20;

pub const DELAY_DEFAULT: u32 = 20;
pub const DELAY_SAME_KEY: u32 = 60;
pub const PIN_INACTIVE: i32 = -1;
pub const ROW_INIT: u32 = 0;
pub const DEBOUNCE_DELAY: u32 = 10;

pub mod ble_keyboard;
pub mod enums;

pub struct Rows<'a> {
    pub row_0: PinDriver<'a, Gpio0, Output>,
    pub row_1: PinDriver<'a, Gpio1, Output>,
    pub row_2: PinDriver<'a, Gpio12, Output>,
    pub row_3: PinDriver<'a, Gpio18, Output>,
    pub row_4: PinDriver<'a, Gpio19, Output>,
}

pub struct Cols<'a> {
    pub col_0: PinDriver<'a, Gpio2, Input>,
    pub col_1: PinDriver<'a, Gpio3, Input>,
    pub col_2: PinDriver<'a, Gpio10, Input>,
    pub col_3: PinDriver<'a, Gpio6, Input>,
    pub col_4: PinDriver<'a, Gpio7, Input>,
    pub col_5: PinDriver<'a, Gpio4, Input>,
}

pub struct KeyMatrix<'a> {
    pub rows: Rows<'a>,
    pub cols: Cols<'a>,
}
pub struct KeyboardSide<'a> {
    pub base_layer: HashMap<(i32, i32), u8>,
    pub shift_layer: HashMap<(i32, i32), u8>,
    pub upper_layer: HashMap<(i32, i32), u8>,
    pub row_active: u32,
    pub pins_active: [(i32, i32); 6],
    pub pins_active_cnt: usize,
    pub layer: Layer,
    pub key_matrix: KeyMatrix<'a>,
}

impl KeyboardSide<'_> {
    pub fn new() -> KeyboardSide<'static> {
        let peripherals = Peripherals::take().unwrap();

        let mut col_0 = PinDriver::input(peripherals.pins.gpio2).unwrap();
        let mut col_1 = PinDriver::input(peripherals.pins.gpio3).unwrap();
        let mut col_2 = PinDriver::input(peripherals.pins.gpio10).unwrap();
        let mut col_3 = PinDriver::input(peripherals.pins.gpio6).unwrap();
        let mut col_4 = PinDriver::input(peripherals.pins.gpio7).unwrap();
        let mut col_5 = PinDriver::input(peripherals.pins.gpio4).unwrap();

        col_0.set_interrupt_type(InterruptType::PosEdge).unwrap();
        col_1.set_interrupt_type(InterruptType::PosEdge).unwrap();
        col_2.set_interrupt_type(InterruptType::PosEdge).unwrap();
        col_3.set_interrupt_type(InterruptType::PosEdge).unwrap();
        col_4.set_interrupt_type(InterruptType::PosEdge).unwrap();
        col_5.set_interrupt_type(InterruptType::PosEdge).unwrap();

        unsafe { col_0.subscribe(col_0_callback).unwrap() }
        unsafe { col_1.subscribe(col_1_callback).unwrap() }
        unsafe { col_2.subscribe(col_2_callback).unwrap() }
        unsafe { col_3.subscribe(col_3_callback).unwrap() }
        unsafe { col_4.subscribe(col_4_callback).unwrap() }
        unsafe { col_5.subscribe(col_5_callback).unwrap() }

        col_0.enable_interrupt().unwrap();
        col_1.enable_interrupt().unwrap();
        col_2.enable_interrupt().unwrap();
        col_3.enable_interrupt().unwrap();
        col_4.enable_interrupt().unwrap();
        col_5.enable_interrupt().unwrap();

        KeyboardSide {
            base_layer: HashMap::new(),
            shift_layer: HashMap::new(),
            upper_layer: HashMap::new(),
            row_active: ROW_INIT,
            pins_active: [(PIN_INACTIVE, PIN_INACTIVE); 6],
            pins_active_cnt: 0,
            layer: Layer::Base,
            key_matrix: KeyMatrix {
                rows: Rows {
                    row_0: PinDriver::output(peripherals.pins.gpio0).unwrap(),
                    row_1: PinDriver::output(peripherals.pins.gpio1).unwrap(),
                    row_2: PinDriver::output(peripherals.pins.gpio12).unwrap(),
                    row_3: PinDriver::output(peripherals.pins.gpio18).unwrap(),
                    row_4: PinDriver::output(peripherals.pins.gpio19).unwrap(),
                },
                cols: Cols {
                    col_0: col_0,
                    col_1: col_1,
                    col_2: col_2,
                    col_3: col_3,
                    col_4: col_4,
                    col_5: col_5,
                },
            },
        }
    }

    pub fn initialize_layers(&mut self) {
        self.initialie_base_layer();
        // self.initialie_shift_layer();
        self.initialie_upper_layer();
    }

    fn initialie_base_layer(&mut self) {
        self.base_layer.insert((0, 2), HidMapings::Escape as u8); // ESC
        self.base_layer.insert((0, 3), HidMapings::Num1 as u8); // 1
        self.base_layer.insert((0, 10), HidMapings::Num2 as u8); // 2
        self.base_layer.insert((0, 6), HidMapings::Num3 as u8); // 3
        self.base_layer.insert((0, 7), HidMapings::Num4 as u8); // 4
        self.base_layer.insert((0, 4), HidMapings::Num5 as u8); // 5

        self.base_layer.insert((1, 2), HidMapings::Tab as u8); // TAB
        self.base_layer.insert((1, 3), HidMapings::Q as u8); // q
        self.base_layer.insert((1, 10), HidMapings::W as u8); // w
        self.base_layer.insert((1, 6), HidMapings::E as u8); // e
        self.base_layer.insert((1, 7), HidMapings::R as u8); // r
        self.base_layer.insert((1, 4), HidMapings::T as u8); // t

        self.base_layer.insert((12, 2), HidMapings::Capslock as u8); // CAP
        self.base_layer.insert((12, 3), HidMapings::A as u8); // a
        self.base_layer.insert((12, 10), HidMapings::S as u8); // s
        self.base_layer.insert((12, 6), HidMapings::D as u8); // d
        self.base_layer.insert((12, 7), HidMapings::F as u8); // f
        self.base_layer.insert((12, 4), HidMapings::G as u8); // g

        self.base_layer.insert((18, 2), HidMapings::No as u8); //
        self.base_layer.insert((18, 3), HidMapings::Z as u8); // z
        self.base_layer.insert((18, 10), HidMapings::X as u8); // x
        self.base_layer.insert((18, 6), HidMapings::C as u8); // c
        self.base_layer.insert((18, 7), HidMapings::V as u8); // v
        self.base_layer.insert((18, 4), HidMapings::B as u8); // b

        self.base_layer.insert((19, 2), HidMapings::No as u8); //
        self.base_layer.insert((19, 3), HidMapings::No as u8); //
        self.base_layer.insert((19, 10), HidMapings::No as u8); // LAYER
        self.base_layer.insert((19, 6), HidMapings::No as u8); // CONTROL
        self.base_layer.insert((19, 7), HidMapings::No as u8); // SHIFT
        self.base_layer.insert((19, 4), HidMapings::Enter as u8); // ENTER
    }

    // fn initialie_shift_layer(&mut self) {
    //     self.shift_layer.insert((0, 2), HidMapings::Escape as u8); // ESC
    //     self.shift_layer.insert((0, 3), HidMapings::No as u8); // !
    //     self.shift_layer.insert((0, 10), HidMapings::No as u8); // @
    //     self.shift_layer.insert((0, 6), HidMapings::No as u8); // #
    //     self.shift_layer.insert((0, 7), HidMapings::No as u8); // $
    //     self.shift_layer.insert((0, 4), HidMapings::No as u8); // %

    //     self.shift_layer.insert((1, 2), HidMapings::Tab as u8); // TAB
    //     self.shift_layer.insert((1, 3), HidMapings::UpperQ as u8); // Q
    //     self.shift_layer.insert((1, 10), HidMapings::UpperW as u8); // W
    //     self.shift_layer.insert((1, 6), HidMapings::UpperE as u8); // E
    //     self.shift_layer.insert((1, 7), HidMapings::UpperR as u8); // R
    //     self.shift_layer.insert((1, 4), HidMapings::UpperT as u8); // T

    //     self.shift_layer.insert((12, 2), HidMapings::Capslock as u8); // CAPSLOCK
    //     self.shift_layer.insert((12, 3), HidMapings::UpperA as u8); // A
    //     self.shift_layer.insert((12, 10), HidMapings::UpperS as u8); // S
    //     self.shift_layer.insert((12, 6), HidMapings::UpperD as u8); // D
    //     self.shift_layer.insert((12, 7), HidMapings::UpperF as u8); // F
    //     self.shift_layer.insert((12, 4), HidMapings::UpperG as u8); // G

    //     self.shift_layer.insert((18, 2), HidMapings::No as u8); //
    //     self.shift_layer.insert((18, 3), HidMapings::UpperZ as u8); // Z
    //     self.shift_layer.insert((18, 10), HidMapings::UpperX as u8); // X
    //     self.shift_layer.insert((18, 6), HidMapings::UpperC as u8); // C
    //     self.shift_layer.insert((18, 7), HidMapings::UpperV as u8); // V
    //     self.shift_layer.insert((18, 4), HidMapings::UpperB as u8); // B

    //     self.shift_layer.insert((19, 2), HidMapings::No as u8); //
    //     self.shift_layer.insert((19, 3), HidMapings::No as u8); //
    //     self.shift_layer.insert((19, 10), HidMapings::No as u8); // LAYER
    //     self.shift_layer.insert((19, 6), HidMapings::No as u8); // CONTROL
    //     self.shift_layer.insert((19, 7), HidMapings::No as u8); // SHIFT
    //     self.shift_layer.insert((19, 4), HidMapings::Enter as u8); // ENTER
    // }

    fn initialie_upper_layer(&mut self) {
        self.upper_layer.insert((0, 2), HidMapings::Escape as u8); // ESC
        self.upper_layer.insert((0, 3), HidMapings::No as u8); //
        self.upper_layer.insert((0, 10), HidMapings::No as u8); //
        self.upper_layer.insert((0, 6), HidMapings::No as u8); //
        self.upper_layer.insert((0, 7), HidMapings::No as u8); //
        self.upper_layer.insert((0, 4), HidMapings::No as u8); //

        self.upper_layer.insert((1, 2), HidMapings::Tab as u8); // TAB
        self.upper_layer.insert((1, 3), HidMapings::No as u8); //
        self.upper_layer.insert((1, 10), HidMapings::Up as u8); // UP
        self.upper_layer.insert((1, 6), HidMapings::No as u8); //
        self.upper_layer.insert((1, 7), HidMapings::No as u8); //
        self.upper_layer.insert((1, 4), HidMapings::No as u8); //

        self.upper_layer.insert((12, 2), HidMapings::Capslock as u8); // CAPSLOCK
        self.upper_layer.insert((12, 3), HidMapings::Left as u8); // LEFT
        self.upper_layer.insert((12, 10), HidMapings::Down as u8); // DOWN
        self.upper_layer.insert((12, 6), HidMapings::Right as u8); // RIGHT
        self.upper_layer.insert((12, 7), HidMapings::No as u8); //
        self.upper_layer.insert((12, 4), HidMapings::No as u8); //

        self.upper_layer.insert((18, 2), HidMapings::No as u8); //
        self.upper_layer.insert((18, 3), HidMapings::No as u8); //
        self.upper_layer.insert((18, 10), HidMapings::No as u8); //
        self.upper_layer.insert((18, 6), HidMapings::No as u8); //
        self.upper_layer.insert((18, 7), HidMapings::No as u8); //
        self.upper_layer.insert((18, 4), HidMapings::No as u8); //

        self.upper_layer.insert((19, 2), HidMapings::No as u8); //
        self.upper_layer.insert((19, 3), HidMapings::No as u8); //
        self.upper_layer.insert((19, 10), HidMapings::No as u8); // LAYER
        self.upper_layer.insert((19, 6), HidMapings::No as u8); // CONTROL
        self.upper_layer.insert((19, 7), HidMapings::No as u8); // SHIFT
        self.upper_layer.insert((19, 4), HidMapings::Enter as u8); // ENTER
    }

    /*
    pub fn check_pins(&mut self) {
        let mut button_state = false;

        if self.key_matrix.cols.col_0.is_high() {
            FreeRtos::delay_ms(DEBOUNCE_DELAY);
            button_state = true;
        }
        /* check again if col is high */
        if self.key_matrix.cols.col_0.is_high() && button_state {
            self.pins_active.1 = self.key_matrix.cols.col_0.pin();
        }

        /*********************************************************/

        if self.key_matrix.cols.col_1.is_high() != button_state {
            FreeRtos::delay_ms(DEBOUNCE_DELAY);
            button_state = true;
        }
        /* check again if col is high */
        if self.key_matrix.cols.col_1.is_high() && button_state {
            self.pins_active.1 = self.key_matrix.cols.col_1.pin();
        }

        /*********************************************************/

        if self.key_matrix.cols.col_2.is_high() != button_state {
            FreeRtos::delay_ms(DEBOUNCE_DELAY);
            button_state = true;
        }
        /* check again if col is high */
        if self.key_matrix.cols.col_2.is_high() && button_state {
            self.pins_active.1 = self.key_matrix.cols.col_2.pin();
        }

        /*********************************************************/

        if self.key_matrix.cols.col_3.is_high() != button_state {
            FreeRtos::delay_ms(DEBOUNCE_DELAY);
            button_state = true;
        }
        /* check again if col is high */
        if self.key_matrix.cols.col_3.is_high() && button_state {
            self.pins_active.1 = self.key_matrix.cols.col_3.pin();
        }

        /*********************************************************/

        if self.key_matrix.cols.col_4.is_high() != button_state {
            FreeRtos::delay_ms(DEBOUNCE_DELAY);
            button_state = true;
        }
        /* check again if col is high */
        if self.key_matrix.cols.col_4.is_high() && button_state {
            self.pins_active.1 = self.key_matrix.cols.col_4.pin();
        }

        /*********************************************************/

        if self.key_matrix.cols.col_5.is_high() != button_state {
            FreeRtos::delay_ms(DEBOUNCE_DELAY);
            button_state = true;
        }
        /* check again if col is high */
        if self.key_matrix.cols.col_5.is_high() && button_state {
            self.pins_active.1 = self.key_matrix.cols.col_5.pin();
        }
    }
    */

    pub fn check_cols(&mut self) {
        if COL_0_FLAG.load(Ordering::Relaxed) {
            COL_0_FLAG.store(false, Ordering::Relaxed);
            self.pins_active[self.pins_active_cnt].1 = self.key_matrix.cols.col_0.pin();
            self.pins_active_cnt += 1;
        }

        if COL_1_FLAG.load(Ordering::Relaxed) {
            COL_1_FLAG.store(false, Ordering::Relaxed);
            self.pins_active[self.pins_active_cnt].1 = self.key_matrix.cols.col_1.pin();
            self.pins_active_cnt += 1;
        }

        if COL_2_FLAG.load(Ordering::Relaxed) {
            COL_2_FLAG.store(false, Ordering::Relaxed);
            self.pins_active[self.pins_active_cnt].1 = self.key_matrix.cols.col_2.pin();
            self.pins_active_cnt += 1;
        }

        if COL_3_FLAG.load(Ordering::Relaxed) {
            COL_3_FLAG.store(false, Ordering::Relaxed);
            self.pins_active[self.pins_active_cnt].1 = self.key_matrix.cols.col_3.pin();
            self.pins_active_cnt += 1;
        }

        if COL_4_FLAG.load(Ordering::Relaxed) {
            COL_4_FLAG.store(false, Ordering::Relaxed);
            self.pins_active[self.pins_active_cnt].1 = self.key_matrix.cols.col_4.pin();
            self.pins_active_cnt += 1;
        }

        if COL_5_FLAG.load(Ordering::Relaxed) {
            COL_5_FLAG.store(false, Ordering::Relaxed);
            self.pins_active[self.pins_active_cnt].1 = self.key_matrix.cols.col_5.pin();
            self.pins_active_cnt += 1;
        }
    }

    pub fn set_rows(&mut self, state: &'static str) {
        match state {
            "high" => match self.row_active {
                0 => {
                    self.key_matrix.rows.row_0.set_high().unwrap();
                    self.pins_active[self.pins_active_cnt].0 = self.key_matrix.rows.row_0.pin()
                }
                1 => {
                    self.key_matrix.rows.row_1.set_high().unwrap();
                    self.pins_active[self.pins_active_cnt].0 = self.key_matrix.rows.row_1.pin()
                }
                2 => {
                    self.key_matrix.rows.row_2.set_high().unwrap();
                    self.pins_active[self.pins_active_cnt].0 = self.key_matrix.rows.row_2.pin()
                }
                3 => {
                    self.key_matrix.rows.row_3.set_high().unwrap();
                    self.pins_active[self.pins_active_cnt].0 = self.key_matrix.rows.row_3.pin()
                }
                4 => {
                    self.key_matrix.rows.row_4.set_high().unwrap();
                    self.pins_active[self.pins_active_cnt].0 = self.key_matrix.rows.row_4.pin()
                }
                _ => {}
            },
            "low" => {
                match self.row_active {
                    0 => self.key_matrix.rows.row_0.set_low().unwrap(),
                    1 => self.key_matrix.rows.row_1.set_low().unwrap(),
                    2 => self.key_matrix.rows.row_2.set_low().unwrap(),
                    3 => self.key_matrix.rows.row_3.set_low().unwrap(),
                    4 => self.key_matrix.rows.row_4.set_low().unwrap(),
                    _ => {}
                }
                /* Increment the active row */
                self.row_active = (self.row_active + 1) % 5;
            }

            _ => {}
        }
    }

    // pub fn provide_value(&mut self) -> Option<&u8> {
    //     match self.layer {
    //         Layer::Base => self.base_layer.get(&self.pins_active),
    //         Layer::Shift => self.shift_layer.get(&self.pins_active),
    //         Layer::Upper => self.upper_layer.get(&self.pins_active),
    //     }
    // }
}

fn col_0_callback() {
    /* Assert FLAG indicating a press button happened */
    COL_0_FLAG.store(true, Ordering::Relaxed);
}

fn col_1_callback() {
    /* Assert FLAG indicating a press button happened */
    COL_1_FLAG.store(true, Ordering::Relaxed);
}

fn col_2_callback() {
    /* Assert FLAG indicating a press button happened */
    COL_2_FLAG.store(true, Ordering::Relaxed);
}

fn col_3_callback() {
    /* Assert FLAG indicating a press button happened */
    COL_3_FLAG.store(true, Ordering::Relaxed);
}

fn col_4_callback() {
    /* Assert FLAG indicating a press button happened */
    COL_4_FLAG.store(true, Ordering::Relaxed);
}

fn col_5_callback() {
    /* Assert FLAG indicating a press button happened */
    COL_5_FLAG.store(true, Ordering::Relaxed);
}
