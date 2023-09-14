use pages::{route, shared::layout};
use std::collections::HashMap;
use yew::prelude::*;
use yew_router::history::{AnyHistory, History, MemoryHistory};
use yew_router::prelude::*;

#[derive(Properties, PartialEq, Eq, Debug)]
pub struct ServerAppProps {
    pub url: AttrValue,
    pub queries: HashMap<String, String>,
}

#[function_component(ServerApp)]
pub fn server_app(props: &ServerAppProps) -> Html {
    let history = AnyHistory::from(MemoryHistory::new());
    history
        .push_with_query(&*props.url, &props.queries)
        .unwrap();

    html! {
        <Router history={history}>
            <layout::Layout>
                <Switch<route::Route> render={route::switch} />
            </layout::Layout>
        </Router>
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <layout::Layout>
                <Switch<route::Route> render={route::switch} />
            </layout::Layout>
        </BrowserRouter>
    }
}
