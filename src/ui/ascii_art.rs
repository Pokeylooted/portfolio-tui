//! ASCII art logos for the application

/// Duck ASCII art logo (similar to the one on the website)
pub const DUCK_LOGO: &str = r#"
    _
  >(.)__
   (___/
"#;

/// Default ASCII art logo
pub const DEFAULT_LOGO: &str = r#"
    _____           _            _           _           _
   |  __ \         | |          | |         | |         | |
   | |__) |__  _ __| |_ ___  ___| | ___   __| | ___   __| |
   |  ___/ _ \| '__| __/ _ \/ __| |/ _ \ / _` |/ _ \ / _` |
   | |  | (_) | |  | ||  __/ (__| | (_) | (_| | (_) | (_| |
   |_|   \___/|_|   \__\___|\___|_|\___/ \__,_|\___/ \__,_|
"#;

/// Get ASCII art logo by name
pub fn get_logo(name: &str) -> &'static str {
    match name.to_lowercase().as_str() {
        "duck" => DUCK_LOGO,
        _ => DEFAULT_LOGO,
    }
}