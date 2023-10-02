use js_sys::Object;
use wasm_bindgen::prelude::*;
use worker::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = Object)]
    pub type EdgeHyperdrive;

    #[wasm_bindgen(method, getter, js_name=connectionString)]
    pub fn connection_string(this: &EdgeHyperdrive) -> String;
}

pub struct Hyperdrive {
    inner: EdgeHyperdrive,
}

impl Hyperdrive {
    pub fn connection_string(&self) -> String {
        self.inner.connection_string()
    }
}

impl EnvBinding for Hyperdrive {
    const TYPE_NAME: &'static str = "Hyperdrive";
}


impl JsCast for Hyperdrive {
    fn instanceof(val: &JsValue) -> bool {
        val.is_instance_of::<EdgeHyperdrive>()
    }

    fn unchecked_from_js(val: JsValue) -> Self {
        Self { inner: val.into() }
    }

    fn unchecked_from_js_ref(val: &JsValue) -> &Self {
        unsafe { &*(val as *const JsValue as *const Self) }
    }
}

impl AsRef<JsValue> for Hyperdrive {
    fn as_ref(&self) -> &JsValue {
        &self.inner
    }
}

impl From<Hyperdrive> for JsValue {
    fn from(hyperdrive: Hyperdrive) -> Self {
        JsValue::from(hyperdrive.inner)
    }
}