use multiversx_sc::derive_imports::*;

#[type_abi]
#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode)]
pub enum Color {
    Transparent = 0,
    Black = 1,
    White = 2,
    Blue,
    Red,
    Yellow,
    Green,
    Purple,
    Grey,
}
