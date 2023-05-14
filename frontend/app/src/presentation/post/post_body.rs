use gloo::utils::document;
use web_sys::{Element, Node};
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct PostBodyProps {
    pub content: String,
}

#[function_component(PostBody)]
pub fn post_body(props: &PostBodyProps) -> Html {
    let div: Element = document().create_element("div").unwrap();
    div.set_inner_html(props.content.as_str());
    let node: Node = div.into();
    let html = Html::VRef(node);
    html! { {html} }
}
