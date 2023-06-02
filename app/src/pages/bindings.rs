#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(module = "/js/meta.js")]
extern "C" {
    pub fn updateTitle(str: &str);
    pub fn updateMetaByName(metaName: String, value: &str);
}
