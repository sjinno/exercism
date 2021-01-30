use std::collections::HashSet;

/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    let mut hs = HashSet::new();
    sentence.chars().into_iter().for_each(|c| { 
        if c.is_alphabetic() { hs.insert(c); }
    });
    hs.len() > 25
}
