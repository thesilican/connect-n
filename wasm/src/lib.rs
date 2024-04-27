use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(message: &str);
}

#[wasm_bindgen]
pub fn welcome() {
    log("Hello from wasm!");
}
