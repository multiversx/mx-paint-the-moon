use multiversx_sc::derive_imports::*;

#[type_abi]
#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode)]
pub enum Color {
    White = 1,
    Black,
    Blue,
    Red,
    Yellow,
    Green,
    Purple,
    Grey,
}
