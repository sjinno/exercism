pub fn abbreviate(phrase: &str) -> String {
    const ALPHABET: &[char; 26] = &[
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
        'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    ];

    if phrase.is_empty() {
        return String::new();
    }

    let mut split_phrase = phrase.split(' ');
    let mut ans = String::new();

    while let Some(word) = split_phrase.next() {
        if word.chars().into_iter().all(|c| !c.is_alphabetic()) {
            continue;
        }

        let tmp = word.clone().to_ascii_uppercase();
        if tmp == word {
            ans.push(word.chars().next().unwrap());
        } else {
            let mut iter = word.chars();
            while let Some(initial) = iter.next() {
                // println!("{:?}", initial);
                if initial.is_alphabetic() {
                    ans.push(initial.to_ascii_uppercase());
                    break;
                }
            }
            while let Some(c) = iter.next() {
                if ALPHABET.contains(&c) {
                    ans.push(c);
                } else if c == '-' {
                    if let Some(ch) = iter.next() {
                        // println!("{:?}", ch);
                        ans.push(ch.to_ascii_uppercase());
                    }
                }
            }
        }
    }

    ans
}
