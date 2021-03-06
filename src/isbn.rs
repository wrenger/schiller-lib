/// Parses the isbn and removing invalid characters.
///
/// If the checksum is invalid the stripped invalid isbn is returned.
pub fn parse(input: &str) -> Result<String, String> {
    // Collect numeric values
    let isbn: Vec<u8> = input
        .chars()
        .filter_map(|c| {
            if c == 'X' || c == 'x' {
                Some(10)
            } else {
                c.to_digit(10).map(|n| n as u8)
            }
        })
        .collect();

    if is10(&isbn) || is13(&isbn) {
        Ok(isbn_str(&isbn))
    } else {
        Err(isbn_str(&isbn))
    }
}

fn is10(isbn: &[u8]) -> bool {
    isbn.len() == 10 && isbn[0..9].iter().all(|&n| n < 10) && isbn[9] == checksum10(isbn)
}

fn is13(isbn: &[u8]) -> bool {
    isbn.len() == 13 && isbn.iter().all(|&n| n < 10) && isbn[12] == checksum13(isbn)
}

fn checksum10(isbn: &[u8]) -> u8 {
    let checksum = isbn[..9]
        .iter()
        .enumerate()
        .fold(0, |acc, (i, &n)| acc + (i + 1) * n as usize);
    (checksum % 11) as u8
}

fn checksum13(isbn: &[u8]) -> u8 {
    let checksum = isbn[..12]
        .iter()
        .enumerate()
        .fold(0, |acc, (i, &n)| acc + (1 + 2 * (i % 2)) * n as usize);
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
        assert_eq!(parse(""), Err("".into()));
        assert_eq!(parse("1234567890"), Err("1234567890".into()));
        assert_eq!(parse("3-440-03914-5"), Ok("3440039145".into()));
        assert_eq!(parse("978-3923923410"), Ok("9783923923410".into()));
        assert_eq!(parse("978-1338099133"), Ok("9781338099133".into()));
        assert_eq!(parse("353411292X"), Ok("353411292X".into()));
        assert_eq!(parse("35341129XX"), Err("35341129XX".into()));
    }
}
