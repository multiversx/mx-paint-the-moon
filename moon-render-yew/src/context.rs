use crate::requests::{get_all_points, get_config};
use common::{Config, Points};
use common_wasm::ConfigWasm;
use gloo::timers::callback::Interval;
use html::ChildrenProps;
use std::{cell::RefCell, rc::Rc};
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct ConfigContext {
    pub points: Points,
    pub config: Rc<RefCell<ConfigWasm>>,
    pub set_points: Callback<Points>,
    pub set_config: Callback<ConfigWasm>,
}

impl Default for ConfigContext {
    fn default() -> Self {
        ConfigContext {
            points: Points::default(),
            config: Rc::new(RefCell::new(ConfigWasm::new())),
            set_points: Callback::noop(),
            set_config: Callback::noop(),
        }
    }
}

pub async fn refresh_context() -> (Config, Points) {
    log::info!("refreshing context");
    // get config from call to the microservice
    let new_config = get_config().await.unwrap_or_default();

    log::info!("new config in context {new_config:#?}");

    let points = get_all_points(&new_config).await.unwrap_or_default();
    // reconstruct entire map (make rest of the points white)
    // or receive it already reconstructed from the microservice
    (new_config, points)
}

#[function_component(ConfigProvider)]
pub fn config_provider(props: &ChildrenProps) -> Html {
    let points = use_state(Points::default);
    let config = use_state(ConfigWasm::new);

    let set_points = {
        let points = points.clone();
        Callback::from(move |new_points: Points| {
            points.set(new_points);
        })
    };

    let set_config = {
        let config = config.clone();
        Callback::from(move |new_config: ConfigWasm| {
            config.set(new_config);
        })
    };

    // clone the callback for async usage in the effect
    let set_points_effect = set_points.clone();
    let set_config_effect = set_config.clone();

    // refresh context periodically
    use_effect_with_deps(
        move |_| {
            let interval = Interval::new(3_000, move || {
                let set_points_effect = set_points_effect.clone();
                let set_config_effect = set_config_effect.clone();

                wasm_bindgen_futures::spawn_local(async move {
                    let (new_config, new_points) = refresh_context().await;
                    set_points_effect.emit(new_points);
                    set_config_effect.emit(ConfigWasm(new_config));
                });
            });

            // cleanup fn to cancel the interval when component is unmounted
            // drop Closure
            move || {
                interval.cancel();
            }
        },
        (), // run once on mount, and poll every 3 seconds
    );

    let context = ConfigContext {
        points: (*points).clone(),
        config: Rc::new(RefCell::new((*config).clone())),
        set_points,
        set_config,
    };

    html! {
        <ContextProvider<ConfigContext> context={context}>
            { for props.children.iter() }
        </ContextProvider<ConfigContext>>
    }
}
