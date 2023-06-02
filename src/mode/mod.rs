mod basic;
mod custom_body;

pub use basic::Basic;
pub use custom_body::CustomBody;

use crate::utils::{extract_message, Unescaped};
use std::borrow::Cow;
use std::ops::Deref;
use std::panic::PanicInfo;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{Document, HtmlElement};

pub trait PresentationMode {
    fn present(&self, info: PanicDetails) -> Result<(), Cow<'static, str>>;
}

pub struct PanicDetails<'a>(pub &'a PanicInfo<'a>);

impl<'a> PanicDetails<'a> {
    pub fn message(&self) -> Unescaped {
        extract_message(&self.0).into()
    }

    pub fn location(&self) -> Option<Unescaped> {
        self.0.location().map(|l| Unescaped::from(l.to_string()))
    }
}

impl<'a> Deref for PanicDetails<'a> {
    type Target = PanicInfo<'a>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> From<&'a PanicInfo<'a>> for PanicDetails<'a> {
    fn from(info: &'a PanicInfo) -> Self {
        Self(info)
    }
}

fn build_body(
    doc: &Document,
    body_class: Option<&str>,
    content: String,
) -> Result<HtmlElement, JsValue> {
    let body = doc.create_element("body")?;

    let body: HtmlElement = body
        .dyn_into()
        .map_err(|_| JsValue::from_str("Unable to convert into HTML element"))?;

    if let Some(class) = body_class {
        body.set_class_name(class);
    }

    body.set_inner_html(&content);

    Ok(body)
}

fn set_body(class: Option<&str>, content: String) -> Result<(), Cow<'static, str>> {
    let doc = web_sys::window()
        .and_then(|w| w.document())
        .ok_or_else(|| "Unable to acquire document")?;

    doc.set_body(Some(&build_body(&doc, class, content).map_err(
        |err| match err.as_string() {
            Some(err) => err,
            None => format!("{err:?}"),
        },
    )?));

    Ok(())
}
