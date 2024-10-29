use multiversx_sc::derive_imports::*;
use paint_the_moon_sc::Color;

pub const ISSUE_COST: u64 = 50000000000000000;

#[type_abi]
#[derive(TopEncode, TopDecode)]
pub struct UserInfo {
    pub nft_nonce: u64,
    pub current_harvest_color: Color,
    pub start_timestamp: u64,
}
