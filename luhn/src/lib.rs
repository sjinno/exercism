pub fn is_valid(code: &str) -> bool {
    let double_num = |num| {
        if num > 4 {
            2 * num - 9
        } else {
            2 * num
        }
    };

    code.chars()
        .rev()
        .filter(|c| !c.is_whitespace())
        .try_fold((0, 0), |(sum, count), val| {
            val.to_digit(10).map(|num| {
                if count % 2 == 1 {
                    (double_num(num) + sum, count + 1)
                } else {
                    (num + sum, count + 1)
                }
            })
        })
        .map_or(false, |(sum, count)| sum % 10 == 0 && count > 1)
}
