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
#[function_component(Image)]
pub fn image(props: &ImageProps) -> Html {
    let ImageProps {
        source,
        class,
        alt,
        height,
        width,
    } = props;
    html! {
        <img
            src={format!("/images/{}", source)}
            class={class.to_string()}
            alt={alt.to_string()}
            height={height.to_string()}
            width={width.to_string()}
        />
    }
}
