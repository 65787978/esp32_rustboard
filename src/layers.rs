/*
*********************************************************************************************
BASE LAYER:

X \ Y|  0  |  1  |  2  |  3  |  4  |  5  |           X \ Y|  0  |  1  |  2  |  3  |  4  |  5  |
   0 |_ESC_|__1__|__2__|__3__|__4__|__5__|              0 |__6__|__7__|__8__|__9__|__0__|__\__|
   1 |_TAB_|__'__|__,__|__.__|__p__|__y__|              1 |__f__|__g__|__c__|__r__|__l__|__/__|
   2 |_BSP_|__a__|__o__|__e__|__u__|__i__|              2 |__d__|__h__|__t__|__n__|__s__|__-__|
   3 |_CTL_|__;__|__q__|__j__|__k__|__x__|              3 |__b__|__m__|__w__|__v__|__z__|__=__|
   4 |_____|_____|_____|_LYR_|_SFT_|_SPC_|              4 |_ENT_|_ALT_|_LYR_|_____|_____|_____|

*********************************************************************************************
UPPER LAYER:

X \ Y|  0  |  1  |  2  |  3  |  4  |  5  |           X \ Y|  0  |  1  |  2  |  3  |  4  |  5  |
   0 |_ESC_|_F1__|_F2__|_F3__|_F4__|_F5__|              0 |_F6__|_F7__|_F8__|_F9__|_F10_|__[__|
   1 |_TAB_|_____|_____|_____|_____|_____|              1 |_____|_____|_____|_____|_____|__]__|
   2 |_BSP_|_____|_____|_____|_____|_____|              2 |_____|_____|_____|_____|_____|_____|
   3 |_CTL_|_____|_____|_____|_____|_____|              3 |_____|_____|_____|_____|_____|_____|
   4 |_____|_____|_____|_LYR_|_SFT_|_SPC_|              4 |_ENT_|_ALT_|_LYR_|_____|_____|_____|
*/
use crate::enums::*;
use crate::KEYBOARD_LEFT_SIDE;
use crate::LAYER_KEY_LEFT_SIDE;
use crate::LAYER_KEY_RIGHT_SIDE;
use std::collections::HashMap;

pub enum Layer {
    Base,
    Upper,
}
pub struct Layers {
    pub base: HashMap<(i8, i8), u8>,
    pub upper: HashMap<(i8, i8), u8>,
    pub state: Layer,
    layer_key: (i8, i8),
}

