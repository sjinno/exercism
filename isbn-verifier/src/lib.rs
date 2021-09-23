/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    if isbn.is_empty() || isbn[..isbn.len() - 1].chars().any(|c| c.is_alphabetic()) {
        return false;
    }

    let filtered_isbn = isbn
        .chars()
        .filter(|c| c.is_numeric() || *c == 'X')
        .map(|c| {
            if c == 'X' {
                return 10;
            }
            c.to_digit(10).unwrap()
        });

    if filtered_isbn.clone().count() != 10 {
        return false;
    }

    filtered_isbn
        .enumerate()
        .fold(0, |acc, (idx, x)| acc + (10 - idx) * x as usize)
        .rem_euclid(11)
        == 0
}
