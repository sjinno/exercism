/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let code = code.replace(" ", "");

    // Check if the length is less than 2.
    // If true, return false.
    let length = code.len();
    if length < 2 {
        return false;
    }

    // Chech if there is any character that is NOT numeric.
    // If true, return false.
    if code.chars().into_iter().any(|c| !c.is_numeric()) {
        return false;
    }

    let mut sum = 0;
    code.chars()
        .into_iter()
        .rev()
        .enumerate()
        .for_each(|(i, x)| {
            let x = x.to_digit(10).unwrap();
            if i.rem_euclid(2) == 1 {
                if x > 4 {
                    sum += 2 * x - 9;
                } else {
                    sum += 2 * x;
                }
            } else {
                sum += x;
            }
        });

    sum.rem_euclid(10) == 0
}
