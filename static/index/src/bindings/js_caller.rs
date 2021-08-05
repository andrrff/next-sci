use wasm_bindgen::prelude::*;

// wasm-bindgen will automatically take care of including this script
#[wasm_bindgen(module = "/public/scripts/script.js")]
extern "C" {
    #[wasm_bindgen(js_name = "calculator")]
    pub fn calculator();
}