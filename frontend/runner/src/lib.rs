mod domain;
mod infrastructure;
mod pages;
mod presentation;
mod usecase;
use crate::routes::{switch, RootRoutes};
use yew::prelude::*;
use yew_router::prelude::*;
pub mod routes;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<RootRoutes> render={switch} />
        </BrowserRouter>
    }
}
