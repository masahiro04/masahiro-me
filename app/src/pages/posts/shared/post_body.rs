use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct PostBodyProps {
    pub content: String,
}

// TODO: もしかしたらGlooも使えないかもしれない。。。。
// 下記のコードだとnon wasm環境でhydrate可能なコードが生成されない
// なのでnotな時でも動くようにしたい
// ちなみに画面遷移では正常に動く、リフレッシュ時に壊れる
#[function_component]
pub fn PostBody(props: &PostBodyProps) -> Html {
    #[cfg(target_arch = "wasm32")]
    {
        let div = gloo::utils::document().create_element("div").unwrap();
        div.set_inner_html(props.content.as_str());
        let node = div.into();
        let html = Html::VRef(node);
        html! { html }
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let hoge = props.content.as_str();
        // let html = Html::VRef(hoge);
        let html = Html::from(hoge);
        html! { html }
    }
}
