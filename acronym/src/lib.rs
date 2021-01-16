// pub fn abbreviate(phrase: &str) -> String {
//     const ALPHABET: &[char; 26] = &[
//         'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
//         'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
//     ];

//     if phrase.is_empty() {
//         return String::new();
//     }

//     let mut split_phrase = phrase.split(' ');
//     let mut ans = String::new();

//     while let Some(word) = split_phrase.next() {
//         if word.chars().into_iter().all(|c| !c.is_alphabetic()) {
//             continue;
//         }

//         let tmp = word.clone().to_ascii_uppercase();
//         if tmp == word {
//             ans.push(word.chars().next().unwrap());
//         } else {
//             let mut iter = word.chars();
//             while let Some(initial) = iter.next() {
//                 // println!("{:?}", initial);
//                 if initial.is_alphabetic() {
//                     ans.push(initial.to_ascii_uppercase());
//                     break;
//                 }
//             }
//             while let Some(c) = iter.next() {
//                 if ALPHABET.contains(&c) {
//                     ans.push(c);
//                 } else if c == '-' {
//                     if let Some(ch) = iter.next() {
//                         // println!("{:?}", ch);
//                         ans.push(ch.to_ascii_uppercase());
//                     }
//                 }
//             }
//         }
//     }

//     ans
// }

// // Community solution 1
// pub fn abbreviate(phrase: &str) -> String {
//     phrase
//         .split(|c: char| !c.is_alphabetic() && c != '\'')
//         .map(|s| {
//             capitalize(s)
//                 .split(|c: char| !c.is_uppercase())
//                 .flat_map(|c| c.chars().nth(0))
//                 .collect::<String>()
//         })
//         .collect::<String>()
// }

// fn capitalize(s: &str) -> String {
//     // Capitalize
//     let mut c = s.chars();
//     match c.next() {
//         None => String::new(),
//         Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
//     }
// }

// Community solution 2
pub fn abbreviate(name: &str) -> String {
    name.split(|c: char| !c.is_alphabetic() && c != '\'')
        .flat_map(|word| {
            word.chars().take(1).chain(
                word.chars()
                    .skip_while(|c| c.is_uppercase())
                    .filter(|c| c.is_uppercase()),
            )
        })
        .collect::<String>()
        .to_uppercase()
}
