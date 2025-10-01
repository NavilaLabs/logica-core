mod eventually;

pub mod prelude {
    #[cfg(feature = "adapter-es-eventually")]
    pub use super::_eventually::*;
}

#[cfg(feature = "adapter-es-eventually")]
mod _eventually {}
