use common::Color;
use std::rc::Rc;
use yew::prelude::*;

use crate::{
    components::Button,
    context::ConfigContext,
    requests::{setup, transaction},
};

#[function_component(AdminPanel)]
pub fn admin_panel() -> Html {
    let context = use_context::<ConfigContext>().expect("Failed to get config context");
    let deploy_response = use_state(String::new);
    let paint_response = use_state(String::new);
    let show_points = use_state(|| false);

    let deploy_ptm_service = {
        let deploy_response = deploy_response.clone();

        Callback::from(move |_| {
            let deploy_response = deploy_response.clone();
            let config = Rc::clone(&context.config);

            log::info!("SC setup request triggered");

            wasm_bindgen_futures::spawn_local(async move {
                let config = config.borrow().clone();
                match setup::deploy_paint_the_moon(&config).await {
                    Ok(result) => {
                        deploy_response.set(format!(
                            "New deployed address: {}",
                            result.to_bech32_string()
                        ));
                    }
                    Err(err) => {
                        log::error!("SC Setup failed: {:?}", err);
                        deploy_response.set("SC Setup failed!".to_string());
                    }
                }
            });
        })
    };

    let paint_service = {
        let paint_response = paint_response.clone();

        Callback::from(move |_| {
            let paint_response = paint_response.clone();

            log::info!("Paint transaction triggered");

            wasm_bindgen_futures::spawn_local(async move {
                let mock_point = 5u64;
                let mock_color = Color::Red;

                match transaction::paint(mock_point, mock_color).await {
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

    let fetch_points_service = {
        let show_points = show_points.clone();
        Callback::from(move |_| {
            log::info!("Points fetch from the context triggered");
            show_points.set(true)
        })
    };

    html! {
        <div class = "admin">
        <h2>{"Paint The Moon Admin Panel"}</h2>
        <div class = "admin-panel-btns">
                <Button name="Fetch points" class_name="query-btn" button_type = "button" on_click={fetch_points_service.clone()} />
                <Button name = "Paint" class_name = "transaction-btn" button_type = "button" on_click={paint_service.clone()} />
                <Button name = "Deploy PTM" class_name = "transaction-btn" button_type = "button" on_click={deploy_ptm_service.clone()} />
        </div>
        {
            if *show_points {
                html! {
                    <>
                        <p>{ context.points.0.len() }</p>
                    </>
                }
            }
            else {
                html! {
                    <>
                    </>
                }
            }
        }
            {
                if !(*paint_response).is_empty() {
                    html! {
                        <>
                            <p>{ (*paint_response).clone() }</p>
                        </>
                    }
                }
                else {
                    html! {
                        <>
                        </>
                    }
                }
            }
            {
                if !(*deploy_response).is_empty() {
                    html! {
                        <>
                            <p>{ (*deploy_response).clone() }</p>
                        </>
                    }
                }
                else {
                    html! {
                        <>
                        </>
                    }
                }
            }
        </div>
    }
}