impl Layers {
    pub fn new() -> Self {
        Layers {
            base: HashMap::new(),
            upper: HashMap::new(),
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
        self.base.insert((0, 0), HidKeys::Escape as u8); // ESC
        self.base.insert((0, 1), HidKeys::Num1 as u8); // 1
        self.base.insert((0, 2), HidKeys::Num2 as u8); // 2
        self.base.insert((0, 3), HidKeys::Num3 as u8); // 3
        self.base.insert((0, 4), HidKeys::Num4 as u8); // 4
        self.base.insert((0, 5), HidKeys::Num5 as u8); // 5

        self.base.insert((1, 0), HidKeys::Tab as u8); // TAB
        self.base.insert((1, 1), HidKeys::Quote as u8); // '
        self.base.insert((1, 2), HidKeys::Comma as u8); // ,
        self.base.insert((1, 3), HidKeys::Period as u8); // .
        self.base.insert((1, 4), HidKeys::P as u8); // p
        self.base.insert((1, 5), HidKeys::Y as u8); // y

        self.base.insert((2, 0), HidKeys::Capslock as u8); // BACKSPACE
        self.base.insert((2, 1), HidKeys::A as u8); // a
        self.base.insert((2, 2), HidKeys::O as u8); // o
        self.base.insert((2, 3), HidKeys::E as u8); // e
        self.base.insert((2, 4), HidKeys::U as u8); // u
        self.base.insert((2, 5), HidKeys::I as u8); // i

        self.base.insert((3, 0), HidModifiers::Control as u8); // CONTROL
        self.base.insert((3, 1), HidKeys::SemiColon as u8); // ;
        self.base.insert((3, 2), HidKeys::Q as u8); // q
        self.base.insert((3, 3), HidKeys::J as u8); // j
        self.base.insert((3, 4), HidKeys::K as u8); // k
        self.base.insert((3, 5), HidKeys::X as u8); // x

        self.base.insert((4, 0), HidKeys::None as u8); //
        self.base.insert((4, 1), HidKeys::None as u8); //
        self.base.insert((4, 2), HidKeys::None as u8); //
        self.base.insert((4, 3), HidKeys::None as u8); // Layer
        self.base.insert((4, 4), HidModifiers::Shift as u8); // SHIFT
        self.base.insert((4, 5), HidKeys::Space as u8); // SPACE
    }

    pub fn initialize_upper_layer_left(&mut self) {
        self.upper.insert((0, 0), HidKeys::None as u8); // NONE
        self.upper.insert((0, 1), HidKeys::F1 as u8); // F1
        self.upper.insert((0, 2), HidKeys::F2 as u8); // F2
        self.upper.insert((0, 3), HidKeys::F3 as u8); // F3
        self.upper.insert((0, 4), HidKeys::F4 as u8); // F4
        self.upper.insert((0, 5), HidKeys::F5 as u8); // F5

        self.upper.insert((1, 0), HidKeys::Tab as u8); // TAB
        self.upper.insert((1, 1), HidKeys::None as u8); // NONE
        self.upper.insert((1, 2), HidKeys::None as u8); // NONE
        self.upper.insert((1, 3), HidKeys::None as u8); // NONE
        self.upper.insert((1, 4), HidKeys::None as u8); // NONE
        self.upper.insert((1, 5), HidKeys::None as u8); // NONE

        self.upper.insert((2, 0), HidKeys::Bspace as u8); // BACKSPACE
        self.upper.insert((2, 1), HidKeys::None as u8); // NONE
        self.upper.insert((2, 2), HidKeys::None as u8); // NONE
        self.upper.insert((2, 3), HidKeys::None as u8); // NONE
        self.upper.insert((2, 4), HidKeys::None as u8); // NONE
        self.upper.insert((2, 5), HidKeys::None as u8); // NONE

        self.upper.insert((3, 0), HidModifiers::Control as u8); // CONTROL
        self.upper.insert((3, 1), HidKeys::None as u8); // NONE
        self.upper.insert((3, 2), HidKeys::None as u8); // NONE
        self.upper.insert((3, 3), HidKeys::None as u8); // NONE
        self.upper.insert((3, 4), HidKeys::None as u8); // NONE
        self.upper.insert((3, 5), HidKeys::None as u8); // NONE

        self.upper.insert((4, 0), HidKeys::None as u8); // NONE
        self.upper.insert((4, 1), HidKeys::None as u8); // NONE
        self.upper.insert((4, 2), HidKeys::None as u8); // NONE
        self.upper.insert((4, 3), HidKeys::None as u8); // Layer
        self.upper.insert((4, 4), HidModifiers::Shift as u8); // SHIFT
        self.upper.insert((4, 5), HidKeys::Space as u8); // SPACE
    }

    pub fn initialize_base_layer_right(&mut self) {
        self.base.insert((0, 0), HidKeys::Num6 as u8); // 6
        self.base.insert((0, 1), HidKeys::Num7 as u8); // 7
        self.base.insert((0, 2), HidKeys::Num8 as u8); // 8
        self.base.insert((0, 3), HidKeys::Num9 as u8); // 9
        self.base.insert((0, 4), HidKeys::Num0 as u8); // 0
        self.base.insert((0, 5), HidKeys::Backslash as u8); // \

        self.base.insert((1, 0), HidKeys::F as u8); // f
        self.base.insert((1, 1), HidKeys::G as u8); // g
        self.base.insert((1, 2), HidKeys::C as u8); // c
        self.base.insert((1, 3), HidKeys::R as u8); // r
        self.base.insert((1, 4), HidKeys::L as u8); // l
        self.base.insert((1, 5), HidKeys::Slash as u8); // /

        self.base.insert((2, 0), HidKeys::D as u8); // d
        self.base.insert((2, 1), HidKeys::H as u8); // h
        self.base.insert((2, 2), HidKeys::T as u8); // t
        self.base.insert((2, 3), HidKeys::N as u8); // n
        self.base.insert((2, 4), HidKeys::S as u8); // s
        self.base.insert((2, 5), HidKeys::Minus as u8); // -

        self.base.insert((3, 0), HidKeys::B as u8); // b
        self.base.insert((3, 1), HidKeys::M as u8); // m
        self.base.insert((3, 2), HidKeys::W as u8); // w
        self.base.insert((3, 3), HidKeys::V as u8); // v
        self.base.insert((3, 4), HidKeys::Z as u8); // z
        self.base.insert((3, 5), HidKeys::Equal as u8); // =

        self.base.insert((4, 0), HidKeys::Enter as u8); // ENTER
        self.base.insert((4, 1), HidModifiers::Alt as u8); // ALT
        self.base.insert((4, 2), HidKeys::None as u8); // LAYER
        self.base.insert((4, 3), HidKeys::None as u8); // NONE
        self.base.insert((4, 4), HidKeys::None as u8); // NONE
        self.base.insert((4, 5), HidKeys::None as u8); // NONE
    }

    pub fn initialize_upper_layer_right(&mut self) {
        self.upper.insert((0, 0), HidKeys::F6 as u8); // F6
        self.upper.insert((0, 1), HidKeys::F7 as u8); // F7
        self.upper.insert((0, 2), HidKeys::F8 as u8); // F8
        self.upper.insert((0, 3), HidKeys::F9 as u8); // F9
        self.upper.insert((0, 4), HidKeys::F10 as u8); // F10
        self.upper.insert((0, 5), HidKeys::Lbracket as u8); // [

        self.upper.insert((1, 0), HidKeys::None as u8); // NONE
        self.upper.insert((1, 1), HidKeys::None as u8); // NONE
        self.upper.insert((1, 2), HidKeys::None as u8); // NONE
        self.upper.insert((1, 3), HidKeys::None as u8); // NONE
        self.upper.insert((1, 4), HidKeys::None as u8); // NONE
        self.upper.insert((1, 5), HidKeys::Rbracket as u8); // ]

        self.upper.insert((2, 0), HidKeys::None as u8); // NONE
        self.upper.insert((2, 1), HidKeys::None as u8); // NONE
        self.upper.insert((2, 2), HidKeys::None as u8); // NONE
        self.upper.insert((2, 3), HidKeys::None as u8); // NONE
        self.upper.insert((2, 4), HidKeys::None as u8); // NONE
        self.upper.insert((2, 5), HidKeys::None as u8); // NONE

        self.upper.insert((3, 0), HidKeys::None as u8); // NONE
        self.upper.insert((3, 1), HidKeys::None as u8); // NONE
        self.upper.insert((3, 2), HidKeys::None as u8); // NONE
        self.upper.insert((3, 3), HidKeys::None as u8); // NONE
        self.upper.insert((3, 4), HidKeys::None as u8); // NONE
        self.upper.insert((3, 5), HidKeys::None as u8); // NONE

        self.upper.insert((4, 0), HidKeys::Enter as u8); // ENTER
        self.upper.insert((4, 1), HidModifiers::Alt as u8); // ALT
        self.upper.insert((4, 2), HidKeys::None as u8); // LAYER
        self.upper.insert((4, 3), HidKeys::None as u8); // NONE
        self.upper.insert((4, 4), HidKeys::None as u8); // NONE
        self.upper.insert((4, 5), HidKeys::None as u8); // NONE
    }

    pub fn set_layer(&mut self, row: i8, col: i8) {
        /* check if the key pressed is the layer key */
        if (row, col) == self.layer_key {
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

    pub fn get(&self, row: i8, col: i8) -> Option<&u8> {
        match self.state {
            Layer::Base => self.base.get(&(row, col)),
            Layer::Upper => self.upper.get(&(row, col)),
        }
    }

    pub fn set_modifier(&self, key: &u8, modifier: &mut u8) {
        let hid_modifier = HidModifiers::from(*key);

        match hid_modifier {
            HidModifiers::Shift => *modifier |= HidModifiers::Shift as u8,
            HidModifiers::Control => *modifier |= HidModifiers::Control as u8,
            HidModifiers::Alt => *modifier |= HidModifiers::Alt as u8,
            HidModifiers::Super => *modifier |= HidModifiers::Super as u8,
            _ => {}
        }
    }
}
