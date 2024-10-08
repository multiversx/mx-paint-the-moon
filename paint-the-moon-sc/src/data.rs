use multiversx_sc::derive_imports::*;

#[type_abi]
#[derive(
    TopEncode, TopDecode, NestedEncode, NestedDecode, Copy, Clone, PartialEq, ManagedVecItem,
)]
pub enum Color {
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
#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, ManagedVecItem)]
pub struct Point {
    pub coordinates: u64,
    pub color: Color,
}
