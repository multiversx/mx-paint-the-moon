use multiversx_sc::derive_imports::*;

pub const MAX_HEIGHT: u32 = 500;
pub const MAX_WIDTH: u32 = 500;

#[type_abi]
#[derive(
    TopEncode, TopDecode, NestedEncode, NestedDecode, Copy, Clone, PartialEq, ManagedVecItem, Debug,
)]
pub enum Color {
    Transparent,
    White,
    Black,
    Blue,
    Red,
    Yellow,
    Green,
    Purple,
    Grey,
}

#[type_abi]
#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, Clone, Copy, Debug, PartialEq)]
pub struct Coordinates(pub u32, pub u32);
