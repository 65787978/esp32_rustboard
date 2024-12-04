/*
*********************************************************************************************
BASE LAYER:

X \ Y|  0  |  1  |  2  |  3  |  4  |  5  |           X \ Y|  0  |  1  |  2  |  3  |  4  |  5  |
   0 |_ESC_|__'__|__,__|__.__|__p__|__y__|              0 |__f__|__g__|__c__|__r__|__l__|__/__|
   1 |_BSP_|__a__|__o__|__e__|__u__|__i__|              1 |__d__|__h__|__t__|__n__|__s__|__-__|
   2 |_CTL_|__;__|__q__|__j__|__k__|__x__|              2 |__b__|__m__|__w__|__v__|__z__|__=__|
   3                   |_LYR_|_SPC_|_SFT_|              3 |_ALT_|_ENT_|_LYR_|

*********************************************************************************************
UPPER LAYER:

X \ Y|  0  |  1  |  2  |  3  |  4  |  5  |           X \ Y|  0  |  1  |  2  |  3  |  4  |  5  |
   0 |_ESC_|__1__|__2__|__3__|__4__|__5__|              0 |__6__|__7__|__8__|__9__|__0__|_____|
   1 |_BSP_|_____|_____|_____|copy_|paste|              1 |_____|_left|_down|__up_|_rght|_____|
   2 |_CTL_|_____|_____|_____|_____|prtsc|              2 |__\__|__[__|__]__|_____|_____|_____|
   3                   |_LYR_|_SPC_|_SFT_|              3 |_ALT_|_ENT_|_LYR_|

*********************************************************************************************
*/
use crate::config::{config::*, enums::*};

use heapless::FnvIndexMap;
pub enum Layer {
    Base,
    Upper,
}
pub struct Layers {
    pub base: FnvIndexMap<(i8, i8), HidKeys, LAYER_INDEXMAP_SIZE>,
    pub upper: FnvIndexMap<(i8, i8), HidKeys, LAYER_INDEXMAP_SIZE>,
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
                #[cfg(feature = "left-side")]
                {
                    LAYER_KEY_LEFT_SIDE
                }

