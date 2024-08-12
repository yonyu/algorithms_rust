
// Computing the nth Fibonacci number from bottom up.

#[allow(dead_code)]
pub fn compute_nth_fibonacci_number_from_bottom_up(n: u32) -> u64 {
    if n == 0 {
        return 0u64;
    } else if n == 1 {
        return 1u64;
    }

    let mut fib: Vec<u64> = vec![0, 1];
    for i in 2..=n {
        fib.push(fib[(i - 1) as usize] + fib[(i - 2) as usize]);
    }

    fib[n as usize]
}

#[allow(dead_code)]
pub fn compute_nth_fibonacci_number_from_bottom_up_2(n: u32) -> u64 {
    let mut fib1: u64 = 0;
    let mut fib2: u64 = 1;
    let mut fib3: u64 = 0;

    if n == 0 {
        fib1
    } else if n == 1 {
        fib2
    } else {
        for _i in 2..=n {
            fib3 = fib1 + fib2;
            fib1 = fib2;
            fib2 = fib3;
        }
        fib3
    }
}

// Computing the nth Fibonacci number from top down.
// Using a vector to store the Fibonacci numbers.
#[allow(dead_code)]
pub fn compute_nth_fibonacci_number_from_top_down(n: u32) -> u64 {
    let mut fib: Vec<u64> = vec![0; (n + 1) as usize];

    if n == 0 {
        return 0u64;
    } else if n == 1 {
        return 1u64;
    }
    fib[1] = 1;

    compute_nth_fibonacci_number_from_top_down_helper(n, &mut fib)
}

#[allow(dead_code)]
fn compute_nth_fibonacci_number_from_top_down_helper(n: u32, fib: &mut Vec<u64>) -> u64 {
    if n == 0 {
        return 0u64;
    } else if n == 1 {
        return 1u64;
    }

    if fib[n as usize] == 0 {
        fib[n as usize] = compute_nth_fibonacci_number_from_top_down_helper(n - 1, fib) + compute_nth_fibonacci_number_from_top_down_helper(n - 2, fib);
    }

    fib[n as usize]
}

