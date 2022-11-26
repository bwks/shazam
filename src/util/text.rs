/// Convert a string to a parameterized string
pub fn parameterize(source: String) -> String {
    let stripped = source
        .replace(
            [
                '-', '_', ':', ';', '(', ')', '[', ']', '{', '}', '<', '>', '/', '\\', '\'', '"',
                '!', '@', '#', '$', '%', '^', '&', '*', '?', ',', '.', '+', '=', '|', '~', '`',
            ],
            " ",
        )
        .to_lowercase();

    let split: Vec<&str> = stripped.split_ascii_whitespace().collect();

    split
        .iter()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
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

#[cfg(test)]
mod tests {
    use crate::util::text::{capitalize, parameterize};

    #[test]
    fn returns_parameterized_string() {
        let test_cases = vec![
            ("Test String".to_owned(), "test-string".to_owned()),
            ("TestString".to_owned(), "teststring".to_owned()),
            ("test string".to_owned(), "test-string".to_owned()),
            ("test_string".to_owned(), "test-string".to_owned()),
            ("test-string".to_owned(), "test-string".to_owned()),
            ("test: string".to_owned(), "test-string".to_owned()),
            ("test : - _ string".to_owned(), "test-string".to_owned()),
            ("test :-_ string".to_owned(), "test-string".to_owned()),
            (
                "test-_:()[]{}<>/\\\"!@#$%^&*? .+=|~`'string".to_owned(),
                "test-string".to_owned(),
            ),
        ];
        for t in test_cases {
            let result = parameterize(t.0);
            assert_eq!(result, t.1);
        }
    }

    #[test]
    fn returns_capitalized_string() {
        let test_cases = vec![
            ("test string".to_owned(), "Test string".to_owned()),
            ("Test string".to_owned(), "Test string".to_owned()),
        ];
        for t in test_cases {
            let result = capitalize(t.0);
            assert_eq!(result, t.1);
        }
    }
}
