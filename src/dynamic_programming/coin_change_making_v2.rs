use std::cmp;
use std::collections::HashMap;

// The coin change problem is a classic dynamic programming problem. 
// The key insight:
// 
// We have coins with denominations d₁ < d₂ < ... < dₘ
// We want to make a target amount using the minimum number of coins
// We have unlimited coins of each denomination

/// Solves the coin change problem using dynamic programming
/// 
/// Given a target amount and a list of coin denominations, finds the minimum
/// number of coins needed to make the target amount.
/// 
/// # Arguments
/// * `coins` - A slice of coin denominations (assumed to be sorted in ascending order)
/// * `amount` - The target amount to make change for
/// 
/// # Returns
/// * `Some(count)` - The minimum number of coins needed
/// * `None` - If it's impossible to make the target amount
/// 
/// # Intuition:
/// We use dynamic programming with the following recurrence relation:
/// dp[i] = min(dp[i], dp[i - coin] + 1) for each coin where coin <= i
/// 
/// This means: to make amount i, we can use any coin and then make the remaining
/// amount (i - coin) optimally.
pub fn min_coins(coins: &[u32], amount: u32) -> Option<u32> {
    // dp[i] represents the minimum coins needed to make amount i
    // Initialize with a value larger than any possible answer
    // The maximum coins needed is amount (using all 1-denomination coins)
    let mut dp:Vec<u32> = vec![amount+1; amount as usize + 1];
    // Base case: 0 coins needed to make amount 0
    dp[0] = 0;

    // For each amount from 1 to target amount
    for i in 1..=amount {
        // Try each coin denomination
        for &coin in coins {
            // If this coin value is greater than current amount, skip it
            // (and since coins are sorted, all remaining coins will be larger too)
            if coin > i {
                break;
            }
            // If we can make (i - coin) amount, then we can make i amount
            // by using this coin plus the optimal solution for (i - coin)
            dp[i as usize] = cmp::min(dp[i as usize], dp[(i-coin) as usize] + 1)
        }
    }

    // If dp[amount] is still the initial value, it means we couldn't make the amount
    if dp[amount as usize] == amount + 1 {
        None
    } else {
        Some(dp[amount as usize])
    }
}


/// Solves change make and also returns coins with their counts using backtracking
/// 
/// This function builds on the DP solution but uses backtracking to reconstruct
/// the optimal solution and count how many times each coin denomination is used.
/// 
/// # Arguments
/// * `coins` - A slice of coin denominations (assumed to be sorted)
/// * `amount` - The target amount to make change for
/// 
/// # Returns
/// * `Some((total_coins, coin_counts))` - Where coin_counts is a vector of (coin_value, count) tuples
/// * `None` - If it's impossible to make the target amount
/// 
/// # Algorithm:
/// 1. First, use DP to find if a solution exists and build the parent tracking table
/// 2. Then, backtrack from the target amount to reconstruct the solution
/// 3. Count occurrences of each coin denomination
/// 4. Return the counts as (coin_value, count) pairs
pub fn min_coins_with_counts(coins: &[u32], amount: u32) -> Option<(u32, Vec<(u32, u32)>)> {
    // dp[i] represents the minimum coins needed to make amount i
    // Initialize with a value larger than any possible answer
    // The maximum coins needed is amount (using all 1-denomination coins)
    let mut dp:Vec<u32> = vec![amount + 1; amount as usize + 1];
    let mut parent = vec![0; amount as usize + 1];
    // Base case: 0 coins needed to make amount 0
    dp[0] = 0;

    // For each amount from 1 to target amount
    for i in 1..=amount {
        // Try each coin denomination
        for &coin in coins {
            // If this coin value is greater than current amount, skip it
            // (and since coins are sorted, all remaining coins will be larger too)
            if coin > i {
                break; // Since coins are sorted, no need to check larger coins
            }
            // If we can make (i - coin) amount, then we can make i amount
            // by using this coin plus the optimal solution for (i - coin)
            if dp[(i-coin) as usize] != amount + 1 {
                if dp[(i-coin) as usize] + 1 < dp[i as usize] {
                    dp[i as usize] = dp[(i-coin) as usize] + 1;
                    parent[i as usize] = coin;
                }
            }
        }
    }

    // Backtrack to reconstruct the solution
    let mut result_coins = HashMap::new();
    let mut current = amount;

    // Trace back through the parent array to find which coins were used
    while current > 0 {
        let coin = parent[current as usize];
        // Increment the count for this coin denomination
        *result_coins.entry(coin).or_insert(0) += 1;

        // Move to the remaining amount after using this coin
        current -= coin;
    }
    // Convert HashMap to sorted vector of (coin_value, count) tuples
    let mut coins_counts: Vec<(u32, u32)> = result_coins.into_iter().collect();

    // Sort by coin value for consistent output
    coins_counts.sort_by_key(|&(coin_value, _coin_count)| coin_value);

    // If dp[amount] is still the initial value, it means we couldn't make the amount
    if dp[amount as usize] == amount + 1 {
        None
    } else {
        Some((dp[amount as usize], coins_counts))
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_coins() {
        // Greedy algorithm will fail: 4+1+1
        let coins = vec![1, 3, 4];
        let actual = min_coins(&coins, 6);
        assert_eq!(actual, Some(2));
    }

    #[test]
    fn test_min_coins_2() {
        let coins = vec![1, 5, 10, 25];
        let actual = min_coins(&coins, 67);
        assert_eq!(actual, Some(6));
    }

    #[test]
    fn test_min_coins_with_counts() {
        let coins = vec![1, 3, 4];
        let amount = 6;
        let actual = min_coins_with_counts(&coins, amount);

        match actual {
            Some((count, coin_counts)) => {
                assert_eq!(count, 2);
                assert_eq!(coin_counts, [(3, 2)]);
            },
            None => {} 
        }
    }

    #[test]
    fn test_min_coins_with_counts_2() {
        let coins = vec![1, 5, 10, 25];
        let amount = 67;
        let result = min_coins_with_counts(&coins, amount);

        match result {
            Some((count, coin_counts)) => {
                assert_eq!(count, 6);
                assert_eq!(coin_counts, [(1, 2), (5, 1), (10, 1), (25, 2)]);
            },
            None => {}
        }
    }
}