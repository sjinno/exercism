/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let mut filtered_isbn = isbn.chars().filter(|c| c.is_numeric());
    if filtered_isbn.count() != 10 {
        return false;
    }

    true
}
