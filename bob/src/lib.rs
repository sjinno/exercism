use std::char;

pub fn reply(message: &str) -> &str {
    let msg = message.trim();
    println!("{:?}", msg);
    match msg {
        "" => "Fine. Be that way!",
        _ => {
            let msg = msg.replace(" ", "");
            let question = msg.ends_with("?");
            let mut chars = msg.chars().into_iter().filter(|c| c.is_alphabetic());
            match chars.next() {
                Some(_) => {
                    if chars.any(|c| {
                        let d = c.to_digit(36).unwrap();
                        let ch = char::from_digit(d, 36).unwrap();
                        c == ch
                    }) {
                        if question {
                            return "Sure.";
                        }
                        return "Whatever.";
                    } else {
                        if question {
                            return "Calm down, I know what I'm doing!";
                        }
                        return "Whoa, chill out!";
                    }
                }
                None => {
                    if question {
                        return "Sure.";
                    }
                    return "Whatever.";
                }
            }
        }
    }
    // println!("{:#?}", last_sentence);
}
