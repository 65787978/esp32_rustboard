/*
*********************************************************************************************
BASE LAYER:

X \ Y|  0  |  1  |  2  |  3  |  4  |  5  |           X \ Y|  0  |  1  |  2  |  3  |  4  |  5  |
   0 |_ESC_|__'__|__,__|__.__|__p__|__y__|              0 |__f__|__g__|__c__|__r__|__l__|__/__|
   1 |_BSP_|__a__|__o__|__e__|__u__|__i__|              1 |__d__|__h__|__t__|__n__|__s__|__-__|
   2 |_CTL_|__;__|__q__|__j__|__k__|__x__|              2 |__b__|__m__|__w__|__v__|__z__|__=__|
   3                   |_LYR_|_SFT_|_SPC_|              3 |_ENT_|_ALT_|_LYR_|

*********************************************************************************************
UPPER LAYER:

X \ Y|  0  |  1  |  2  |  3  |  4  |  5  |           X \ Y|  0  |  1  |  2  |  3  |  4  |  5  |
   0 |_ESC_|__1__|__2__|__3__|__4__|__5__|              0 |__6__|__7__|__8__|__9__|__0__|_____|
   1 |_BSP_|_____|_____|_____|copy_|paste|              1 |_____|_left|_down|__up_|_rght|_____|
   2 |_CTL_|_____|_____|_____|_____|prtsc|              2 |__\__|__[__|__]__|_____|_____|_____|
   3                   |_LYR_|_SFT_|_SPC_|              3 |_ENT_|_ALT_|_LYR_|

*********************************************************************************************
*/
use crate::config::{config::*, enums::*};

use heapless::FnvIndexMap;
pub enum Layer {
    Base,
    Upper,
}
pub struct Layers {
    pub base: FnvIndexMap<(i8, i8), u8, LAYER_INDEXMAP_SIZE>,
    pub upper: FnvIndexMap<(i8, i8), u8, LAYER_INDEXMAP_SIZE>,
    pub state: Layer,
    layer_key: (i8, i8),
}

impl Layers {
    pub fn new() -> Self {
        Layers {
            base: FnvIndexMap::new(),
            upper: FnvIndexMap::new(),
            state: Layer::Base,
            layer_key: {
                if KEYBOARD_LEFT_SIDE {
                    LAYER_KEY_LEFT_SIDE
                } else {
                    LAYER_KEY_RIGHT_SIDE
                }
            },
        }
    }
    pub fn initialize_base_layer_left(&mut self) {
        self.base.insert((0, 0), HidKeys::Escape as u8).unwrap(); // ESC
        self.base.insert((0, 1), HidKeys::Quote as u8).unwrap(); // '
        self.base.insert((0, 2), HidKeys::Comma as u8).unwrap(); // ,
        self.base.insert((0, 3), HidKeys::Period as u8).unwrap(); // .
        self.base.insert((0, 4), HidKeys::P as u8).unwrap(); // p
        self.base.insert((0, 5), HidKeys::Y as u8).unwrap(); // y

        self.base.insert((1, 0), HidKeys::Bspace as u8).unwrap(); // BACKSPACE
        self.base.insert((1, 1), HidKeys::A as u8).unwrap(); // a
        self.base.insert((1, 2), HidKeys::O as u8).unwrap(); // o
        self.base.insert((1, 3), HidKeys::E as u8).unwrap(); // e
        self.base.insert((1, 4), HidKeys::U as u8).unwrap(); // u
        self.base.insert((1, 5), HidKeys::I as u8).unwrap(); // i

        self.base
            .insert((2, 0), HidModifiers::Control as u8)
            .unwrap(); // CONTROL
        self.base.insert((2, 1), HidKeys::SemiColon as u8).unwrap(); // ;
        self.base.insert((2, 2), HidKeys::Q as u8).unwrap(); // q
        self.base.insert((2, 3), HidKeys::J as u8).unwrap(); // j
        self.base.insert((2, 4), HidKeys::K as u8).unwrap(); // k
        self.base.insert((2, 5), HidKeys::X as u8).unwrap(); // x

        self.base.insert((3, 0), HidKeys::None as u8).unwrap(); //
        self.base.insert((3, 1), HidKeys::None as u8).unwrap(); //
        self.base.insert((3, 2), HidKeys::None as u8).unwrap(); //
        self.base.insert((3, 3), HidKeys::None as u8).unwrap(); // Layer
        self.base.insert((3, 4), HidModifiers::Shift as u8).unwrap(); // SHIFT
        self.base.insert((3, 5), HidKeys::Space as u8).unwrap(); // SPACE
    }

