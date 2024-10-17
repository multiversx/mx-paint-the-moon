#![no_std]
use common::{Color, Coordinates, Point, MAX_HEIGHT, MAX_WIDTH};
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
    /// We pass a point containing the coordinates and new color.
    #[payable("*")]
    #[endpoint]
    fn paint(&self, point: Point) {
        // x, y, new color
        let payment = self.call_value().single_esdt();
        let (x, y, new_color) = point.into_tuple();
        let paint_id = self.paint_id(&new_color).get();

        require!(
            payment.token_identifier == paint_id
                && payment.token_nonce == 0u64
                && payment.amount == 1u64,
            "only one unit of paint can be sent at once"
        );

        // decode point into X, Y, check that both are under height and width
        require!(
            x <= MAX_WIDTH && y <= MAX_HEIGHT,
            "wrong point coordinates (key)"
        );

        let position = Coordinates(x, y);

        if new_color == Color::White {
            self.color(position).clear();
            self.all_points().swap_remove(&position);
        } else {
            self.color(position).set(new_color);
            self.all_points().insert(position);
        }

        self.splash(position, &new_color);
    }

    // owner endpoints
    /// Endpoint used initially to set up the current stage of the map.
    /// We use a mercator representation of the moon for the initial setup, so the actual moon layout will be passed here.
    #[only_owner]
    #[endpoint]
    fn initial_map_setup(&self, points: ManagedVec<Point>) {
        for point in points.iter() {
            self.all_points().insert(point.coordinates());
            self.color(point.coordinates()).set(point.color);
        }
    }

    // storage
    /// Stores the state (color) of a point.
    #[storage_mapper("color")]
    fn color(&self, position: Coordinates) -> SingleValueMapper<Color>;

    /// We only store the keys that hold information (color). We assume the rest of the points are white.
    /// Painting in white removes the key from the mapper.
    #[storage_mapper("allPoints")]
    fn all_points(&self) -> UnorderedSetMapper<Coordinates>;

    /// The TokenIdentifier associated with the Paint ESDT.
    #[storage_mapper("paintId")]
    fn paint_id(&self, color: &Color) -> SingleValueMapper<TokenIdentifier>;

    // events
    /// Announces the succesful painting of a point.
    #[event]
    fn splash(&self, #[indexed] point_position: Coordinates, #[indexed] new_color: &Color);

    // views
    /// Iterates through all available keys and fetches the current state (color).
    #[view(getAllPoints)]
    fn get_all_points(&self) -> ManagedVec<Point> {
        let mut vec = ManagedVec::new();
        for coords in self.all_points().iter() {
            vec.push(Point {
                x: coords.0,
                y: coords.1,
                color: self.color(coords).get(),
            })
        }
        vec
    }
}
