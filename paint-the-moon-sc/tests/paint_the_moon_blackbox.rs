use common::PaintTheMoonScProxy as proxy;
use common::{Color, Point};
use multiversx_sc::types::{TestAddress, TestSCAddress};
use multiversx_sc_scenario::{imports::*, ScenarioWorld};
const CODE_PATH: MxscPath = MxscPath::new("mxsc:output/paint-the-moon-sc.wasm");
const OWNER: TestAddress = TestAddress::new("OWNER");
const SC_ADDRESS: TestSCAddress = TestSCAddress::new("PAINT_THE_MOON_SC");

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

        world.account(OWNER).nonce(1);

        Self { world }
    }

    fn deploy_contract(&mut self) -> &mut Self {
        self.world
            .tx()
            .from(OWNER)
            .typed(proxy)
            .init()
            .code(CODE_PATH)
            .new_address(SC_ADDRESS)
            .run();
        self
    }

    fn initial_map_setup(&mut self) {
        let mut points = ManagedVec::new();

        points.extend((0..2800).flat_map(|x| {
            (0..2800).map(move |y| Point {
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

    fn get_all_points(&mut self) {
        self
            .world
            .query()
            .to(SC_ADDRESS)
            .typed(proxy)
            .get_all_points()
            .returns(ReturnsResultUnmanaged)
            .run();
    }
}

#[test]
fn paint_the_moon_blackbox() {
    let mut state = PaintTheMoonTestState::new();
    state.deploy_contract();
    state.initial_map_setup();
    state.get_all_points();
}
