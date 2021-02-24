pub fn nth(n: usize) -> Option<usize> {
    match n {
        0 => None,
        1 => Some(2),
        _ => (3..).step_by(2).filter(is_prime).nth(n - 2),
    }
}

fn is_prime(x: &usize) -> bool {
    match x {
        _ if x < &2 => false,
        _ => {
            let mut i = 2;
            while &(i * i) <= x {
                if x % i == 0 {
                    return false;
                }
                i += 1;
            }
            true
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_second_prime() {
        assert_eq!(nth(2), Some(3));
    }

    #[test]
    fn test_sixth_prime() {
        assert_eq!(nth(6), Some(13));
    }

    #[test]
    fn test_big_prime() {
        assert_eq!(nth(10001), Some(104743));
    }

    #[test]
    fn test_zeroth_prime() {
        assert_eq!(nth(0), None);
    }
}
