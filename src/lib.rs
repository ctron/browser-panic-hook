use std::panic;

mod mode;
mod utils;

pub fn handle_panic(info: &panic::PanicInfo) {
    // always print out an error on the console first
    let msg = format!("Application panicked: {info}");
    web_sys::console::error_1(&msg.into());

    // now try the error handler
    if let Err(err) = mode::replace::handle_panic(info) {
        // or fail handling, showing this on the console too
        web_sys::console::error_2(&"Panic hook failed".into(), &err.as_ref().into());
    }
}

pub fn set_once() {
    use std::sync::Once;
    static SET_HOOK: Once = Once::new();
    SET_HOOK.call_once(|| panic::set_hook(Box::new(handle_panic)))
}