// Computing the nth Fibonacci number using the formula.
#[allow(dead_code)]
pub fn compute_fibonacci_number_using_formula(n: u32) -> u64 {
    let fai: f64 = (1.0 + 5f64.sqrt()) / 2.0;

    ((fai.powi(n as i32)  - (-1f64 / fai).powi(n as i32)) / 5f64.sqrt()) as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_nth_fibonacci_number() {
        let result = compute_nth_fibonacci_number_from_bottom_up(0);
        assert_eq!(result, 0);

        let result = compute_nth_fibonacci_number_from_bottom_up(1);
        assert_eq!(result, 1);

        let result = compute_nth_fibonacci_number_from_bottom_up(2);
        assert_eq!(result, 1);

        let result = compute_nth_fibonacci_number_from_bottom_up(3);
        assert_eq!(result, 2);

        let result = compute_nth_fibonacci_number_from_bottom_up(4);
        assert_eq!(result, 3);

        let result = compute_nth_fibonacci_number_from_bottom_up(5);
        assert_eq!(result, 5);

        let result = compute_nth_fibonacci_number_from_bottom_up(6);
        assert_eq!(result, 8);

        let result = compute_nth_fibonacci_number_from_bottom_up(7);
        assert_eq!(result, 13);

        let result = compute_nth_fibonacci_number_from_bottom_up(8);
        assert_eq!(result, 21);

        let result = compute_nth_fibonacci_number_from_bottom_up(9);
        assert_eq!(result, 34);

        let result = compute_nth_fibonacci_number_from_bottom_up(10);
        assert_eq!(result, 55);

        let result = compute_nth_fibonacci_number_from_bottom_up(11);
        assert_eq!(result, 89);

        let result = compute_nth_fibonacci_number_from_bottom_up(12);
        assert_eq!(result, 144);

        let result = compute_nth_fibonacci_number_from_bottom_up(13);
        assert_eq!(result, 233);

        let result = compute_nth_fibonacci_number_from_bottom_up(14);
        assert_eq!(result, 377);

        let result = compute_nth_fibonacci_number_from_bottom_up(15);
        assert_eq!(result, 610);

        let result = compute_nth_fibonacci_number_from_bottom_up(16);
        assert_eq!(result, 987);

        let result = compute_nth_fibonacci_number_from_bottom_up(17);
        assert_eq!(result, 1597);

        let result = compute_nth_fibonacci_number_from_bottom_up(18);
        assert_eq!(result, 2584);

        let result = compute_nth_fibonacci_number_from_bottom_up(19);
        assert_eq!(result, 4181);

        let result = compute_nth_fibonacci_number_from_bottom_up(20);
        assert_eq!(result, 6765);
    }

    #[test]
    fn test_computing_fibonacci() {
        let result = compute_fibonacci_number_using_formula(0);
        assert_eq!(result, 0);

        let result = compute_fibonacci_number_using_formula(1);
        assert_eq!(result, 1);

        let result = compute_fibonacci_number_using_formula(2);
        assert_eq!(result, 1);

        let result = compute_fibonacci_number_using_formula(3);
        assert_eq!(result, 2);

        let result = compute_fibonacci_number_using_formula(4);
        assert_eq!(result, 3);

        let result = compute_fibonacci_number_using_formula(5);
        assert_eq!(result, 5);

        let result = compute_fibonacci_number_using_formula(6);
        assert_eq!(result, 8);

        let result = compute_fibonacci_number_using_formula(7);
        assert_eq!(result, 13);

        let result = compute_fibonacci_number_using_formula(8);
        assert_eq!(result, 21);

        let result = compute_fibonacci_number_using_formula(9);
        assert_eq!(result, 34);

        let result = compute_fibonacci_number_using_formula(10);
        assert_eq!(result, 55);

        let result = compute_fibonacci_number_using_formula(11);
        assert_eq!(result, 89);

        let result = compute_fibonacci_number_using_formula(12);
        assert_eq!(result, 144);

        let result = compute_fibonacci_number_using_formula(13);
        assert_eq!(result, 233);

        let result = compute_fibonacci_number_using_formula(14);
        assert_eq!(result, 377);

        let result = compute_fibonacci_number_using_formula(15);
        assert_eq!(result, 610);

        let result = compute_fibonacci_number_using_formula(16);
        assert_eq!(result, 987);

        let result = compute_fibonacci_number_using_formula(17);
        assert_eq!(result, 1597);

        let result = compute_fibonacci_number_using_formula(18);
        assert_eq!(result, 2584);

        let result = compute_fibonacci_number_using_formula(19);
        assert_eq!(result, 4181);

        let result = compute_fibonacci_number_using_formula(20);
        assert_eq!(result, 6765);
    }

    #[test]
    fn test_both() {
        assert_eq!(compute_fibonacci_number_using_formula(0), compute_nth_fibonacci_number_from_bottom_up(0));
        assert_eq!(compute_fibonacci_number_using_formula(1), compute_nth_fibonacci_number_from_bottom_up(1));
        assert_eq!(compute_fibonacci_number_using_formula(2), compute_nth_fibonacci_number_from_bottom_up(2));
        assert_eq!(compute_fibonacci_number_using_formula(3), compute_nth_fibonacci_number_from_bottom_up(3));
        assert_eq!(compute_fibonacci_number_using_formula(4), compute_nth_fibonacci_number_from_bottom_up(4));
        assert_eq!(compute_fibonacci_number_using_formula(5), compute_nth_fibonacci_number_from_bottom_up(5));
        assert_eq!(compute_fibonacci_number_using_formula(6), compute_nth_fibonacci_number_from_bottom_up(6));
        assert_eq!(compute_fibonacci_number_using_formula(7), compute_nth_fibonacci_number_from_bottom_up(7));
        assert_eq!(compute_fibonacci_number_using_formula(8), compute_nth_fibonacci_number_from_bottom_up(8));
        assert_eq!(compute_fibonacci_number_using_formula(9), compute_nth_fibonacci_number_from_bottom_up(9));
        assert_eq!(compute_fibonacci_number_using_formula(10), compute_nth_fibonacci_number_from_bottom_up(10));

    }

    #[test]
    fn test_compute_nth_fibonacci_number_from_top_down() {
        let result = compute_nth_fibonacci_number_from_top_down(0);
        assert_eq!(result, 0);

        let result = compute_nth_fibonacci_number_from_top_down(1);
        assert_eq!(result, 1);

        let result = compute_nth_fibonacci_number_from_top_down(2);
        assert_eq!(result, 1);

        let result = compute_nth_fibonacci_number_from_top_down(3);
        assert_eq!(result, 2);

        let result = compute_nth_fibonacci_number_from_top_down(4);
        assert_eq!(result, 3);

        let result = compute_nth_fibonacci_number_from_top_down(5);
        assert_eq!(result, 5);

        let result = compute_nth_fibonacci_number_from_top_down(6);
        assert_eq!(result, 8);

        let result = compute_nth_fibonacci_number_from_top_down(7);
        assert_eq!(result, 13);

        let result = compute_nth_fibonacci_number_from_top_down(8);
        assert_eq!(result, 21);

        let result = compute_nth_fibonacci_number_from_top_down(9);
        assert_eq!(result, 34);

        let result = compute_nth_fibonacci_number_from_top_down(10);
        assert_eq!(result, 55);

        let result = compute_nth_fibonacci_number_from_top_down(11);
        assert_eq!(result, 89);

        let result = compute_nth_fibonacci_number_from_top_down(12);
        assert_eq!(result, 144);

        let result = compute_nth_fibonacci_number_from_top_down(13);
        assert_eq!(result, 233);

        let result = compute_nth_fibonacci_number_from_top_down(14);
        assert_eq!(result, 377);

        let result = compute_nth_fibonacci_number_from_top_down(15);
        assert_eq!(result, 610);

        let result = compute_nth_fibonacci_number_from_top_down(16);
        assert_eq!(result, 987);

        let result = compute_nth_fibonacci_number_from_top_down(17);
        assert_eq!(result, 1597);

        let result = compute_nth_fibonacci_number_from_top_down(18);
        assert_eq!(result, 2584);

        let result = compute_nth_fibonacci_number_from_top_down(19);
        assert_eq!(result, 4181);
    }
}