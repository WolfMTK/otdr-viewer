use serde::de::DeserializeOwned;
use serde::Serialize;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"], js_name = invoke)]
    pub async fn invoke(cmd: &str, args: JsValue) -> JsValue;

    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"], js_name = invoke, catch)]
    async fn invoke_catch(cmd: &str, args: JsValue) -> Result<JsValue, JsValue>;
}

pub fn invoke_fire_and_forget(cmd: &'static str) {
    wasm_bindgen_futures::spawn_local(async move {
        invoke(cmd, JsValue::NULL).await;
    })
}

fn to_js_args<A: Serialize>(args: &A) -> JsValue {
    serde_wasm_bindgen::to_value(args).unwrap_or(JsValue::NULL)
}

fn js_error_to_string(err: JsValue) -> String {
    err.as_string()
        .unwrap_or_else(|| format!("Неизвестная ошибка команды: {err:?}"))
}

pub async fn try_invoke_parsed<T: DeserializeOwned>(cmd: &str) -> Result<T, String> {
    let value = invoke_catch(cmd, JsValue::NULL).await.map_err(js_error_to_string)?;
    serde_wasm_bindgen::from_value(value).map_err(|e| format!("Не удалось разобрать ответ команды {cmd}: {e}"))
}

pub async fn try_invoke_parsed_with_args<A: Serialize, T: DeserializeOwned>(cmd: &str, args: &A) -> Result<T, String> {
    let value = invoke_catch(cmd, to_js_args(args)).await.map_err(js_error_to_string)?;
    serde_wasm_bindgen::from_value(value).map_err(|e| format!("Не удалось разобрать ответ команды {cmd}: {e}"))
}

pub async fn try_invoke_with_args<A: Serialize>(cmd: &str, args: &A) -> Result<(), String> {
    invoke_catch(cmd, to_js_args(args)).await.map_err(js_error_to_string)?;
    Ok(())
}

pub async fn try_invoke(cmd: &str) -> Result<(), String> {
    invoke_catch(cmd, JsValue::NULL).await.map_err(js_error_to_string)?;
    Ok(())
}

pub async fn invoke_parsed<T: DeserializeOwned + Default>(cmd: &str) -> T {
    serde_wasm_bindgen::from_value(invoke(cmd, JsValue::NULL).await).unwrap_or_default()
}

pub async fn invoke_parsed_with_args<A: Serialize, T: DeserializeOwned + Default>(cmd: &str, args: &A) -> T {
    try_invoke_parsed_with_args(cmd, args).await.unwrap_or_default()
}

pub async fn invoke_with_args<A: Serialize>(cmd: &str, args: &A) {
    invoke(cmd, to_js_args(args)).await;
}

pub async fn invoke_and_wait(cmd: &str) {
    invoke(cmd, JsValue::NULL).await;
}
