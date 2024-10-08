use yew::prelude::*;

use crate::components::Map;

#[function_component(Dashboard)]
pub fn dashboard() -> Html {
    html! {
        <>
            <h2>
            { "Dashboard" }
            </h2>
            <Map />
        </>
    }
}
