use std::panic::PanicInfo;

/// Extract the message of the panic info
///
/// We need to extract the message from the string. That can fail and lead to wrong results, but
/// currently is the only wait to only get the message.
///
/// If we fail, we do return the full "to string" representation of the panic info, which might
/// be better than nothing.
///
/// This is necessary until `panic_info_message` is stabilized, see rust-lang/rust#66745
pub fn extract_message(info: &PanicInfo) -> String {
    // first turn it into a string using the Display format
    let display = info.to_string();

    // try to strip away the prefix "panicked at", up until the first '
    let s = match display.strip_prefix("panicked at '") {
        Some(s) => s,
        None => return display,
    };

    // if that worked, try to find the first `'` from the other side of the string
    let s = match s.rsplit_once('\'') {
        Some((s, _)) => s,
        None => return display,
    };

    // we should have captured everything between the two outer most '

    s.to_string()
}
