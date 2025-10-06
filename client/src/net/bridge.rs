use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/js/spacetime_bridge.js")]
extern "C" {
    #[wasm_bindgen(js_name = "connect")]
    pub async fn js_connect(uri: &str, module: &str, token: Option<String>);

    #[wasm_bindgen(js_name = "callReducer")]
    pub async fn js_call_reducer(name: &str, payload: &[u8]);

    #[wasm_bindgen(js_name = "subscribe")]
    pub fn js_subscribe(query: &str);
}
