use common::PaintTheMoonScProxy as proxy;
use common::{Color, Point, MAX_HEIGHT, MAX_WIDTH};
use multiversx_sc::types::{TestAddress, TestSCAddress};
use multiversx_sc_scenario::{imports::*, ScenarioWorld};
const CODE_PATH: MxscPath = MxscPath::new("mxsc:output/paint-the-moon-sc.wasm");
const OWNER: TestAddress = TestAddress::new("OWNER");
const SC_ADDRESS: TestSCAddress = TestSCAddress::new("PAINT_THE_MOON_SC");
const TEST_TOKEN: TestTokenIdentifier = TestTokenIdentifier::new("TEST_TOKEN");
const WRONG_TOKEN: TestTokenIdentifier = TestTokenIdentifier::new("WRONG_TOKEN");
const NFT_TOKEN: TestTokenIdentifier = TestTokenIdentifier::new("NFT_TOKEN");

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.register_contract(CODE_PATH, paint_the_moon_sc::ContractBuilder);
    blockchain
}

struct PaintTheMoonTestState {
    world: ScenarioWorld,
}

impl PaintTheMoonTestState {
    fn new() -> Self {
        let mut world = world();

        world
            .account(OWNER)
            .nonce(1)
            .esdt_balance(TEST_TOKEN, 1000)
            .esdt_balance(WRONG_TOKEN, 1000)
            .esdt_nft_balance(NFT_TOKEN, 1, 10, ManagedBuffer::new());

        world.start_trace();
        Self { world }
    }

    fn deploy_contract(
        &mut self,
        setup: MultiValueEncoded<StaticApi, (TokenIdentifier<StaticApi>, Color)>,
    ) -> &mut Self {
        self.world
            .tx()
            .from(OWNER)
            .typed(proxy)
            .init(setup)
            .code(CODE_PATH)
            .new_address(SC_ADDRESS)
            .run();
        self
    }

    fn initial_map_setup(&mut self) {
        let mut points = ManagedVec::new();

        points.extend((0..500).flat_map(|x| {
            (0..500).map(move |y| Point {
                x,
                y,
                color: Color::Red,
            })
        }));

        self.world
            .tx()
            .from(OWNER)
            .to(SC_ADDRESS)
            .typed(proxy)
            .initial_map_setup(points)
            .run();
    }

    fn paint_error(
        &mut self,
        point: Point,
        payment: EsdtTokenPayment<StaticApi>,
        error: ExpectError,
    ) {
        self.world
            .tx()
            .from(OWNER)
            .to(SC_ADDRESS)
            .gas(5_000_000u64)
            .typed(proxy)
            .paint(point)
            .payment(payment)
            .returns(error)
            .run();
    }

    fn paint(&mut self, point: Point, payment: EsdtTokenPayment<StaticApi>) {
        self.world
            .tx()
            .from(OWNER)
            .to(SC_ADDRESS)
            .gas(5_000_000u64)
            .typed(proxy)
            .paint(point)
            .payment(payment)
            .returns(ReturnsResultUnmanaged)
            .run();
    }

    fn get_all_points(&mut self) {
        let result = self
            .world
            .query()
            .to(SC_ADDRESS)
            .typed(proxy)
            .get_all_points()
            .returns(ReturnsResultUnmanaged)
            .run();
        println!("{:?}", result);
    }

    fn write_scenario_trace(&mut self, path: &str) {
        self.world.write_scenario_trace(path);
    }
}

#[test]
fn test_max_size_moon_blackbox() {
    let mut state = PaintTheMoonTestState::new();
    let mut setup = MultiValueEncoded::new();
    setup.push((TEST_TOKEN.to_token_identifier(), Color::Red));
    state.deploy_contract(setup);
    state.initial_map_setup();
    state.get_all_points();
    state.write_scenario_trace("scenarios/test_max_size_moon_blackbox.scen.json");
}

#[test]
fn test_paint_failure_wrong_token_blackbox() {
    let mut state = PaintTheMoonTestState::new();
    let mut setup = MultiValueEncoded::new();
    setup.push((TEST_TOKEN.to_token_identifier(), Color::Red));
    state.deploy_contract(setup);
    state.initial_map_setup();
    state.paint_error(
        Point {
            x: 0,
            y: 0,
            color: Color::Red,
        },
        EsdtTokenPayment::new(WRONG_TOKEN.into(), 0u64, BigUint::from(1u64)),
        ExpectError(4, "only one unit of paint can be sent at once"),
    );
    state.write_scenario_trace("scenarios/test_paint_failure_wrong_token_blackbox.scen.json");
}

