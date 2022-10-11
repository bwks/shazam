use titlecase::titlecase;

/// Convert a string to a parameterized string
pub fn dasherize(source: String) -> String {
    source.replace(" ", "-").to_lowercase()
}

// Convert a string to title case
pub fn title(source: String) -> String {
    titlecase(source.as_str())
}
