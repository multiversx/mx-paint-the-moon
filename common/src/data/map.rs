use multiversx_sc::derive_imports::*;
use serde::{Deserialize, Serialize};

pub const ISSUE_COST: u64 = 50000000000000000;

pub const MAX_HEIGHT: u32 = 500;
pub const MAX_WIDTH: u32 = 500;

#[type_abi]
#[derive(
    Serialize,
    Deserialize,
    TopEncode,
    TopDecode,
    NestedEncode,
    NestedDecode,
    Copy,
    Clone,
    PartialEq,
    ManagedVecItem,
    Debug,
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
    Serialize,
    Deserialize,
    TopEncode,
    TopDecode,
    NestedEncode,
    NestedDecode,
    ManagedVecItem,
    Copy,
    Clone,
    Debug,
    PartialEq,
)]
pub struct Point {
    pub x: u32,
    pub y: u32,
    pub color: Color,
}

impl Point {
    pub fn into_tuple(self) -> (u32, u32, Color) {
        (self.x, self.y, self.color)
    }

    pub fn coordinates(&self) -> Coordinates {
        Coordinates(self.x, self.y)
    }
}

#[type_abi]
#[derive(TopEncode, TopDecode)]
pub struct UserInfo {
    pub nft_nonce: u64,
    pub current_harvest_color: Color,
    pub start_timestamp: u64,
}

#[type_abi]
#[derive(
    TopEncode,
    TopDecode,
    NestedEncode,
    NestedDecode,
    Clone,
    Copy,
    Debug,
    Serialize,
    Deserialize,
    PartialEq,
)]
pub struct Coordinates(pub u32, pub u32);