#[test]
fn test_paint_failure_no_token_color_blackbox() {
    let mut state = PaintTheMoonTestState::new();
    let mut setup = MultiValueEncoded::new();
    setup.push((TEST_TOKEN.to_token_identifier(), Color::Red));
    state.deploy_contract(setup);
    state.initial_map_setup();
    state.paint_error(
        Point {
            x: 0,
            y: 0,
            color: Color::Green,
        },
        EsdtTokenPayment::new(TEST_TOKEN.into(), 0u64, BigUint::from(1u64)),
        ExpectError(4, "only one unit of paint can be sent at once"),
    );
    state.write_scenario_trace("scenarios/test_paint_failure_no_token_color_blackbox.scen.json");
}

#[test]
fn test_paint_failure_wrong_nonce_blackbox() {
    let mut state = PaintTheMoonTestState::new();
    let mut setup = MultiValueEncoded::new();
    setup.push((NFT_TOKEN.to_token_identifier(), Color::Red));
    state.deploy_contract(setup);
    state.initial_map_setup();
    state.paint_error(
        Point {
            x: 0,
            y: 0,
            color: Color::Red,
        },
        EsdtTokenPayment::new(NFT_TOKEN.into(), 1u64, BigUint::from(1u64)),
        ExpectError(4, "only one unit of paint can be sent at once"),
    );
    state.write_scenario_trace("scenarios/test_paint_failure_wrong_nonce_blackbox.scen.json");
}

#[test]
fn test_paint_failure_wrong_amount_blackbox() {
    let mut state = PaintTheMoonTestState::new();
    let mut setup = MultiValueEncoded::new();
    setup.push((TEST_TOKEN.to_token_identifier(), Color::Red));
    state.deploy_contract(setup);
    state.initial_map_setup();
    state.paint_error(
        Point {
            x: 0,
            y: 0,
            color: Color::Red,
        },
        EsdtTokenPayment::new(TEST_TOKEN.into(), 0u64, BigUint::from(10u64)),
        ExpectError(4, "only one unit of paint can be sent at once"),
    );
    state.write_scenario_trace("scenarios/test_paint_failure_wrong_amount_blackbox.scen.json");
}

#[test]
fn test_paint_failure_wrong_coords_blackbox() {
    let mut state = PaintTheMoonTestState::new();
    let mut setup = MultiValueEncoded::new();
    setup.push((TEST_TOKEN.to_token_identifier(), Color::Red));
    state.deploy_contract(setup);
    state.initial_map_setup();
    state.paint_error(
        Point {
            x: MAX_HEIGHT + 1,
            y: MAX_WIDTH + 1,
            color: Color::Red,
        },
        EsdtTokenPayment::new(TEST_TOKEN.into(), 0u64, BigUint::from(1u64)),
        ExpectError(4, "wrong point coordinates (key)"),
    );
    state.write_scenario_trace("scenarios/test_paint_failure_wrong_coords_blackbox.scen.json");
}

#[test]
fn test_paint_the_same_pixel_different_colors_blackbox() {
    let mut state = PaintTheMoonTestState::new();
    let mut setup = MultiValueEncoded::new();
    setup.push((TEST_TOKEN.to_token_identifier(), Color::Blue));
    setup.push((TEST_TOKEN.to_token_identifier(), Color::Green));
    state.deploy_contract(setup);
    state.initial_map_setup();
    state.paint(
        Point {
            x: 499,
            y: 499,
            color: Color::Green,
        },
        EsdtTokenPayment::new(TEST_TOKEN.into(), 0u64, BigUint::from(1u64)),
    );
    state.paint(
        Point {
            x: 499,
            y: 499,
            color: Color::Blue,
        },
        EsdtTokenPayment::new(TEST_TOKEN.into(), 0u64, BigUint::from(1u64)),
    );
    state.get_all_points();
    state.write_scenario_trace("scenarios/test_paint_the_same_pixel_different_colors_blackbox.scen.json");
}
