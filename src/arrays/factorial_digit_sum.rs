/*
Project Euler #20: Factorial Digit Sum

n! means n x (n-1) x ... x 3 x 2 x 1.
For example, 10! = 10 x 9 x ... x 3 x 2 x 1 = 3628800,
and the sum of the digits in the number 10! is 3+6+2+8+8+0+0=29.

Find the sum of the digits in the number 100!
*/
use num_bigint::BigUint;

pub fn factorial_digit_sum(n: u64) -> u64 {
    let fact = factorial(n);
    let sum = sum_of_digits(&fact);
    //println!("The sum of digits in the number {}! is {}", n, sum);
    sum
}

fn sum_of_digits(num: &BigUint) -> u64 {
    num.to_string()
        .chars()
        .filter_map(|c| c.to_digit(10))
        .map(|d| d as u64)
        //.map(|c| c.to_digit(10).unwrap() as u64)
        .sum::<u64>()
}

// helper function to calculate the factorial of n
// using the BigUint type from the num_bigint crate
// to handle large factorials
// (note: this function is not optimized for large numbers)
fn factorial(n: u64) -> BigUint {
    // find the factorial of n
    (1..=n).product()
}

// test cases
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factorial_digit_sum() {
        let result = factorial_digit_sum(100);
        assert_eq!(result, 648u64)
    }
}