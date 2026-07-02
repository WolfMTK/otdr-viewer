use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"], js_name = invoke)]
    pub async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

pub fn invoke_fire_and_forget(cmd: &'static str) {
    wasm_bindgen_futures::spawn_local(async move {
        invoke(cmd, JsValue::NULL).await;
    })
}
