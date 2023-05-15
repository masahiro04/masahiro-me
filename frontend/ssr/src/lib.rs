use app::app::App;
use wasm_bindgen::prelude::*;
// use wasm_bindgen_futures;
// use yew;
use web_sys::Window;

#[macro_export]
macro_rules! console_log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[wasm_bindgen]
pub async fn render(html_string: String) -> Result<JsValue, JsValue> {
    console_log!("html_string: {}", html_string);
    let (index_html_before, index_html_after) = html_string.split_once("<body>").unwrap();
    let mut index_html_before = index_html_before.to_owned();
    index_html_before.push_str("<body>");
    let index_html_after = index_html_after.to_owned();
    console_log!("sentinel1");

    let mut str = String::new();
    let renderer = yew::ServerRenderer::<App>::new();
    console_log!("sentinel2");
    // let rendered_text = &renderer.render().await;
    renderer.render_to_string(&mut str).await;
    console_log!("sentinel3");
    let body = index_html_before + &str.clone() + &index_html_after;
    console_log!("sentinel4");
    Ok(JsValue::from_str(&body))
}

// type BoxedError = Box<dyn Error + Send + Sync + 'static>;
//
// #[wasm_bindgen]
// pub async fn render(
//     html_string: String,
// ) -> Box<dyn Stream<Item = Result<Bytes, BoxedError>> + Send> {
//     let (index_html_before, index_html_after) = html_string.split_once("<body>").unwrap();
//     let mut index_html_before = index_html_before.to_owned();
//     index_html_before.push_str("<body>");
//     let index_html_after = index_html_after.to_owned();
//     let renderer = yew::ServerRenderer::<App>::new();
//
//     Box::new(
//         stream::once(async move { index_html_before })
//             .chain(renderer.render_stream())
//             .chain(stream::once(async move { index_html_after }))
//             .map(|m| Result::<_, BoxedError>::Ok(m.into())),
//     )
// }
