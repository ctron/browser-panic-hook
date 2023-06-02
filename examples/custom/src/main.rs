use browser_panic_hook::Unescaped;
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
    browser_panic_hook::set_once(|| {
        browser_panic_hook::CustomBody(Box::new(|details| {
            format!(
                r##"

    <div class="container" style="padding-top: 40vh;">
        
        <div class="row">
        
            <div class="col-2 d-flex justify-content-end align-items-center">
        
                    <svg xmlns="http://www.w3.org/2000/svg" width="64" height="64" fill="#dc3545" class="bi bi-exclamation-triangle-fill" viewBox="0 0 16 16">
                        <path d="M8.982 1.566a1.13 1.13 0 0 0-1.96 0L.165 13.233c-.457.778.091 1.767.98 1.767h13.713c.889 0 1.438-.99.98-1.767L8.982 1.566zM8 5c.535 0 .954.462.9.995l-.35 3.507a.552.552 0 0 1-1.1 0L7.1 5.995A.905.905 0 0 1 8 5zm.002 6a1 1 0 1 1 0 2 1 1 0 0 1 0-2z"/>
                    </svg>
           
            </div>
            
            <div class="col">
            
            <div>
                <h1>Application panicked</h1>
                <h2>{message}</h2>
            </div>
            
            </div>
    
        </div>
        
        <div class="row">
            <div class="col-2"></div>
            <div class="col text-secondary">
                <a class="link-secondary" data-bs-toggle="collapse" href="#panickedDetails" role="button" aria-expanded="false" aria-controls="panickedDetails">Details</a>
                ‧
                <a class="link-secondary" href="#" onclick="alert('This is just a demo!');">Report</a>

                <div class="collapse" id="panickedDetails">
                    <p>{location}</p> 
                    <pre>{internals}</pre>
                </div>
            </div>
        </div>

    </div>

"##,
                message = details.message(),
                location = details
                    .location()
                    .map(|l| format!(r#"Location: <code>{l}</code>"#))
                    .unwrap_or_else(|| "<i>Unknown Location</i>".to_string()),
                internals = Unescaped::from(format!("{info}\n\n{info:#?}", info = &details.0)),
            )
        }))
    });

    if let Some(element) = gloo_utils::document().get_element_by_id("panic") {
        gloo_events::EventListener::new(&element, "click", |_| {
            cause_panic();
        })
        .forget();
    }

    Ok(())
}
