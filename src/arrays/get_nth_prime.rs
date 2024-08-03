/*
Project Euler: #7 10001st Prime

By listing the first six prime numbers: 2, 3, ,5, 7, 11 and 13, we can see that the 6th prime is 13.

What is the 10001st prime number?
*/

pub fn get_nth_prime() -> i32 {
    let limit = 200_000; // Estimate, can be adjusted
    let primes = sieve(limit);
    println!("{}", primes[10000]); // 0-indexed, so 10000 is the 10001st prime

    primes[10000] as i32
}

/*
Time Complexity of Sieve of Eratosthenes
The Sieve of Eratosthenes has a time complexity of 𝑂(𝑛 log log 𝑛), which is significantly 
more efficient for generating large numbers of primes.
*/
fn sieve(limit: usize) -> Vec<usize> {
    let mut sieve = vec![true; limit];
    sieve[0] = false; // 0 is not a prime number
    sieve[1] = false; // 1 is not a prime number
    for i in 2..((limit as f64).sqrt() as usize + 1) {
        if sieve[i] {
            for j in (i*i..limit).step_by(i) {
                sieve[j] = false;
            }
        }
    }
    sieve.iter()
         .enumerate()
         .filter_map(|(i, &is_prime)| if is_prime { Some(i) } else { None })
         .collect()
}

/**
  To find the 10,001st prime number by checking each odd number for primality against previously 
  found primes. 
  
  Let's analyze its time complexity and consider possible optimizations.

Time Complexity Analysis
1. Outer loop (while primes.len() < 10001):

Runs until we have 10,001 primes.
In the worst case, this means iterating through approximately 
𝑛 log 𝑛 numbers, where n is roughly the 10,001st prime.

2. Inner loop (primes.iter().all(|p| n % p != 0)):

For each candidate number  𝑛, checks divisibility by all previously found primes.
On average, this involves checking about sqrt(𝑛) primes due to the properties of prime distribution.

Overall, the time complexity is influenced by both the number of primes and the cost of checking each 
candidate. This results in an average complexity of approximately 𝑂(n * sqrt(n)).
*/
pub fn get_nth_prime_2() -> i32 {
    let mut primes = vec![2];
    let mut n = 3;
    while primes.len() < 10001 {
        if primes.iter().all(|p| n % p != 0) {
            primes.push(n);
        }
        n += 2;
    }
    println!("{}", primes.last().unwrap());

    primes[10000]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_nth_prime() {
        let result: i32 = get_nth_prime();
        assert_eq!(result, 104743);
    }

    #[test]
    fn test_get_nth_prime_2() {
        let result = get_nth_prime_2();
        assert_eq!(result, 104743);
    }
}