use crate::utils::extract_message;
use std::borrow::Cow;
use std::panic::PanicInfo;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{Document, HtmlElement};

pub fn handle_panic(info: &PanicInfo) -> Result<(), Cow<'static, str>> {
    let doc = web_sys::window()
        .and_then(|w| w.document())
        .ok_or_else(|| "Unable to acquire document")?;

    doc.set_body(Some(&build_body(&doc, info).map_err(
        |err| match err.as_string() {
            Some(err) => err,
            None => format!("{err:?}"),
        },
    )?));

    Ok(())
}

fn build_body(doc: &Document, info: &PanicInfo) -> Result<HtmlElement, JsValue> {
    let body = doc.create_element("body")?;

    let body: HtmlElement = body
        .dyn_into()
        .map_err(|_| JsValue::from_str("Unable to convert into HTML element"))?;

    body.set_class_name("panicked");

    let message = escape_text(extract_message(info));
    let details = escape_text(format!("{info}\n\n\n{info:#?}"));

    let location = info
        .location()
        .map(|l| escape_text(l.to_string()))
        .unwrap_or_else(|| "<i>Unknown</i>".to_string());

    body.set_inner_html(&format!(
        r#"
<main class="panicked__main">
<h1 class="panicked__title">Application panicked!</h1>

<dl class="panicked__overview">
    <dt class="panicked__overview__reason">Reason</dt><dd class="panicked__overview__reason">{message}</dd>
    <dt class="panicked__overview__location">Location</dt><dd class="panicked__overview__location">{location}</dd>
</dl>

<details class="panicked__details">
    <summary>Internal Details</summary>
    <pre>{details}</pre>
</details>

</main>
    "#,
    ));

    Ok(body)
}

fn escape_text(text: String) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}
