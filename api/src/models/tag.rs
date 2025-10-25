use std::ops::Deref;

/// Validated tag wrapper
///
/// Tags must be:
/// - Non-empty
/// - Max 50 characters
/// - Alphanumeric, hyphens, and underscores only
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tag(String);

impl Tag {
    /// Maximum length for a tag
    pub const MAX_LENGTH: usize = 50;

    /// Maximum number of tags allowed in a query
    pub const MAX_COUNT: usize = 10;

    /// Create a new validated tag
    pub fn new(s: impl Into<String>) -> Option<Self> {
        let s = s.into();
        if Self::is_valid(&s) {
            Some(Tag(s))
        } else {
            None
        }
    }

    /// Validate a tag string
    fn is_valid(s: &str) -> bool {
        !s.is_empty()
            && s.len() <= Self::MAX_LENGTH
            && s.chars().all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
    }

    /// Parse and validate a comma-separated list of tags from query parameter
    /// Returns only valid tags, up to MAX_COUNT
    pub fn parse_many(value: &str) -> Vec<Tag> {
        value
            .split(',')
            .take(Self::MAX_COUNT)
            .filter_map(|s| Tag::new(s.trim()))
            .collect()
    }
}

impl Deref for Tag {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_tags() {
        assert!(Tag::new("rust").is_some());
        assert!(Tag::new("web-dev").is_some());
        assert!(Tag::new("rust_lang").is_some());
        assert!(Tag::new("rust-2024").is_some());
    }

    #[test]
    fn test_invalid_tags() {
        assert!(Tag::new("").is_none()); // empty
        assert!(Tag::new("a".repeat(51)).is_none()); // too long
        assert!(Tag::new("rust/lang").is_none()); // invalid char
        assert!(Tag::new("rust lang").is_none()); // space
        assert!(Tag::new("../etc/passwd").is_none()); // path traversal attempt
        assert!(Tag::new("'; DROP TABLE").is_none()); // SQL injection attempt
    }

    #[test]
    fn test_parse_many() {
        let tags = Tag::parse_many("rust,python,javascript");
        assert_eq!(tags.len(), 3);
        assert_eq!(&*tags[0], "rust");
        assert_eq!(&*tags[1], "python");
        assert_eq!(&*tags[2], "javascript");

        // Test trimming
        let tags = Tag::parse_many(" rust , python , javascript ");
        assert_eq!(tags.len(), 3);

        // Test filtering invalid
        let tags = Tag::parse_many("rust,invalid tag,python,../etc/passwd");
        assert_eq!(tags.len(), 2);
        assert_eq!(&*tags[0], "rust");
        assert_eq!(&*tags[1], "python");

        // Test max count limit
        let tags = Tag::parse_many("t1,t2,t3,t4,t5,t6,t7,t8,t9,t10,t11,t12");
        assert_eq!(tags.len(), Tag::MAX_COUNT);

        // Test empty result
        let tags = Tag::parse_many("invalid tag,another bad");
        assert!(tags.is_empty());
    }

    #[test]
    fn test_deref() {
        let tag = Tag::new("rust").unwrap();
        // Can use as &str
        assert_eq!(&*tag, "rust");
        // Works with string methods
        assert!(tag.starts_with("ru"));
        assert_eq!(tag.len(), 4);
    }
}
