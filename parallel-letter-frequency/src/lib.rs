use crossbeam::thread;
use std::collections::HashMap;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let mut answer: HashMap<char, usize> = HashMap::new();
    let length = input.len();
    if length == 0 {
        return answer;
    }

    let cap = length.div_euclid(worker_count);
    let mut rem = length.rem_euclid(worker_count);
    let mut workers = Vec::with_capacity(worker_count);
    let mut iter = input.iter();
    for _ in 0..worker_count {
        let chunk;
        if rem != 0 {
            chunk = iter.by_ref().take(cap + 1).collect::<Vec<_>>();
            rem -= 1;
        } else {
            chunk = iter.by_ref().take(cap).collect::<Vec<_>>();
        }
        workers.push(chunk);
    }

    let mut handles = vec![];
    for worker in workers {
        let ans = thread::scope(|_| {
            let mut hm = HashMap::<char, usize>::new();
            worker.into_iter().for_each(|w| {
                w.chars().into_iter().for_each(|c| {
                    if c.is_alphabetic() {
                        let c = c.to_ascii_lowercase();
                        *hm.entry(c).or_insert(0) += 1;
                    }
                });
            });
            hm
        });
        handles.push(ans);
    }

    for handle in handles {
        let ans = handle.unwrap();
        for (key, value) in ans {
            println!("{} {}", key, value);
            *answer.entry(key).or_insert(0) += value;
        }
    }

    answer
}
