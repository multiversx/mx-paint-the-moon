use crate::requests::query;
use common::Point;
use html::ChildrenProps;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct ConfigContext {
    pub points: Vec<Point>,
    pub set_points: Callback<Vec<Point>>,
}

impl Default for ConfigContext {
    fn default() -> Self {
        ConfigContext {
            points: Vec::new(),
            set_points: Callback::noop(),
        }
    }
}

pub async fn refresh_context() -> Vec<Point> {
    log::info!("refreshing context");
    query::get_all_points().await.unwrap_or_default()
}

#[function_component(ConfigProvider)]
pub fn config_provider(props: &ChildrenProps) -> Html {
    let points = use_state(Vec::new);

    let set_points = {
        let points = points.clone();
        Callback::from(move |new_points: Vec<Point>| {
            points.set(new_points);
        })
    };

    // clone the callback for async usage in the effect
    let set_points_async = set_points.clone();

    // refresh context on component mount
    use_effect_with_deps(
        move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let new_points = refresh_context().await;
                // let new_points = Vec::new();

                // Emit the new status inside the async block
                set_points_async.emit(new_points);
            });
            || () // no cleanup fn
        },
        (), // empty dependency array, run once on mount
    );

    let context = ConfigContext {
        points: (*points).clone(),
        set_points,
    };

    html! {
        <ContextProvider<ConfigContext> context={context}>
            { for props.children.iter() }
        </ContextProvider<ConfigContext>>
    }
}
