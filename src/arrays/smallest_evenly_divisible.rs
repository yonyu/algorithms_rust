/*
Project Euler
Problem  5: Smallest multiple

 2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any
 remainder.

 What is the smallest positive number that is evenly divisible by all the numbers from 1 to 20?
*/

#[allow(dead_code)]
pub fn find_smallest_evenly_divisible_number(biggest: i32) -> u64 {
    let mut number = biggest as u64;
    loop {
        if is_divisible(number, biggest) {
            break;
        }
        number += biggest as u64;
    }

    number
}

#[allow(dead_code)]
fn is_divisible(number: u64, biggest: i32) -> bool {
    let mut divisible = true;
    for i in 1..=biggest 
    {
        if number % i as u64 != 0 {
            divisible = false;
            break;
        }

    }

    divisible
}

#[allow(dead_code)]
pub fn find_smallest_evenly_divisible_number_2(biggest: u32) -> u64 {
    // Uses a fold operation to compute the LCM of all numbers from 1 to biggest.
    // Starts with an accumulator (acc) of 1 and updates it with the LCM of the current accumulator 
    // and the next number in the range.
    (1..=biggest).fold(1, |acc, x| lcm(acc, x as u64))
}

/*
   Computes the greatest common divisor using the Euclidean algorithm.
*/
#[allow(dead_code)]
fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 { a } else { gcd(b, a % b) }
}

/*
   Computes the least common multiple using the formula lcm(a, b) = a * (b / gcd(a, b)).
*/
#[allow(dead_code)]
fn lcm(a: u64, b: u64) -> u64 {
    a * (b / gcd(a, b))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_smallest_evenly_divisible_number() {
        // get the start time of the test in seconds
        let start = std::time::Instant::now();
        // call the function being tested
        let result = find_smallest_evenly_divisible_number(20);

        // get the end time of the test in seconds
        let duration = start.elapsed().as_secs_f64();

        println!("Test took: {} seconds", duration);
        assert_eq!(result, 232792560u64);
    }

    #[test]
    fn test_find_smallest_evenly_divisible_number_2() {
        // get the start time of the test in seconds
        let start = std::time::Instant::now();
        // call the function being tested
        let result = find_smallest_evenly_divisible_number_2(20);

        // get the end time of the test in seconds
        let duration = start.elapsed().as_secs_f64();

        println!("Test 2 took: {} seconds", duration);
        assert_eq!(result, 232792560u64);
    }
}