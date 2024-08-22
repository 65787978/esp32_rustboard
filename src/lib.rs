/*

___|  0  |  1  |  2  |  3  |  4  |  5  |            ___|  0  |  1  |  2  |  3  |  4  |  5  |
 0 |_ESC_|__1__|__2__|__3__|__4__|__5__|             0 |__6__|__7__|__8__|__9__|__0__|__-__|
 1 |_TAB_|__Q__|__W__|__E__|__R__|__T__|             1 |__Y__|__U__|__I__|__O__|__P__|__{__|
 2 |_CAP_|__A__|__S__|__D__|__F__|__G__|             2 |__H__|__J__|__K__|__L__|__;__|__}__|
 3 |_SFT_|__Z__|__X__|__C__|__V__|__B__|             3 |__N__|__M__|__,__|__.__|__/__|__\__|
 4 |_____|_____|_____|_CTL_|_BSP_|_DEL_|             4 |_CTL_|_ENT_|_SPC_|_FUN_|_____|_____|

*/
use std::collections::HashMap;

#[derive(Clone, Default, Debug)]
pub struct KeyboardLeftSide {
    pub key: HashMap<(u8, u8), bool>,
}

impl KeyboardLeftSide {
    pub fn new() -> KeyboardLeftSide {
        KeyboardLeftSide {
            key: HashMap::new(),
        }
    }

    pub fn initialize_keys(&mut self) {
        self.key.insert((0, 0), false); /* ESC */
        self.key.insert((0, 1), false); /* 1 */
        self.key.insert((0, 2), false); /* 2 */
        self.key.insert((0, 3), false); /* 3 */
        self.key.insert((0, 4), false); /* 4 */
        self.key.insert((0, 5), false); /* 5 */

        self.key.insert((1, 0), false); /* TAB */
        self.key.insert((1, 1), false); /* Q */
        self.key.insert((1, 2), false); /* W */
        self.key.insert((1, 3), false); /* E */
        self.key.insert((1, 4), false); /* R */
        self.key.insert((1, 5), false); /* T */

        self.key.insert((2, 0), false); /* CAP */
        self.key.insert((2, 1), false); /* A */
        self.key.insert((2, 2), false); /* S */
        self.key.insert((2, 3), false); /* D */
        self.key.insert((2, 4), false); /* F */
        self.key.insert((2, 5), false); /* G */

        self.key.insert((3, 0), false); /* SFT */
        self.key.insert((3, 1), false); /* Z */
        self.key.insert((3, 2), false); /* X */
        self.key.insert((3, 3), false); /* C */
        self.key.insert((3, 4), false); /* V */
        self.key.insert((3, 5), false); /* B */

        self.key.insert((4, 0), false); /* placeHolder */
        self.key.insert((4, 1), false); /* placeHolder */
        self.key.insert((4, 2), false); /* placeHolder */
        self.key.insert((4, 3), false); /* CTL */
        self.key.insert((4, 4), false); /* BSP */
        self.key.insert((4, 5), false); /* DEL */
    }
}
