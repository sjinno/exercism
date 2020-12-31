use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let length = factors.len();
    if length == 0 {
        return 0;
    }

    let mut visited_nums: HashSet<u32> = HashSet::new();
    let mut sum = 0;
    for num in factors {
        if num == &0 {
            continue;
        }
        let mut n = *num;
        while n < limit {
            if !visited_nums.contains(&n) {
                visited_nums.insert(n);
                sum += n;
            }
            n += num;
        }
    }
    sum
}
