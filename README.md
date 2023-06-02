# Browser Panic Hook

[![CI](https://github.com/ctron/browser-panic-hook/workflows/CI/badge.svg)](https://github.com/ctron/browser-panic-hook/actions?query=workflow%3A%22CI%22)
[![GitHub release (latest SemVer)](https://img.shields.io/github/v/tag/ctron/browser-panic-hook?sort=semver)](https://github.com/ctron/browser-panic-hook/releases)
[![crates.io](https://img.shields.io/crates/v/browser-panic-hook.svg)](https://crates.io/crates/browser-panic-hook)
[![docs.rs](https://docs.rs/browser-panic-hook/badge.svg)](https://docs.rs/browser-panic-hook)


A go-to panic handler for WebAssembly based Rust application is [console_error_panic_hook](https://github.com/rustwasm/console_error_panic_hook), which does the job, but isn't really end user-friendly.

In the case of running the WebAssembly application as a frontend application, we do have the browser which can help
interacting with the user, so why not leverage it.

That is what this crate does, present the panic to the user in a reasonable way.

## Presentation

In order to keep things relatively simple, some basic HTML with some CSS classes is rendered. This can be used to
control the styling of the representation.

In order to understand what can be styled, please take a look at the actual HTML.

**NOTE:** I most cases you will have a style sheet in place, as only basic HTML is rendered, it might actually
be necessary to provide some styles, other overriding or adapting to your environment.

The panic is also always still logged to the console.

## Usage

In a nutshell, you need to add the dependency and then set the handler once:

```rust
pub fn main() -> Result<(), JsValue> {
    browser_panic_hook::set_once();

    // run your application ...

    Ok(())
}
```

A more complete example can be found in the [example](example) folder.

## Modes

Currently, there is only one mode of presenting/handling the panic, by replacing the full body content.

Additional modes in the future are possible, and should be configured using feature flags, as only one mode seems to
make sense.

## Yew

Yew already sets a default panic hook. This can be overridden using:

```rust
pub fn main() -> Result<(), JsValue> {
    // provide a custom panic hook
    yew::set_custom_panic_hook(Box::new(browser_panic_hook::handle_panic));
    // run the application
    yew::Renderer::<app::Application>::new().render();
    Ok(())
}
```

## Future improvements

Additional improvements could be done, like:

* [ ] Call a diagnostic endpoint with the panic information
* [ ] Create a mode which overlays instead of replaces the HTML body
* [ ] Allow adding additional, application specific, HTML (like "click here to report the error") 
* [ ] For sure some more …
