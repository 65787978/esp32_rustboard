use crate::config::{config::*, enums::*, layout::*};

use heapless::FnvIndexMap;
pub enum Layer {
    Base,
    Upper,
}
pub struct Layers {
    pub base: FnvIndexMap<(i8, i8), HidKeys, LAYER_INDEXMAP_SIZE>,
    pub upper: FnvIndexMap<(i8, i8), HidKeys, LAYER_INDEXMAP_SIZE>,
}

impl Layers {
    pub fn new() -> Self {
        Layers {
            base: FnvIndexMap::new(),
            upper: FnvIndexMap::new(),
        }
    }
    pub fn load_layout(&mut self) {
        *self = provide_layout();
    }

    pub fn get(&mut self, row: &i8, col: &i8, layer_state: &Layer) -> Option<&HidKeys> {
        /* provide the key depending on the layer */
        match layer_state {
            Layer::Base => self.base.get(&(*row, *col)),
            Layer::Upper => self.upper.get(&(*row, *col)),
        }
    }
}
