use crate::requests::query;
use common::{Config, Points};
use html::ChildrenProps;
use std::{cell::RefCell, rc::Rc};
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct ConfigContext {
    pub points: Points,
    pub config: Rc<RefCell<Config>>,
    pub set_points: Callback<Points>,
    pub set_config: Callback<Config>,
}

impl Default for ConfigContext {
    fn default() -> Self {
        ConfigContext {
            points: Points::default(),
            config: Rc::new(RefCell::new(Config::new())),
            set_points: Callback::noop(),
            set_config: Callback::noop(),
        }
    }
}

pub async fn refresh_context() -> (Config, Points) {
    log::info!("refreshing context");
    // get config from call to the microservice
    let new_config = query::get_config().await.unwrap_or_default();

    let points = query::get_all_points(&new_config).await.unwrap_or_default();
    // reconstruct entire map (make rest of the points white)
    // or receive it already reconstructed from the microservice
    (new_config, points)
}

#[function_component(ConfigProvider)]
pub fn config_provider(props: &ChildrenProps) -> Html {
    let points = use_state(Points::default);
    let config = use_state(Config::new);

    let set_points = {
        let points = points.clone();
        Callback::from(move |new_points: Points| {
            points.set(new_points);
        })
    };

    let set_config = {
        let config = config.clone();
        Callback::from(move |new_config: Config| {
            config.set(new_config);
        })
    };

    // clone the callback for async usage in the effect
    let set_points_effect = set_points.clone();
    let set_config_effect = set_config.clone();

    // refresh context on component mount
    use_effect_with_deps(
        move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let (new_config, new_points) = refresh_context().await;

                // Emit the new status inside the async block
                set_points_effect.emit(new_points);
                set_config_effect.emit(new_config);
            });
            || () // no cleanup fn
        },
        (), // empty dependency array, run once on mount
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
