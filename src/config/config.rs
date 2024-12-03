use embassy_time::Duration;

/* USER CONFIGURABLE PARAMETERS */
pub const ROWS: usize = 4;
pub const COLS: usize = 6;
pub const LAYER_KEY_LEFT_SIDE: (i8, i8) = (3, 3);
pub const LAYER_KEY_RIGHT_SIDE: (i8, i8) = (3, 2);
pub const DEBOUNCE_DELAY: Duration = Duration::from_millis(50);
pub const SLEEP_DELAY: Duration = Duration::from_millis(120000);
pub const SLEEP_DELAY_INIT: Duration = Duration::from_millis(60000);
pub const PRESSED_KEYS_INDEXMAP_SIZE: usize = 16;
pub const LAYER_INDEXMAP_SIZE: usize = 32;
