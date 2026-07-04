use serde::de::DeserializeOwned;
use serde::Serialize;
use wasm_bindgen::prelude::{wasm_bindgen, Closure};
use wasm_bindgen::{JsCast, JsValue};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"], js_name = invoke)]
    pub async fn invoke(cmd: &str, args: JsValue) -> JsValue;

    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"], js_name = invoke, catch)]
    async fn invoke_catch(cmd: &str, args: JsValue) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "event"], js_name = listen, catch)]
    async fn listen_raw(event: &str, handler: &js_sys::Function) -> Result<JsValue, JsValue>;
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

pub async fn invoke_parsed_with_args<A: Serialize, T: DeserializeOwned + Default>(cmd: &str, args: &A) -> T {
    try_invoke_parsed_with_args(cmd, args).await.unwrap_or_default()
}

pub fn listen_app_lifetime(event: &'static str, handler: impl FnMut(JsValue) + 'static) {
    let closure = Closure::<dyn FnMut(JsValue)>::new(handler);
    wasm_bindgen_futures::spawn_local(async move {
        if let Err(e) = listen_raw(event, closure.as_ref().unchecked_ref()).await {
            leptos::logging::error!("Не удалось подписаться на {event}: {e:?}");
        }
        closure.forget();
    });
}

#[derive(serde::Deserialize)]
struct EventEnvelope<T> {
    payload: T,
}

pub fn listen_app_lifetime_parsed<T: DeserializeOwned + 'static>(
    event: &'static str,
    mut handler: impl FnMut(T) + 'static,
) {
    listen_app_lifetime(event, move |raw| {
        match serde_wasm_bindgen::from_value::<EventEnvelope<T>>(raw) {
            Ok(envelope) => handler(envelope.payload),
            Err(e) => leptos::logging::error!("Событие {event}: не удалось разобрать payload: {e}"),
        }
    });
}
