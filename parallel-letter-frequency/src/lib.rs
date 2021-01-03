// I could've done this using is_alphabetic() but I wanted practice regular expressions..
// Although I probably should've benchmarked both cases.
use regex::Regex;
use std::collections::HashMap;
use std::thread;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let mut answer: HashMap<char, usize> = HashMap::new();
    // Initialize regex to match non-alphabetic letters.

    let re = Regex::new(r"[^A-Za-zäöüßÄÖÜ]").unwrap();
    // Join all the strs into one.
    let joined_input = input.join("");
    // Replace non-alphabetic letters with "".
    let special_chars_removed_input = re.replace_all(&joined_input, "");
    let length = special_chars_removed_input.len();
    if length == 0 {
        return answer;
    }

    // Lowercase all letters.
    let lowercased_input = special_chars_removed_input.to_lowercase();
    let mut chars = lowercased_input.chars();
    // Calculate how many chars to hold in each index of a vector.
    // I did +1 so that I could fit chars to each index almost all equally
    // in case there is a remainder from the division.
    let cap = length.div_euclid(worker_count) + 1;
    let mut workers = Vec::with_capacity(worker_count);
    for _ in 0..worker_count {
        let chunk = chars.by_ref().take(cap).collect::<Vec<_>>();
        workers.push(chunk);
    }

    let mut handles = vec![];
    for worker in workers {
        let ans = thread::spawn(move || {
            let mut hm = HashMap::<char, usize>::new();
            worker
                .into_iter()
                .for_each(|c| *hm.entry(c).or_insert(0) += 1);
            hm
        });
        handles.push(ans);
    }

    for handle in handles {
        let ans = handle.join().unwrap();
        for (key, value) in ans {
            println!("{} {}", key, value);
            *answer.entry(key).or_insert(0) += value;
        }
    }

    answer
}
