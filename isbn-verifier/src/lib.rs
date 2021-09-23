/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let mut counter = 1;
    let mut sum = 0;

    for c in isbn.chars().rev() {
        if c.is_numeric() {
            sum += c.to_digit(10).unwrap() * counter;
            counter += 1;
        } else if c == '-' {
            continue;
        } else if counter == 1 && c == 'X' {
            sum += 10;
            counter += 1;
        } else {
            return false;
        }
    }

    counter == 11 && sum.rem_euclid(11) == 0
}
