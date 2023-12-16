#[cfg(any(feature = "front", feature = "all"))]
pub mod front {
    pub use blockly_front::*;
    pub mod macros {
        pub use blockly_macros::blockly_ir;
    }
}

#[cfg(any(feature = "back", feature = "all"))]
pub use blockly_back as back;