    pub fn initialize_upper_layer_left(&mut self) {
        self.upper.insert((0, 0), HidKeys::Escape as u8).unwrap(); // ESC
        self.upper.insert((0, 1), HidKeys::Num1 as u8).unwrap(); // 1
        self.upper.insert((0, 2), HidKeys::Num2 as u8).unwrap(); // 2
        self.upper.insert((0, 3), HidKeys::Num3 as u8).unwrap(); // 3
        self.upper.insert((0, 4), HidKeys::Num4 as u8).unwrap(); // 4
        self.upper.insert((0, 5), HidKeys::Num5 as u8).unwrap(); // 5

        self.upper.insert((1, 0), HidKeys::Bspace as u8).unwrap(); // BACKSPACE
        self.upper.insert((1, 1), HidKeys::None as u8).unwrap(); // NONE
        self.upper.insert((1, 2), HidKeys::None as u8).unwrap(); // NONE
        self.upper.insert((1, 3), HidKeys::None as u8).unwrap(); // NONE
        self.upper.insert((1, 4), HidKeys::Copy as u8).unwrap(); // COPY
        self.upper.insert((1, 5), HidKeys::Paste as u8).unwrap(); // PASTE

        self.upper
            .insert((2, 0), HidModifiers::Control as u8)
            .unwrap(); // CONTROL
        self.upper.insert((2, 1), HidKeys::None as u8).unwrap(); // NONE
        self.upper.insert((2, 2), HidKeys::None as u8).unwrap(); // NONE
        self.upper.insert((2, 3), HidKeys::None as u8).unwrap(); // NONE
        self.upper.insert((2, 4), HidKeys::None as u8).unwrap(); // NONE
        self.upper.insert((2, 5), HidKeys::Pscreen as u8).unwrap(); // PSCREEN

        self.upper.insert((3, 0), HidModifiers::None as u8).unwrap(); // NONE
        self.upper.insert((3, 1), HidModifiers::None as u8).unwrap(); // NONE
        self.upper.insert((3, 2), HidModifiers::None as u8).unwrap(); // NONE
        self.upper.insert((3, 3), HidKeys::None as u8).unwrap(); // Layer
        self.upper
            .insert((3, 4), HidModifiers::Shift as u8)
            .unwrap(); // SHIFT
        self.upper.insert((3, 5), HidKeys::Space as u8).unwrap(); // SPACE
    }

    pub fn initialize_base_layer_right(&mut self) {
        self.base.insert((0, 0), HidKeys::F as u8).unwrap(); // f
        self.base.insert((0, 1), HidKeys::G as u8).unwrap(); // g
        self.base.insert((0, 2), HidKeys::C as u8).unwrap(); // c
        self.base.insert((0, 3), HidKeys::R as u8).unwrap(); // r
        self.base.insert((0, 4), HidKeys::L as u8).unwrap(); // l
        self.base.insert((0, 5), HidKeys::Slash as u8).unwrap(); // /

        self.base.insert((1, 0), HidKeys::D as u8).unwrap(); // d
        self.base.insert((1, 1), HidKeys::H as u8).unwrap(); // h
        self.base.insert((1, 2), HidKeys::T as u8).unwrap(); // t
        self.base.insert((1, 3), HidKeys::N as u8).unwrap(); // n
        self.base.insert((1, 4), HidKeys::S as u8).unwrap(); // s
        self.base.insert((1, 5), HidKeys::Minus as u8).unwrap(); // -

        self.base.insert((2, 0), HidKeys::B as u8).unwrap(); // b
        self.base.insert((2, 1), HidKeys::M as u8).unwrap(); // m
        self.base.insert((2, 2), HidKeys::W as u8).unwrap(); // w
        self.base.insert((2, 3), HidKeys::V as u8).unwrap(); // v
        self.base.insert((2, 4), HidKeys::Z as u8).unwrap(); // z
        self.base.insert((2, 5), HidKeys::Equal as u8).unwrap(); // =

        self.base.insert((3, 0), HidKeys::Enter as u8).unwrap(); // ENTER
        self.base.insert((3, 1), HidModifiers::Alt as u8).unwrap(); // ALT
        self.base.insert((3, 2), HidKeys::None as u8).unwrap(); // LAYER
        self.base.insert((3, 3), HidKeys::None as u8).unwrap(); // NONE
        self.base.insert((3, 4), HidKeys::None as u8).unwrap(); // NONE
        self.base.insert((3, 5), HidKeys::None as u8).unwrap(); // NONE
    }

