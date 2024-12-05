use embassy_time::Duration;

use crate::matrix::Key;

/* USER CONFIGURABLE PARAMETERS */
pub const ROWS: usize = 4;
pub const COLS: usize = 6;
pub const DEBOUNCE_DELAY: Duration = Duration::from_millis(80);
pub const DEBOUNCE_DELAY_LAYER_KEY: Duration = Duration::from_millis(300);
pub const SLEEP_DELAY: Duration = Duration::from_millis(120000);
pub const SLEEP_DELAY_INIT: Duration = Duration::from_millis(60000);
pub const PRESSED_KEYS_INDEXMAP_SIZE: usize = 16;
pub const LAYER_INDEXMAP_SIZE: usize = 32;

#[cfg(feature = "left-side")]
pub const LAYER_KEY: Key = Key { row: 3, col: 3 };

#[cfg(feature = "right-side")]
pub const LAYER_KEY: Key = Key { row: 3, col: 2 };
