/*
Project Euler:
Problem 3: Largest prime factor

The prime factors of 13195 are 5, 7, 13 and 29.

What is the largest prime factor of the number 600851475143? Answer is 6857.
*/

#[allow(dead_code)]
pub fn find_largest_prime_factor(n: u64) -> u64 {
    let mut result: u64 = 0;
    let mut factor: u64 = 2;
    while factor * factor <= n {
        if n % factor == 0 && is_prime(factor) { // super::largest_prime_factor::
            result = factor;
        }
        factor += 1;
    }

    result
}

#[allow(dead_code)]
fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    } else if n == 2 {
        return true;
    } else {
        let mut i: u64 = 2;
        while i * i <= n {
            if n % i == 0 {
                return false;
            }
            i += 1;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_find_largest_prime_factor() {
        let factor = find_largest_prime_factor(600851475143) as i32;
        assert_eq!(factor, 6857);
    }
}