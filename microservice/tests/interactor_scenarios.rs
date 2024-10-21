use common::{Color, Point};
use interactor::ContractInteract;
use multiversx_sc_snippets::imports::TestTokenIdentifier;
use multiversx_sc_snippets::{
    imports::{BigUint, EsdtTokenPayment, MultiValueEncoded, StaticApi},
    tokio,
};

const GREEN_TOKEN_ID: TestTokenIdentifier = TestTokenIdentifier::new("GREEN-0e161c");

#[tokio::test]
async fn paint_scenario() {
    let mut interactor = ContractInteract::new().await;
    let mut setup = MultiValueEncoded::new();
    setup.push((GREEN_TOKEN_ID.to_token_identifier(), Color::Green));

    let payment = EsdtTokenPayment::<StaticApi>::from((
        GREEN_TOKEN_ID.to_token_identifier(),
        0,
        BigUint::from(1u64),
    ));
    let point = Point {
        x: 5u32,
        y: 5u32,
        color: Color::Green,
    };

    let _ = interactor.deploy_paint_the_moon(setup).await;
    let result = interactor.paint(point, payment).await;
    println!("paint result: {:#?}", result);
}
