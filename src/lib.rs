mod mode;
mod utils;

pub use mode::*;
pub use utils::{escape_text, Unescaped};

use std::panic;
use std::sync::Once;

fn handle<M>(mode: &M, info: &panic::PanicInfo)
where
    M: PresentationMode,
{
    // always print out an error on the console first
    let msg = format!("Application panicked: {info}");
    web_sys::console::error_1(&msg.into());

    let details = PanicDetails::from(info);
    if let Err(err) = mode.present(details) {
        // or fail handling, showing this on the console too
        web_sys::console::error_2(&"Panic hook failed".into(), &err.as_ref().into());
    }
}

/// Turn something into a panic hook.
pub trait IntoPanicHook {
    fn into_panic_hook(self) -> Box<dyn Fn(&panic::PanicInfo) + 'static + Sync + Send>;
}

impl<T> IntoPanicHook for T
where
    T: PresentationMode + 'static + Send + Sync,
{
    fn into_panic_hook(self) -> Box<dyn Fn(&panic::PanicInfo) + 'static + Sync + Send> {
        Box::new(move |info| handle(&self, info))
    }
}

static SET_HOOK: Once = Once::new();

/// Set the panic hook and ensure it is only set once.
pub fn set_once<F, M>(f: F)
where
    F: FnOnce() -> M,
    M: PresentationMode + 'static + Send + Sync,
{
    SET_HOOK.call_once(move || panic::set_hook(f().into_panic_hook()))
}

/// Set the panic hook to [`Basic`] and ensure it is only set once.
#[inline]
pub fn set_once_default() {
    set_once(|| Basic)
}
