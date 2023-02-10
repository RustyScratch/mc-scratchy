//! # Create Scratch project in Rust

pub mod blocks;
pub mod scripting;

macro_rules! all_derive {
    (
        #[derive $derives:tt]
        $($item:item)*
    ) => {
        $(
            #[derive $derives]
            $item
        )*
    }
}
pub(crate) use all_derive;
