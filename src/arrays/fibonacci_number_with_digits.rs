/*
 Project Euler
 Problem 25:
  1000-digit Fibonacci number
  The Fibonacci sequence is defined by the recurrence relation:

  Fn = Fn−1 + Fn−2, where F1 = 1 and F2 = 1.
  Hence the first 12 terms will be:

  F1 = 1
  F2 = 1
  F3 = 2
  F4 = 3
  F5 = 5
  F6 = 8
  F7 = 13
  F8 = 21
  F9 = 34
  F10 = 55
  F11 = 89
  F12 = 144

  The 12th term, F12, is the first term to contain three digits.

  What is the index of the first term in the Fibonacci sequence to contain 1000 digits?

  Write the code in Rust, pay attention to overflow.
 */
use num_bigint::BigUint;
use num_traits::One;

#[allow(dead_code)]
pub fn fibonacci_number_with_specified_digits(n: u32) -> u32 {
    let mut fib1: BigUint = BigUint::one();
    let mut fib2: BigUint = BigUint::one();
    let mut index: u32 = 2;

    loop {
        let next_fib = fib1.clone() + fib2.clone();
        fib1 = fib2;
        fib2 = next_fib;
        index += 1;

        if fib2.to_string().len() >= n as usize {
            return index;
        }
    }
}

#[allow(dead_code)]
pub fn fibonacci_number_with_specified_digits_2(n: u32) -> u32 {
    // Initialize the first two Fibonacci numbers
    let mut fib1: BigUint = BigUint::from(1u64);
    let mut fib2: BigUint = One::one();
    // Start the index at 2 since we already have the first two Fibonacci numbers
    let mut index: u32 = 2;
    // Compute the Fibonacci sequence until the desired number of digits is reached
    loop {
        let temp = fib2.clone();
        fib2 = fib2 + &fib1;
        fib1 = temp;
        index += 1;
        if fib2.to_string().len() >= n as usize {
            break;
        }
    }
    //println!("The index of the first term with 1000 digits is: {}", index);

    index
}

/*
 Output:
 The index of the first term with 1000 digits is: 4782
 Explanation:
 •  Dependencies:
 •	num-bigint provides the BigUint type for big integer operations.
 •	num-traits provides the One trait for easy initialization of BigUint to 1.
 •  Initialization:
 •	Initialize fib1 and fib2 to the first two Fibonacci numbers, both set to 1.
 •	Start the index at 2 since we already have the first two Fibonacci numbers.
 •  Fibonacci Calculation:
 •	In a loop, continue computing the next Fibonacci number by summing the previous two.
 •	Swap the values of fib1 and fib2 accordingly.
 •	Increment the index on each iteration.
 •  Digit Count Check:
 •	Convert fib2 to a string and check its length to see if it has reached 1000 digits.
 •  Output:
 •	Once a term with 1000 digits is found, return its index, which is 4782.

 •	This solution has a time complexity of O(n), where n is the index of the first term with 1000
 •	digits, as it iterates through the Fibonacci sequence until the desired condition is met.
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci_number_with_digits() {
        assert_eq!(fibonacci_number_with_specified_digits(1000), 4782);
    }
}
