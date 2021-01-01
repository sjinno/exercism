// /// Check a Luhn checksum.
// pub fn is_valid(code: &str) -> bool {
//     let code = code.replace(" ", "");

//     // Check if the length is less than 2.
//     // If true, return false.
//     let length = code.len();
//     if length < 2 {
//         return false;
//     }

//     // Chech if there is any character that is NOT numeric.
//     // If true, return false.
//     if code.chars().into_iter().any(|c| !c.is_numeric()) {
//         return false;
//     }

//     let mut sum = 0;
//     code.chars()
//         .into_iter()
//         .rev()
//         .enumerate()
//         .for_each(|(i, x)| {
//             let x = x.to_digit(10).unwrap();
//             if i.rem_euclid(2) == 1 {
//                 if x > 4 {
//                     sum += 2 * x - 9;
//                 } else {
//                     sum += 2 * x;
//                 }
//             } else {
//                 sum += x;
//             }
//         });

//     sum.rem_euclid(10) == 0
// }

// // Community solution 1
// const INVALID_CHECKSUM: u32 = 1;

// /// Check a Luhn checksum.
// pub fn is_valid(code: &str) -> bool {
//     let mut count = 0;
//     let checksum: u32 = code
//         .chars()
//         .filter(|ch| !ch.is_whitespace())
//         .rev()
//         .enumerate()
//         .try_fold(0, |acc, (i, ch)| {
//             char::to_digit(ch, 10).map(|n| {
//                 count += 1;
//                 acc + if i & 1 == 1 {
//                     let n = 2 * n;
//                     if n > 9 {
//                         n - 9
//                     } else {
//                         n
//                     }
//                 } else {
//                     n
//                 }
//             })
//         })
//         .unwrap_or(INVALID_CHECKSUM);
//     checksum % 10 == 0 && count > 1
// }

// Community solution 2
/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    code.chars()
        .rev()
        .filter(|c| !c.is_whitespace())
        .try_fold((0, 0), |(sum, count), val| {
            val.to_digit(10)
                .map(|num| if count % 2 == 1 { num * 2 } else { num })
                .map(|num| if num > 9 { num - 9 } else { num })
                .map(|num| (num + sum, count + 1))
        })
        .map_or(false, |(sum, count)| sum % 10 == 0 && count > 1)
}
