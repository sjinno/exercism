use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut output: HashSet<&'a str> = HashSet::new();
    let word_lower = word.to_lowercase();
    let mut word_sorted: Vec<char> = word_lower.chars().collect();
    word_sorted.sort_unstable();

    for anagram in possible_anagrams {
        let anagram_lower = anagram.to_lowercase();
        // Condition check 1: The lengths of `word` and `anagram` should match.
        // Condition check 2: The lowercase of `word` and `anagram` are NOT the same.
        if word_lower.len() == anagram_lower.len() && word_lower != anagram_lower {
            let mut anagram_sorted: Vec<char> = anagram_lower.chars().collect();
            anagram_sorted.sort_unstable();
            // If word_sorted = anagram_sorted, insert it into `output`.
            if word_sorted == anagram_sorted {
                output.insert(anagram);
            }
        }
    }

    output
}
