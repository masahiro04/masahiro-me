use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct ImageProps {
    pub source: String,
    #[prop_or(String::new())]
    pub class: String,
    #[prop_or(String::new())]
    pub alt: String,
    #[prop_or(0)]
    pub height: i32,
    #[prop_or(0)]
    pub width: i32,
}
#[function_component]
pub fn Image(props: &ImageProps) -> Html {
    html! {
        <img src={props.source.clone()} class={props.class.clone()} alt={props.alt.clone()} height={props.height.to_string()} width={props.width.to_string()} />
    }
}
