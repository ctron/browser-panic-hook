use crate::mode::{PanicDetails, PresentationMode};
use crate::utils::Unescaped;
use std::borrow::Cow;

/// Replace the body with a simple panic information representation.
pub struct Basic;

impl PresentationMode for Basic {
    fn present(&self, details: PanicDetails) -> Result<(), Cow<'static, str>> {
        super::set_body(Some("panicked"), build_content(details))
    }
}

fn build_content(details: PanicDetails) -> String {
    let message = details.message();
    let internals = Unescaped::from(format!("{info}\n\n\n{info:#?}", info = &details.0));
    let location = details
        .location()
        .unwrap_or_else(|| Unescaped::safe("<i>Unknown</i>"));

    format!(
        r#"
<main class="panicked__main">
<h1 class="panicked__title">Application panicked!</h1>

<dl class="panicked__overview">
    <dt class="panicked__overview__reason">Reason</dt><dd class="panicked__overview__reason">{message}</dd>
    <dt class="panicked__overview__location">Location</dt><dd class="panicked__overview__location">{location}</dd>
</dl>

<details class="panicked__details">
    <summary>Internal Details</summary>
    <pre>{internals}</pre>
</details>

</main>
    "#,
    )
}
