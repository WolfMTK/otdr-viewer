use serde::de::DeserializeOwned;
use serde::Serialize;
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

fn to_js_args<A: Serialize>(args: &A) -> JsValue {
    serde_wasm_bindgen::to_value(args).unwrap_or(JsValue::NULL)
}

pub async fn invoke_parsed<T: DeserializeOwned + Default>(cmd: &str) -> T {
    serde_wasm_bindgen::from_value(invoke(cmd, JsValue::NULL).await).unwrap_or_default()
}

pub async fn invoke_parsed_with_args<A: Serialize, T: DeserializeOwned + Default>(cmd: &str, args: &A) -> T {
    serde_wasm_bindgen::from_value(invoke(cmd, to_js_args(args)).await).unwrap_or_default()
}

pub async fn invoke_with_args<A: Serialize>(cmd: &str, args: &A) {
    invoke(cmd, to_js_args(args)).await;
}