                #[cfg(feature = "right-side")]
                {
                    LAYER_KEY_RIGHT_SIDE
                }
            },
        }
    }
    pub fn initialize_base_layer_left(&mut self) {
        self.base.insert((0, 0), HidKeys::Escape).unwrap(); // ESC
        self.base.insert((0, 1), HidKeys::Quote).unwrap(); // '
        self.base.insert((0, 2), HidKeys::Comma).unwrap(); // ,
        self.base.insert((0, 3), HidKeys::Period).unwrap(); // .
        self.base.insert((0, 4), HidKeys::P).unwrap(); // p
        self.base.insert((0, 5), HidKeys::Y).unwrap(); // y

        self.base.insert((1, 0), HidKeys::Bspace).unwrap(); // BACKSPACE
        self.base.insert((1, 1), HidKeys::A).unwrap(); // a
        self.base.insert((1, 2), HidKeys::O).unwrap(); // o
        self.base.insert((1, 3), HidKeys::E).unwrap(); // e
        self.base.insert((1, 4), HidKeys::U).unwrap(); // u
        self.base.insert((1, 5), HidKeys::I).unwrap(); // i

        self.base.insert((2, 0), HidKeys::Control).unwrap(); // CONTROL
        self.base.insert((2, 1), HidKeys::SemiColon).unwrap(); // ;
        self.base.insert((2, 2), HidKeys::Q).unwrap(); // q
        self.base.insert((2, 3), HidKeys::J).unwrap(); // j
        self.base.insert((2, 4), HidKeys::K).unwrap(); // k
        self.base.insert((2, 5), HidKeys::X).unwrap(); // x

        self.base.insert((3, 0), HidKeys::None).unwrap(); //
        self.base.insert((3, 1), HidKeys::None).unwrap(); //
        self.base.insert((3, 2), HidKeys::None).unwrap(); //
        self.base.insert((3, 3), HidKeys::None).unwrap(); // Layer
        self.base.insert((3, 4), HidKeys::Space).unwrap(); // SPACE
        self.base.insert((3, 5), HidKeys::Shift).unwrap(); // SHIFT
    }

    pub fn initialize_upper_layer_left(&mut self) {
        self.upper.insert((0, 0), HidKeys::Escape).unwrap(); // ESC
        self.upper.insert((0, 1), HidKeys::Num1).unwrap(); // 1
        self.upper.insert((0, 2), HidKeys::Num2).unwrap(); // 2
        self.upper.insert((0, 3), HidKeys::Num3).unwrap(); // 3
        self.upper.insert((0, 4), HidKeys::Num4).unwrap(); // 4
        self.upper.insert((0, 5), HidKeys::Num5).unwrap(); // 5

        self.upper.insert((1, 0), HidKeys::Bspace).unwrap(); // BACKSPACE
        self.upper.insert((1, 1), HidKeys::None).unwrap(); // NONE
        self.upper.insert((1, 2), HidKeys::None).unwrap(); // NONE
        self.upper.insert((1, 3), HidKeys::None).unwrap(); // NONE
        self.upper.insert((1, 4), HidKeys::Copy).unwrap(); // COPY
        self.upper.insert((1, 5), HidKeys::Paste).unwrap(); // PASTE

        self.upper.insert((2, 0), HidKeys::Control).unwrap(); // CONTROL
        self.upper.insert((2, 1), HidKeys::None).unwrap(); // NONE
        self.upper.insert((2, 2), HidKeys::None).unwrap(); // NONE
        self.upper.insert((2, 3), HidKeys::None).unwrap(); // NONE
        self.upper.insert((2, 4), HidKeys::None).unwrap(); // NONE
        self.upper.insert((2, 5), HidKeys::Pscreen).unwrap(); // PSCREEN

        self.upper.insert((3, 0), HidKeys::None).unwrap(); // NONE
        self.upper.insert((3, 1), HidKeys::None).unwrap(); // NONE
        self.upper.insert((3, 2), HidKeys::None).unwrap(); // NONE
        self.upper.insert((3, 3), HidKeys::None).unwrap(); // Layer
        self.upper.insert((3, 4), HidKeys::Space).unwrap(); // SPACE
        self.upper.insert((3, 5), HidKeys::Shift).unwrap(); // SHIFT
    }

    pub fn initialize_base_layer_right(&mut self) {
        self.base.insert((0, 0), HidKeys::F).unwrap(); // f
        self.base.insert((0, 1), HidKeys::G).unwrap(); // g
        self.base.insert((0, 2), HidKeys::C).unwrap(); // c
        self.base.insert((0, 3), HidKeys::R).unwrap(); // r
        self.base.insert((0, 4), HidKeys::L).unwrap(); // l
        self.base.insert((0, 5), HidKeys::Slash).unwrap(); // /

        self.base.insert((1, 0), HidKeys::D).unwrap(); // d
        self.base.insert((1, 1), HidKeys::H).unwrap(); // h
        self.base.insert((1, 2), HidKeys::T).unwrap(); // t
        self.base.insert((1, 3), HidKeys::N).unwrap(); // n
        self.base.insert((1, 4), HidKeys::S).unwrap(); // s
        self.base.insert((1, 5), HidKeys::Minus).unwrap(); // -

        self.base.insert((2, 0), HidKeys::B).unwrap(); // b
        self.base.insert((2, 1), HidKeys::M).unwrap(); // m
        self.base.insert((2, 2), HidKeys::W).unwrap(); // w
        self.base.insert((2, 3), HidKeys::V).unwrap(); // v
        self.base.insert((2, 4), HidKeys::Z).unwrap(); // z
        self.base.insert((2, 5), HidKeys::Equal).unwrap(); // =

        self.base.insert((3, 0), HidKeys::Alt).unwrap(); // ALT
        self.base.insert((3, 1), HidKeys::Enter).unwrap(); // ENTER
        self.base.insert((3, 2), HidKeys::None).unwrap(); // LAYER
        self.base.insert((3, 3), HidKeys::None).unwrap(); // NONE
        self.base.insert((3, 4), HidKeys::None).unwrap(); // NONE
        self.base.insert((3, 5), HidKeys::None).unwrap(); // NONE
    }

    pub fn initialize_upper_layer_right(&mut self) {
        self.upper.insert((0, 0), HidKeys::Num6).unwrap(); // 6
        self.upper.insert((0, 1), HidKeys::Num7).unwrap(); // 7
        self.upper.insert((0, 2), HidKeys::Num8).unwrap(); // 8
        self.upper.insert((0, 3), HidKeys::Num9).unwrap(); // 9
        self.upper.insert((0, 4), HidKeys::Num0).unwrap(); // 0
        self.upper.insert((0, 5), HidKeys::None).unwrap(); // NONE

        self.upper.insert((1, 0), HidKeys::None).unwrap(); // NONE
        self.upper.insert((1, 1), HidKeys::Left).unwrap(); // LEFT
        self.upper.insert((1, 2), HidKeys::Down).unwrap(); // DOWN
        self.upper.insert((1, 3), HidKeys::Up).unwrap(); // UP
        self.upper.insert((1, 4), HidKeys::Right).unwrap(); // RIGHT
        self.upper.insert((1, 5), HidKeys::None).unwrap(); // NONE

        self.upper.insert((2, 0), HidKeys::Backslash).unwrap(); // \
        self.upper.insert((2, 1), HidKeys::Lbracket).unwrap(); // [
        self.upper.insert((2, 2), HidKeys::Rbracket).unwrap(); // ]
        self.upper.insert((2, 3), HidKeys::None).unwrap(); // NONE
        self.upper.insert((2, 4), HidKeys::None).unwrap(); // NONE
        self.upper.insert((2, 5), HidKeys::None).unwrap(); // NONE

        self.upper.insert((3, 0), HidKeys::Alt).unwrap(); // ALT
        self.upper.insert((3, 1), HidKeys::Enter).unwrap(); // ENTER
        self.upper.insert((3, 2), HidKeys::None).unwrap(); // LAYER
        self.upper.insert((3, 3), HidKeys::None).unwrap(); // NONE
        self.upper.insert((3, 4), HidKeys::None).unwrap(); // NONE
        self.upper.insert((3, 5), HidKeys::None).unwrap(); // NONE
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

    pub fn get(&self, row: &i8, col: &i8) -> Option<&HidKeys> {
        /* provide the key depending on the layer */
        match self.state {
            Layer::Base => self.base.get(&(*row, *col)),
            Layer::Upper => self.upper.get(&(*row, *col)),
        }
    }

    pub fn set_modifier(&self, key: &HidKeys, modifier: &mut u8) {
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
