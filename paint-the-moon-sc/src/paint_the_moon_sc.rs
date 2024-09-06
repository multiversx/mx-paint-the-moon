#![no_std]
use data::Color;
#[allow(unused_imports)]
use multiversx_sc::imports::*;

mod data;

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
    fn paint(&self, point: u8, new_color: Color) {
        let payment = self.call_value().single_esdt();
        let paint_id = self.paint_id(&new_color).get();

        require!(
            &payment.token_identifier == &paint_id
                && payment.token_nonce == 0u64
                && &payment.amount == &BigUint::from(1u64),
            "only one unit of paint can be sent at once"
        );

        self.splash(point, &new_color);
        self.color(point).set(new_color);
    }

    #[event]
    fn splash(&self, #[indexed] point: u8, #[indexed] new_color: &Color);

    #[storage_mapper("color")]
    fn color(&self, point: u8) -> SingleValueMapper<Color>;

    #[storage_mapper("paint_id")]
    fn paint_id(&self, color: &Color) -> SingleValueMapper<TokenIdentifier>;

    // TODO: find a way to store the points. We will receive a 2D projection of a 3D space.
    // Maybe it would be a good idea to not store them per se, but only their state.
}
