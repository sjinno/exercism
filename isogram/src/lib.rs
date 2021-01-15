use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    if candidate.is_empty() {
        return true;
    }

    let candidate = candidate.to_ascii_lowercase();
    let mut hs = HashSet::<char>::new();
    for c in candidate.chars() {
        match c {
            '-' | ' ' => continue,
            _ => {
                if !hs.insert(c) {
                    return false;
                }
            }
        }
    }
    true
}
