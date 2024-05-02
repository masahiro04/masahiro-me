use yew::prelude::*;
use yew::virtual_dom::VNode;
use yew::{html, AttrValue, Html};

#[derive(Properties, Clone, PartialEq)]
pub struct PostBodyProps {
    pub content: String,
}

// NOTE: 元々glooを使っていたけど、wasm使えないエラーでた
// なのでVNodeでHTMLをstringから生成して表示 -> hydrateできない問題を解消するためにhookを利用
#[function_component(PostBody)]
pub fn post_body(props: &PostBodyProps) -> Html {
    let PostBodyProps { content } = props;
    let node: UseStateHandle<Option<VNode>> = use_state(|| None);
    let content_clone = content.clone();
    let set_node = node.clone();
    use_effect_with(props.clone(), move |_| {
        let parsed = Html::from_html_unchecked(AttrValue::from(content_clone.clone()));
        set_node.set(Some(parsed));
        || ()
    });
    if let Some(node) = &*node {
        node.clone()
    } else {
        html! { <div></div> }
    }
}
