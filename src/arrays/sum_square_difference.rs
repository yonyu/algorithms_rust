/*
Project Euler
Problem 6: Sum square difference

The sum of the squares of the first ten natural numbers is,
 1^2 + 2^2 + ... + 10^2 = 385

The square of the sum of the first ten natural numbers is,

( 1 + 2 + ... + 10)^2 = 55^2 = 3025

Hence the difference between the sum of the squares of the first ten natural numbers and the square of the sum is
3025 - 385 = 2640
.

Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.
 */

#[allow(dead_code)]
fn sum_square_difference() -> u64 {
    // let mut sum1: i64 = 0;
    // for i  in 1i64..=100 {
    //     sum1 += i * i;
    // }

    // let mut sum2: i64 = 0;
    // for i in 1i64..=100 {
    //     sum2 += i;
    // }
    // // sum2 = 100 * 101 / 2;
    // sum2 = sum2 * sum2;
    let n = 100;

    let sum1: u64 = (1..=n).map(|x| x * x).sum();
    let sum2: u64 = (1..=n).sum::<u64>().pow(2);

    sum2 - sum1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_square_difference() {
        let result = sum_square_difference();
        assert_eq!(result, 25164150u64)
    }
}