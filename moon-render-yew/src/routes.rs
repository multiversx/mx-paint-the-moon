use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::{AdminPage, Dashboard};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/admin")]
    Admin,
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <Dashboard /> },
        Route::Admin => html! { <AdminPage /> },
    }
}
