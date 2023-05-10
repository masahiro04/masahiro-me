use app::app::App;
use yew;

use std::error::Error;
use std::fs;
use std::path::PathBuf;

use bytes::Bytes;
use clap::Parser;
use futures::stream::{self, Stream, StreamExt};
// use js_sys::Promise;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::{future_to_promise, JsFuture};

// use web_sys;
// use simple_ssr::App;

// use warp::Filter;

// use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

type BoxedError = Box<dyn Error + Send + Sync + 'static>;

/// A basic example
#[derive(Parser, Debug)]
struct Opt {
    /// the "dist" created by trunk directory to be served for hydration.
    #[structopt(short, long, parse(from_os_str))]
    dir: PathBuf,
}

// TODO: 小手先の修正試みたけど、コードを全部理解した方が良さそうだし、多分早い

async fn render(
    index_html_before: String,
    index_html_after: String,
) -> Box<dyn Stream<Item = Result<Bytes, BoxedError>> + Send> {
    let renderer = yew::ServerRenderer::<App>::new();
    // let s = index_html_before + &renderer.render_to_string() + &index_html_after;
    // renderer.render_to_string()

    let b = Box::new(
        stream::once(async move { index_html_before })
            .chain(renderer.render_stream())
            .chain(stream::once(async move { index_html_after }))
            .map(|m| Result::<_, BoxedError>::Ok(m.into())),
    );
    b
}

#[async_std::main]
// #[wasm_bindgen]
async fn main() {
    let opts = Opt::parse();
    let index_html_s =
        fs::read_to_string(opts.dir.join("index.html")).expect("failed to read index.html");
    let (index_html_before, index_html_after) = index_html_s.split_once("<body>").unwrap();
    let mut index_html_before = index_html_before.to_owned();
    index_html_before.push_str("<body>");
    let index_html_after = index_html_after.to_owned();

    let resp = render(index_html_before, index_html_after).await;
    match resp {
        Ok(resp) => resp,
        Err(e) => String::from("Error"),
    };
}

async fn my_async_function(input: JsValue) -> Result<JsValue, JsValue> {
    // ここで非同期処理を行います。
    // この例では、入力値に 1 を追加します。
    let number: f64 = input.as_f64().unwrap();
    let result = number + 1.0;
    Ok(JsValue::from(result))
}

#[wasm_bindgen]
pub fn call_my_async_function(input: JsValue) -> Promise {
    let future = my_async_function(input);
    future_to_promise(future)
}
