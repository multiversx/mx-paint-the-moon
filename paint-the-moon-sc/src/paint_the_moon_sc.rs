#![no_std]
use common::{decode_coordinates, Color, Point, MAX_HEIGHT, MAX_WIDTH};
use multiversx_sc::imports::*;

/// A very light contract containing the map points and their state.
#[multiversx_sc::contract]
pub trait PaintTheMoonSc {
    // endpoints
    #[init]
    fn init(&self) {}

    #[upgrade]
    fn upgrade(&self) {}

    /// Endpoint used to change the state (color) of a point.
    #[payable("*")]
    #[endpoint]
    fn paint(&self, point: u64, new_color: Color) {
        let payment = self.call_value().single_esdt();
        let paint_id = self.paint_id(&new_color).get();

        require!(
            payment.token_identifier == paint_id
                && payment.token_nonce == 0u64
                && payment.amount == BigUint::from(1u64),
            "only one unit of paint can be sent at once"
        );

        // decode point into X, Y, check that both are under height and width
        let (x, y) = decode_coordinates(point);
        require!(
            x <= MAX_WIDTH && y <= MAX_HEIGHT,
            "wrong point coordinates (key)"
        );

        if new_color == Color::White {
            self.color(point).clear();
            self.all_points().swap_remove(&point);
        } else {
            self.color(point).set(new_color);
            self.all_points().insert(point);
        }

        self.splash(point, &new_color);
    }

    // owner endpoints
    /// Endpoint used initially to set up the current stage of the map.
    /// We use a mercator representation of the moon for the initial setup, so the actual moon layout will be passed here.
    #[only_owner]
    #[endpoint]
    fn initial_map_setup(&self, points: ManagedVec<Point>) {
        for point in points.iter() {
            self.all_points().insert(point.coordinates);
            self.color(point.coordinates).set(point.color);
        }
    }

    // storage
    /// Stores the state (color) of a point.
    /// The key is formed by encoding X and Y into an u64.
    #[storage_mapper("color")]
    fn color(&self, point: u64) -> SingleValueMapper<Color>;

    /// We only store the keys that hold information (color). We assume the rest of the points are white.
    /// The key is formed by encoding X and Y into an u64.
    /// Painting in white removes the key from the mapper.
    #[storage_mapper("allPoints")]
    fn all_points(&self) -> UnorderedSetMapper<u64>;

    /// The TokenIdentifier associated with the Paint ESDT.
    #[storage_mapper("paintId")]
    fn paint_id(&self, color: &Color) -> SingleValueMapper<TokenIdentifier>;

    // events
    /// Announces the succesful painting of a point.
    #[event]
    fn splash(&self, #[indexed] point: u64, #[indexed] new_color: &Color);

    // views
    /// Iterates through all available keys and fetches the current state (color).
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
}
