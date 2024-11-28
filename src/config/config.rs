use embassy_time::Duration;

/* USER CONFIGURABLE PARAMETERS */
pub const ROWS: usize = 4;
pub const COLS: usize = 6;
pub const LAYER_KEY_LEFT_SIDE: (i8, i8) = (3, 3);
pub const LAYER_KEY_RIGHT_SIDE: (i8, i8) = (3, 2);
pub const DEBOUNCE_DELAY: Duration = Duration::from_millis(200);
pub const SLEEP_DELAY: Duration = Duration::from_millis(15000);
pub const SLEEP_DELAY_INIT: Duration = Duration::from_millis(30000);
pub const KEYBOARD_LEFT_SIDE: bool = false;
pub const PRESSED_KEYS_INDEXMAP_SIZE: usize = 16;
pub const LAYER_INDEXMAP_SIZE: usize = 32;
