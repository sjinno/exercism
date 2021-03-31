pub fn number(user_number: &str) -> Option<String> {
    let cleaned_user_number = user_number
        .chars()
        .filter(|c| *c == '+' || c.is_numeric())
        .collect::<Vec<_>>();

    match cleaned_user_number.len() {
        10 => validate_phone_number(&cleaned_user_number),
        11 if cleaned_user_number[0] == '1' => validate_phone_number(&cleaned_user_number[1..]),
        12 if cleaned_user_number[0..2] == ['+', '1'] => {
            validate_phone_number(&cleaned_user_number[2..])
        }
        _ => None,
    }
}

fn validate_phone_number(number: &[char]) -> Option<String> {
    match (number[0], number[3]) {
        //     Area code != 0 or 1.
        // Exchange code != 0 or 1.
        ('0'..='1', _) | (_, '0'..='1') => None,
        _ => Some(number.iter().collect()),
    }
}
