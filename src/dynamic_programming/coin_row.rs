use std::cmp::max;

// 0132316811, P285
//
// Coin-row problem: There is a row of n coins whose values are some
// positive integers c1, c2, ..., cn, not necessarily distinct.
// The goal is to pick up the maximum amount of money subject to
// the constraint that no two coins adjacent in the initial row can
// be picked up.
//
// Let F(n) be the maximum amount that can be picked up from the row
// of n coins. One group is F(n-2) + cn, the other group is F(n-1).
// the maximum is among F(n-2) + cn and F(n-1).
// Also, F(0) = 0, if no coin, no money.
//       F(1) = c1, if only one coin, pick it up.
#[allow(dead_code)]
pub fn coin_row(coins: Vec<i32>) -> i32 {
    let n = coins.len();
    let mut dp = vec![0; n + 1];
    dp[1] = coins[0];
    for i in 2..=n {
        dp[i] = max(dp[i - 2] + coins[i - 1], dp[i - 1]); // the i-th coin is coins[i-1]
    }

    dp[n]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coin_row() {
        let coins = vec![5, 1, 2, 10, 6, 2];
        assert_eq!(coin_row(coins), 17);

        let coins = vec![1, 2, 3, 4, 5, 1];
        assert_eq!(coin_row(coins), 9);

        let coins = vec![1, 20, 3];
        assert_eq!(coin_row(coins), 20);
    }
}