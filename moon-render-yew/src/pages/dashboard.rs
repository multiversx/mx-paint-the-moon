use common::{Color, Point};
use yew::prelude::*;

use crate::{
    components::{Button, Map},
    requests::paint,
};

#[function_component(Dashboard)]
pub fn dashboard() -> Html {
    let paint_response = use_state(String::new);

    let paint_service = {
        let paint_response = paint_response.clone();

        Callback::from(move |_| {
            let paint_response = paint_response.clone();

            log::info!("Paint transaction triggered");

            wasm_bindgen_futures::spawn_local(async move {
                let mock_point = Point {
                    x: 5u32,
                    y: 400u32,
                    color: Color::Green,
                };

                match paint(mock_point).await {
                    Ok(result) => {
                        paint_response.set(result);
                    }
                    Err(err) => {
                        log::error!("Paint transaction failed: {:?}", err);
                        paint_response.set("Paint transaction failed!".to_string());
                    }
                }
            });
        })
    };

    html! {
        <>
        <div class = "admin">
            <h2>
            { "Dashboard" }
            </h2>
            <Map />
            <Button name = "Paint" class_name = "transaction-btn" button_type = "button" on_click={paint_service.clone()} />
        </div>
        </>
    }
}
