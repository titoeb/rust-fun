//! This modules contains greetings for the english language.
//!
//! # Examples
//! ```
//! let username = "John";
//! println("{}, {}!", phrases::greetings::english::hello(), username);
//! ```

/// Greet someone you have met.
pub fn hello() -> String {
    return "Hello".to_string();
}
/// Say goodbye to someone.
pub fn goodbye() -> String {
    return "Goodbye".to_string();
}
