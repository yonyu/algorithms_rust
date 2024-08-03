/*
Project Euler:
Problem 4: Largest palindrome product

A palindromic number reads the same both ways. The largest palindrome made from the product of two
2-digit numbers is 9009 = 91 x 99.

Find the largest palindrome made from the product of two 3-digit numbers.
Answer: 906609
*/

pub fn find_largest_palindrome_product() -> u64 {
    let mut largest : u64 = 0;
    for i in 100..=999 {
        for  j in i..=999  {
            let p: u64 = i * j;
            if p > largest && is_palindrome(p) {
                largest = p;
            }
        }
    }

    largest
}

fn is_palindrome(n: u64) -> bool {
    let s = n.to_string();
    let rev = s.chars().rev().collect::<String>();

    s == rev
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_largest_palindrome_product() {
        // get a time value for the test
        let start = std::time::Instant::now();

        let result = find_largest_palindrome_product();
        // get a time when the test ends
        let duration = start.elapsed();
        println!("Test took: {:?}", duration);
        assert_eq!(result, 906609);
    }
}