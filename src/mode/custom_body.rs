use crate::mode::{PanicDetails, PresentationMode};
use std::borrow::Cow;

/// Replace the body with a string generated by a custom function.
///
/// **NOTE:** The function needs to ensure that the [`PanicDetails`] information is escaped. This
/// can be done using [`crate::utils::escape_text`].
pub struct CustomBody(pub Box<dyn Fn(PanicDetails) -> String + 'static + Send + Sync>);

impl<F> From<F> for CustomBody
where
    F: for<'a> Fn(PanicDetails<'a>) -> String + 'static + Send + Sync,
{
    fn from(value: F) -> Self {
        Self(Box::new(value))
    }
}

impl PresentationMode for CustomBody {
    fn present(&self, details: PanicDetails) -> Result<(), Cow<'static, str>> {
        super::set_body(None, self.0(details))
    }
}
