use gloo::utils::document;
// use js_sys::Object;
// use std::rc::Rc;
// use web_sys::{Element, Node};
// use wasm_bindgen::JsCast;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct PostBodyProps {
    pub content: String,
}

#[function_component]
pub fn PostBody(props: &PostBodyProps) -> Html {
    let div = document().create_element("div").unwrap();
    div.set_inner_html(props.content.as_str());
    let node = div.into();
    let html = Html::VRef(node);
    html! { {html} }
}
