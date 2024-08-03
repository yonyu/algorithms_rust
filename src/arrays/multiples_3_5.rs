/* https://projecteuler.net/problem=1
 * If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9.
 * The sum of these multiples is 23.
 *
 * Find the sum of all the multiples of 3 or 5 below 1000.
 */
pub fn sum_multiples_3_5_below_1000() -> i32 {
    let mut sum : i32 = 0;

    for i in 1..1000 {
        if i % 3 == 0 || i % 5 == 0 {
            sum += i;
        }
    }

    sum
}

fn sum_multiples_3_5_below_1000_functional() -> i32 {
    let sum = (1..1000)
        .filter(|&x| x % 3 == 0 || x % 5 == 0)
        .sum();

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_multiples_3_5_below_1000() {
        let result = sum_multiples_3_5_below_1000();
        assert_eq!(result, 233168);
    }

    #[test]
    fn test_sum_multiples_3_5_below_1000_functional() {
        let result = sum_multiples_3_5_below_1000_functional();
        assert_eq!(result, 233168);
    }

}
