#![no_std]
use data::{Color, Point};
#[allow(unused_imports)]
use multiversx_sc::imports::*;

mod data;

const MAX_HEIGHT: u32 = 500;
const MAX_WIDTH: u32 = 500;

/// A very light contract containing the map points and their state.
#[multiversx_sc::contract]
pub trait PaintTheMoonSc {
    #[init]
    fn init(&self) {}

    #[upgrade]
    fn upgrade(&self) {}

    #[payable("*")]
    #[endpoint]
    fn paint(&self, point: u64, new_color: Color) {
        let payment = self.call_value().single_esdt();
        let paint_id = self.paint_id(&new_color).get();

        require!(
            &payment.token_identifier == &paint_id
                && payment.token_nonce == 0u64
                && &payment.amount == &BigUint::from(1u64),
            "only one unit of paint can be sent at once"
        );

        // decode point into X, Y, check that both are under height and width
        let (x, y) = decode_point(point);
        require!(x <= MAX_WIDTH && y <= MAX_HEIGHT, "wrong point coordinates");

        if &new_color == &Color::White {
            self.color(point).clear();
            self.all_points().swap_remove(&point);
        } else {
            self.color(point).set(new_color);
            self.all_points().insert(point);
        }

        self.splash(point, &new_color);
    }

    #[event]
    fn splash(&self, #[indexed] point: u64, #[indexed] new_color: &Color);

    #[storage_mapper("color")]
    fn color(&self, point: u64) -> SingleValueMapper<Color>;

    #[storage_mapper("allPoints")]
    fn all_points(&self) -> UnorderedSetMapper<u64>;

    #[storage_mapper("paintId")]
    fn paint_id(&self, color: &Color) -> SingleValueMapper<TokenIdentifier>;

    // TODO: find a way to store the points. We will receive a 2D projection of a 3D space.
    // Maybe it would be a good idea to not store them per se, but only their state.
    #[view(getAllPoints)]
    fn get_all_points(&self) -> ManagedVec<Point> {
        let mut vec = ManagedVec::new();
        for point in self.all_points().iter() {
            vec.push(Point {
                coordinates: point,
                color: self.color(point).get(),
            })
        }
        vec
    }

    #[only_owner]
    #[endpoint]
    fn initial_map_setup(&self, points: ManagedVec<Point>) {
        for point in points.iter() {
            self.all_points().insert(point.coordinates);
            self.color(point.coordinates).set(point.color);
        }
    }
}

fn decode_point(encoded: u64) -> (u32, u32) {
    let x = (encoded >> 32) as u32; // Upper 32 bits
    let y = (encoded & 0xFFFFFFFF) as u32; // Lower 32 bits
    (x, y)
}