    pub fn initialize_upper_layer_right(&mut self) {
        self.upper.insert((0, 0), HidKeys::Num6 as u8).unwrap(); // 6
        self.upper.insert((0, 1), HidKeys::Num7 as u8).unwrap(); // 7
        self.upper.insert((0, 2), HidKeys::Num8 as u8).unwrap(); // 8
        self.upper.insert((0, 3), HidKeys::Num9 as u8).unwrap(); // 9
        self.upper.insert((0, 4), HidKeys::Num0 as u8).unwrap(); // 0
        self.upper.insert((0, 5), HidKeys::None as u8).unwrap(); // NONE

        self.upper.insert((1, 0), HidKeys::None as u8).unwrap(); // NONE
        self.upper.insert((1, 1), HidKeys::Left as u8).unwrap(); // LEFT
        self.upper.insert((1, 2), HidKeys::Down as u8).unwrap(); // DOWN
        self.upper.insert((1, 3), HidKeys::Up as u8).unwrap(); // UP
        self.upper.insert((1, 4), HidKeys::Right as u8).unwrap(); // RIGHT
        self.upper.insert((1, 5), HidKeys::None as u8).unwrap(); // NONE

        self.upper.insert((2, 0), HidKeys::Backslash as u8).unwrap(); // \
        self.upper.insert((2, 1), HidKeys::Lbracket as u8).unwrap(); // [
        self.upper.insert((2, 2), HidKeys::Rbracket as u8).unwrap(); // ]
        self.upper.insert((2, 3), HidKeys::None as u8).unwrap(); // NONE
        self.upper.insert((2, 4), HidKeys::None as u8).unwrap(); // NONE
        self.upper.insert((2, 5), HidKeys::None as u8).unwrap(); // NONE

        self.upper.insert((3, 0), HidKeys::Enter as u8).unwrap(); // ENTER
        self.upper.insert((3, 1), HidModifiers::Alt as u8).unwrap(); // ALT
        self.upper.insert((3, 2), HidKeys::None as u8).unwrap(); // LAYER
        self.upper.insert((3, 3), HidKeys::None as u8).unwrap(); // NONE
        self.upper.insert((3, 4), HidKeys::None as u8).unwrap(); // NONE
        self.upper.insert((3, 5), HidKeys::None as u8).unwrap(); // NONE
    }

    pub fn set_layer(&mut self, row: &i8, col: &i8) {
        /* check if the key pressed is the layer key */
        if (*row, *col) == self.layer_key {
            /* change the layer */
            match self.state {
                Layer::Base => {
                    self.state = Layer::Upper;
                }
                Layer::Upper => {
                    self.state = Layer::Base;
                }
            }
        }
    }

    pub fn get(&self, row: &i8, col: &i8) -> Option<&u8> {
        /* provide the key depending on the layer */
        match self.state {
            Layer::Base => self.base.get(&(*row, *col)),
            Layer::Upper => self.upper.get(&(*row, *col)),
        }
    }

    pub fn set_modifier(&self, key: &u8, modifier: &mut u8) {
        /* map the key to a modifier */
        let hid_modifier = HidModifiers::from(*key);

        /* set the modifier */
        match hid_modifier {
            HidModifiers::Shift => *modifier |= HidModifiers::Shift as u8,
            HidModifiers::Control => *modifier |= HidModifiers::Control as u8,
            HidModifiers::Alt => *modifier |= HidModifiers::Alt as u8,
            HidModifiers::Super => *modifier |= HidModifiers::Super as u8,
            _ => {}
        }
    }
}
