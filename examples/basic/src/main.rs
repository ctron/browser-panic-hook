use wasm_bindgen::prelude::*;

#[cfg(not(debug_assertions))]
const LOG_LEVEL: log::Level = log::Level::Info;
#[cfg(debug_assertions)]
const LOG_LEVEL: log::Level = log::Level::Trace;

fn cause_panic() {
    panic!("Don't panic!");
}

pub fn main() -> Result<(), JsValue> {
    wasm_logger::init(wasm_logger::Config::new(LOG_LEVEL));
    browser_panic_hook::set_once_default();

    if let Some(element) = gloo_utils::document().get_element_by_id("panic") {
        gloo_events::EventListener::new(&element, "click", |_| {
            cause_panic();
        })
        .forget();
    }

    Ok(())
}
