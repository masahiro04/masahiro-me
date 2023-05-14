use app::app::App;
// use clap::Parser;
// use std::fs;
// use std::path::PathBuf;
use wasm_bindgen::prelude::*;
use yew;

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[macro_export]
macro_rules! console_log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[wasm_bindgen]
pub async fn render(html_string: String) -> String {
    console_log!("html_string: {}", html_string);
    console_log!("sentinel1");
    let (index_html_before, index_html_after) = html_string.split_once("<body>").unwrap();
    console_log!("sentinel2");
    let mut index_html_before = index_html_before.to_owned();
    console_log!("sentinel3");
    index_html_before.push_str("<body>");
    console_log!("sentinel4");
    let index_html_after = index_html_after.to_owned();
    console_log!("sentinel5");

    let renderer = yew::ServerRenderer::<App>::new();
    console_log!("sentinel6");
    let body = index_html_before + &renderer.render().await + &index_html_after;
    console_log!("sentinel7");
    body
}
