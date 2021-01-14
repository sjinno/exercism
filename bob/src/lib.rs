// use std::char;

// pub fn reply(message: &str) -> &str {
//     let msg = message.trim();
//     println!("{:?}", msg);
//     match msg {
//         "" => "Fine. Be that way!",
//         _ => {
//             let msg = msg.replace(" ", "");
//             let question = msg.ends_with('?');
//             let mut chars = msg.chars().into_iter().filter(|c| c.is_alphabetic());
//             match chars.next() {
//                 Some(_) => {
//                     if chars.any(|c| {
//                         let d = c.to_digit(36).unwrap();
//                         let ch = char::from_digit(d, 36).unwrap();
//                         c == ch
//                     }) {
//                         if question {
//                             return "Sure.";
//                         }
//                         return "Whatever.";
//                     } else {
//                         if question {
//                             return "Calm down, I know what I'm doing!";
//                         }
//                         return "Whoa, chill out!";
//                     }
//                 }
//                 None => {
//                     if question {
//                         return "Sure.";
//                     }
//                     return "Whatever.";
//                 }
//             }
//         }
//     }
//     // println!("{:#?}", last_sentence);
// }

// // Community solution 1 -- This is very nice...
// fn is_yelling(message: &str) -> bool {
//     let have_letters: bool = message.chars().filter(|x| x.is_alphabetic()).count() > 0;
//     message.to_uppercase() == message && have_letters
// }

// pub fn reply(message: &str) -> &str {
//     match message.trim() {
//         m if m.trim().len() == 0 => "Fine. Be that way!",
//         m if m.ends_with("?") && is_yelling(m) => "Calm down, I know what I'm doing!",
//         m if m.ends_with("?") => "Sure.",
//         m if is_yelling(m) => "Whoa, chill out!",
//         _ => "Whatever.",
//     }
// }

// Community solution 2 -- This one is actually cool, too.
pub fn reply(message: &str) -> &str {
    match message.trim() {
        x if x.ends_with('?') && x.to_uppercase() == x && x.contains(char::is_alphabetic) => {
            "Calm down, I know what I'm doing!"
        }
        x if x.to_uppercase() == x && x.contains(char::is_alphabetic) => "Whoa, chill out!",
        x if x.ends_with('?') => "Sure.",
        x if x.is_empty() => "Fine. Be that way!",
        _ => "Whatever.",
    }
}
