use crate::{
    config::{config::*, enums::*, layout::*},
    debounce::{Debounce, KEY_RELEASED},
    matrix::Key,
};

use heapless::FnvIndexMap;
pub enum Layer {
    Base,
    Upper,
}
pub struct Layers {
    pub base: FnvIndexMap<(i8, i8), HidKeys, LAYER_INDEXMAP_SIZE>,
    pub upper: FnvIndexMap<(i8, i8), HidKeys, LAYER_INDEXMAP_SIZE>,
    pub state: Layer,
    layer_key: Key,
}

impl Layers {
    pub fn new() -> Self {
        Layers {
            base: FnvIndexMap::new(),
            upper: FnvIndexMap::new(),
            state: Layer::Base,
            layer_key: LAYER_KEY,
        }
    }
    pub fn load_layout(&mut self) {
        *self = provide_layout();
    }

    pub fn set_layer(&mut self, key: &Key, debounce: &mut Debounce) {
        /* check if the key pressed is the layer key */
        if *key == self.layer_key {
            /* change the layer */
            match self.state {
                Layer::Base => {
                    self.state = Layer::Upper;
                }
                Layer::Upper => {
                    self.state = Layer::Base;
                }
            }

            debounce.key_state = KEY_RELEASED;
        }
    }

    pub fn get(&self, row: &i8, col: &i8) -> Option<&HidKeys> {
        /* provide the key depending on the layer */
        match self.state {
            Layer::Base => self.base.get(&(*row, *col)),
            Layer::Upper => self.upper.get(&(*row, *col)),
        }
    }

    pub fn check_modifier(&self, key: &HidKeys) -> Option<u8> {
        /* map the key to a modifier */
        let hid_modifier = HidModifiers::from(*key);

        /* set the modifier */
        match hid_modifier {
            HidModifiers::Shift => Some(HidModifiers::Shift as u8),
            HidModifiers::Control => Some(HidModifiers::Control as u8),
            HidModifiers::Alt => Some(HidModifiers::Alt as u8),
            HidModifiers::Super => Some(HidModifiers::Super as u8),
            _ => None,
        }
    }
}
