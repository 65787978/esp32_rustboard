pub mod dvorak;
pub mod qwerty;
use crate::config::layers::*;

pub fn provide_layout() -> Layers {
    #[cfg(feature = "dvorak")]
    {
        dvorak::layout()
    }

    #[cfg(feature = "qwerty")]
    {
        qwerty::layout()
    }
}
