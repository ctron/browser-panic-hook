use browser_panic_hook::{Basic, IntoPanicHook};

/// ensure that we can transform into a yew custom panic hook
#[test]
fn test() {
    yew::set_custom_panic_hook(Basic.into_panic_hook());
}
