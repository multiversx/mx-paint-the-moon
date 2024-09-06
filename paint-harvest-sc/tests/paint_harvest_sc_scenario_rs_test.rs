use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    blockchain.register_contract("mxsc:output/paint-harvest-sc.mxsc.json", paint_harvest_sc::ContractBuilder);
    blockchain
}

#[test]
fn empty_rs() {
    world().run("scenarios/paint_harvest_sc.scen.json");
}
