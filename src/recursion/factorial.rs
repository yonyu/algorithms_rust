///
/// calculate factorial with recursion.
/// this concept proof only, not efficient.
pub fn factorial(n: usize) -> usize {
    // base case
    if n == 0 {
        return 1;
    }

    // recursive case
    return n * factorial(n - 1);
}

pub fn factorial_recursive_cache(n: usize, cache: &mut Vec<usize>) -> usize {
    //let mut cache = vec![0usize; n + 1];
    // cache[0] = 1;
    // cache[1] = 1;

    if n == 0 || n == 1 {
        return 1;
    }

    if cache.len() <= n{
        cache.resize(n * 2, 0);
    }
    
    if cache[n] != 0 {
        return cache[n];
    }
    
    cache[n] = n * factorial_recursive_cache(n - 1, cache);

    cache[n]

}

pub fn factorial_iterative(n: usize) -> usize {
    if n == 0 {
        return 1;
    }
    let mut fact = 1;
    for i in 1..=n {
        fact *= i;
    }

    return fact;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_factorial() {
        assert_eq!(factorial(0), 1);
        assert_eq!(factorial(1), 1);
        assert_eq!(factorial(2), 2);

        assert_eq!(factorial(5), 120);
    }

    #[test]
    fn test_factorial_recursive_cache() {
        let mut cache = vec![0usize; 10];
        cache[0] = 1;
        cache[1] = 1;

        assert_eq!(factorial_recursive_cache(0, &mut cache), 1);
        assert_eq!(factorial_recursive_cache(1, &mut cache), 1);
        assert_eq!(factorial_recursive_cache(2, &mut cache), 2);

        assert_eq!(factorial_recursive_cache(5, &mut cache), 120);
        assert_eq!(factorial_recursive_cache(6, &mut cache), 720);
    }

    #[test]
    fn test_factorial_iterative() {
        assert_eq!(factorial(0), 1);
        assert_eq!(factorial(1), 1);
        assert_eq!(factorial(2), 2);

        assert_eq!(factorial(5), 120);
    }
}