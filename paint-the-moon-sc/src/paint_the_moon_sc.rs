#![no_std]
// use color::Color;
#[allow(unused_imports)]
use multiversx_sc::imports::*;

mod color;
pub mod pixel_block;

#[cfg(feature = "block-size-8")]
pub type Block = pixel_block::PixelBlock<pixel_block::PixelBlockData8>;

#[cfg(feature = "block-size-16")]
pub type Block = pixel_block::PixelBlock<pixel_block::PixelBlockData16>;

#[cfg(feature = "block-size-32")]
pub type Block = pixel_block::PixelBlock<pixel_block::PixelBlockData32>;

#[cfg(feature = "block-size-64")]
pub type Block = pixel_block::PixelBlock<pixel_block::PixelBlockData64>;

// const MAP_SIZE_BITS_X: u32 = MAP_SIZE_BITS_Y + 1; // ratio is always 2:1
// const MAP_SIZE_BITS_Y: u32 = 9;

// const MAP_SIZE_PIXELS_X: u32 = 1 << MAP_SIZE_BITS_X; // 1024
// const MAP_SIZE_PIXELS_Y: u32 = 1 << MAP_SIZE_BITS_Y; // 512



/// A very light contract containing the map points and their state.
#[multiversx_sc::contract]
pub trait PaintTheMoonSc {
    #[init]
    fn init(&self) {}

    #[upgrade]
    fn upgrade(&self) {}

    // can paint every point white at the beginning
    #[payable("*")]
    #[endpoint]
    fn paint(&self, x: i32, y: i32, new_color: u8) {
        // let payment = self.call_value().single_esdt();
        // let paint_id = self.paint_id(&new_color).get();

        // require!(
        //     &payment.token_identifier == &paint_id
        //         && payment.token_nonce == 0u64
        //         && &payment.amount == &BigUint::from(1u64),
        //     "only one unit of paint can be sent at once"
        // );

        let (block_x, sub_x) = Block::split_coord(x);
        let (block_y, sub_y) = Block::split_coord(y);

        let raw_block_mapper = self.raw_blocks(block_x, block_y);
        let raw_block = raw_block_mapper.get();
        let mut block = Block::from_managed_buffer(&raw_block);
        block.set_raw_pixel(sub_x, sub_y, new_color);
        let raw_block = block.to_managed_buffer();
        self.block_changed(block_x, block_y, &raw_block);
        raw_block_mapper.set(raw_block);


        // self.splash(point, &new_color);
        // self.color(point).set(new_color);
    }

    #[event("blockChanged")]
    fn block_changed(&self, #[indexed] block_x: i32, #[indexed] block_y: i32, raw_block: &ManagedBuffer);

    #[storage_mapper("blocks")]
    fn raw_blocks(&self, block_x: i32, block_y: i32) -> SingleValueMapper<ManagedBuffer>;

    // #[storage_mapper("paint_id")]
    // fn paint_id(&self, color: &Color) -> SingleValueMapper<TokenIdentifier>;

    // TODO: find a way to store the points. We will receive a 2D projection of a 3D space.
    // Maybe it would be a good idea to not store them per se, but only their state.
}
