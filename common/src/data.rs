use multiversx_sc::derive_imports::*;

pub const ISSUE_COST: u64 = 50000000000000000;

pub const MAX_HEIGHT: u32 = 500;
pub const MAX_WIDTH: u32 = 500;

#[type_abi]
#[derive(
    TopEncode, TopDecode, NestedEncode, NestedDecode, Copy, Clone, PartialEq, ManagedVecItem, Debug,
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
#[derive(
    TopEncode, TopDecode, NestedEncode, NestedDecode, ManagedVecItem, Copy, Clone, Debug, PartialEq,
)]
pub struct Point {
    pub coordinates: u64,
    pub color: Color,
}

#[type_abi]
#[derive(TopEncode, TopDecode)]
pub struct UserInfo {
    pub nft_nonce: u64,
    pub current_harvest_color: Color,
    pub start_timestamp: u64,
}
