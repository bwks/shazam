/// Convert a string to a parameterized string
pub fn dasherize(source: String) -> String {
    source.replace(" ", "-").to_lowercase()
}
