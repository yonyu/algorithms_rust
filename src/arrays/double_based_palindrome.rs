
/*
Porject Euler problem 36

The decimal number, 585 = 1001001001 (binary), is palindromic in both bases.
Find the sum of all numbers, less than n, which are palindromic in base 10 and base 2.

 */

pub fn double_based_palindrome_sum_below_n(n: i32) -> i32 {
    let mut sum = 0;
    for i in 1..n {
        if is_double_based_palindrome(i) {
            sum += i;
        }
    }

    sum
}

fn is_double_based_palindrome(n: i32) -> bool {
    let binary = format!("{:b}", n);
    let decimal = n.to_string();

    decimal == decimal.chars().rev().collect::<String>() && binary == binary.chars().rev().collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_double_based_palindrome_sum_below_n() {
        let result = double_based_palindrome_sum_below_n(1000);
        assert_eq!(result, 1772);

        let result = double_based_palindrome_sum_below_n(1000000);
        assert_eq!(result, 872187);
    }
}