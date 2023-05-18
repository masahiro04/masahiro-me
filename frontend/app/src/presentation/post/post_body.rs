// #[cfg(target_arch = "wasm32")]
// use gloo::utils::document;
use web_sys::Document;
use web_sys::{Element, Node};
use yew::prelude::*;

use crate::console_log;

#[derive(Properties, Clone, PartialEq)]
pub struct PostBodyProps {
    pub content: String,
}

#[function_component(PostBody)]
pub fn post_body(props: &PostBodyProps) -> Html {
    let document = Document::new().unwrap();
    let div: Element = document.create_element("div").unwrap();

    console_log!("sentinel1");
    div.set_outer_html(&props.content.as_str());
    console_log!("sentinel2");
    let node: Node = div.into();
    let html = Html::VRef(node);
    console_log!("sentinel3");
    html! { {html} }
}
