use titlecase::titlecase;

/// Convert a string to a parameterized string
pub fn parameterize(source: String) -> String {
    source.replace(" ", "-").to_lowercase()
}

/// Convert a string to title case
pub fn title_case(source: String) -> String {
    titlecase(source.as_str())
}

/// Convert a string to capital case.
/// This capitalizes the first ascii character of a string
pub fn capitalize(s: String) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}
