pub fn is_armstrong_number(num: u32) -> bool {
    let num_in_string = num.to_string();
    let length = num_in_string.len() as u32;
    num_in_string.chars().into_iter().fold(0, |acc, x| {
        let x = x.to_digit(10).unwrap();
        acc + x.pow(length)
    }) == num
}
