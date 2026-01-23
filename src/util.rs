use unicode_normalization::UnicodeNormalization;

pub const PKG_NAME: &str = env!("CARGO_PKG_NAME");
pub const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const PKG_REPOSITORY: &str = env!("CARGO_PKG_REPOSITORY");
pub const PKG_AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
pub const PKG_DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
pub const PKG_LICENSE: &str = env!("CARGO_PKG_LICENSE");

/// initialize tracing
pub fn logging() {
    use tracing_subscriber::EnvFilter;
    use tracing_subscriber::layer::SubscriberExt;
    use tracing_subscriber::util::SubscriberInitExt;
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();
}

pub fn convert_ascii_lower(input: &str, spaces: bool) -> String {
    let input = input.trim();
    let mut output = String::with_capacity(input.len());
    let mut last = None;
    for c in input.nfkd() {
        let c = c.to_ascii_lowercase();
        match (last, c) {
            (Some('a' | 'o' | 'u'), '\u{0308}') => output.push_str("e"),
            (_, 'ß') => output.push_str("ss"),
            (Some(' ' | '.'), c) if c.is_whitespace() => {}
            _ if c.is_whitespace() => output.push(if spaces { ' ' } else { '.' }),
            _ if c.is_ascii_alphanumeric() => output.push(c),
            _ => {}
        }
        last = Some(c);
    }
    output
}

#[cfg(test)]
mod tests {
    use crate::util::logging;

    #[test]
    fn test_strip_to_ascii_lower() {
        logging();
        assert_eq!(
            super::convert_ascii_lower("Föö Bär 123!", false),
            "foeoe.baer.123"
        );
        assert_eq!(
            super::convert_ascii_lower("  ÄÖÜß Test ", false),
            "aeoeuess.test"
        );
        assert_eq!(
            super::convert_ascii_lower("NormalString", false),
            "normalstring"
        );
        assert_eq!(
            super::convert_ascii_lower("  Multiple   Spaces ", true),
            "multiple spaces"
        );
    }
}
