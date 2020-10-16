/// Parses the isbn and removing invalid characters.
pub fn parse(input: &str) -> Option<String> {
    // Collect numeric values
    let isbn: Vec<u8> = input
        .chars()
        .flat_map(|c| {
            if c == 'X' {
                Some(10)
            } else {
                c.to_digit(10).map(|n| n as u8)
            }
        })
        .collect();

    if is10(&isbn) || is13(&isbn) {
        Some(isbn_str(&isbn))
    } else {
        None
    }
}

fn is10(isbn: &[u8]) -> bool {
    isbn.len() == 10 && isbn[9] == checksum10(isbn)
}

fn is13(isbn: &[u8]) -> bool {
    isbn.len() == 13 && isbn[12] == checksum13(isbn)
}

fn checksum10(isbn: &[u8]) -> u8 {
    let checksum = isbn[..9]
        .iter()
        .enumerate()
        .fold(0, |acc, (i, &n)| acc + (i as u32 + 1) * n as u32);
    (checksum % 11) as u8
}

fn checksum13(isbn: &[u8]) -> u8 {
    let checksum = isbn[..12]
        .iter()
        .enumerate()
        .fold(0, |acc, (i, &n)| acc + (1 + 2 * (i as u32 % 2)) * n as u32);
    ((400 - checksum) % 10) as u8
}

fn isbn_str(isbn: &[u8]) -> String {
    isbn.iter()
        .map(|&n| std::char::from_digit(n as _, 10).unwrap_or('X'))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_isbns() {
        assert_eq!(parse(""), None);
        assert_eq!(parse("1234567890"), None);
        assert_eq!(parse("3-440-03914-5"), Some("3440039145".into()));
        assert_eq!(parse("978-3923923410"), Some("9783923923410".into()));
        assert_eq!(parse("978-1338099133"), Some("9781338099133".into()));
        assert_eq!(parse("353411292X"), Some("353411292X".into()));
        assert_eq!(parse("35341129XX"), None);
    }
}
