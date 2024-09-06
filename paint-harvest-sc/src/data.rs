use multiversx_sc::derive_imports::*;

pub const ISSUE_COST: u64 = 50000000000000000;

#[type_abi]
#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, Clone, Copy, ManagedVecItem)]
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
#[derive(ManagedVecItem)]
pub struct RewardPerColor {
    pub color: Color,
    pub amount: u64,
}

#[type_abi]
#[derive(TopEncode, TopDecode)]
pub struct UserInfo {
    pub nft_nonce: u64,
    pub current_harvest_color: Color,
    pub start_timestamp: u64,
    // accumulated_resources
}
