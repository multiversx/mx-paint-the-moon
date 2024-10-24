use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::Dashboard;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/harvest")]
    HarvestCenter,
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <Dashboard /> },
        Route::HarvestCenter => html! {},
    }
}
