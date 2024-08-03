
// 0132316811, P287
// Change-making problem: Consider the general instance of the following
// well-known problem. Give change for amount n using the minimum number
// of coins of denominations d1<d2< ... <dm. For the coin
// denominations used in the United States, as for those used in most if
// not all other countries, there is a very simple and efficient algorithm
// discussed in the next chapter. Here, we consider a dynamic programming
// algorithm for the general case, assuming availability of unlimited
// quantities of coins for each of the m denominations
// d1<d2< ... <dm where d1 = 1.
// Notice dj is ordered, dm is the one with maximum denomination value.
//
// Solution:
// Let F(n) be the minimum number of coins whose values add up to n; it is
// convenient to define F(0) = 0. The amount n can only be obtained by adding one
// coin of denomination dj to the amount n − dj for j = 1, 2, ..., m such that n ≥ dj.
//
// Therefore, we can consider all such denominations and select the one minimizing
// F(n − dj) + 1. Since 1 is a constant, we can, of course, find the smallest F(n − dj)
// first and then add 1 to it. Hence, we have the following recurrence for F(n):
// F(n) = min {F(n − dj)} + 1 for n > 0.
//        j : n≥dj
// F(0) = 0.
// The algorithm is as follows:
// 1. Set F(0) = 0.
// 2. For n = 1, 2, ..., N, compute F(n) using the formula above.
// 3. The final answer is F(N).
// Please write code in Rust. The function should return a vector of tuples, each tuple contains the coin denomination value
// and the corresponding number of coins used, the vector should be sorted by the coin denomination
// value with the biggest denomination value first.
//
// The time complexity of this algorithm is O(Nm), where N is the amount for which
// change is to be made and m is the number of denominations.
// The space complexity is O(N) since we only need to store the values of F(n) for n = 0, 1, ..., N.
//
// The following code implements the change-making problem using dynamic programming.
// The function change_making takes two arguments:
// // n, the amount for which change is to be made,
// and coins, a vector of coin denominations.
// It returns the minimum number of coins needed to make
// change for the amount N using the given coin denominations.

// The function uses a dynamic programming approach to solve the problem by computing the minimum
// number of coins needed for each amount from 0 to N.
// It initializes an array dp of size N + 1 to store the minimum number of coins needed for each amount.
// The base case is dp[0] = 0, as no coins are needed to make change for 0.
// For each amount i from 1 to N, the function computes the minimum number of coins
// needed by iterating over the coin denominations and selecting the one that minimizes the number of coins.
//
// The function returns dp[N], which contains the minimum number of coins needed to make change for the amount N.
pub fn change_making(n: usize, coins: Vec<usize>) -> usize {
    // Initialize the dp array where dp[i] represents the minimum number of coins needed for amount i.
    //                               *****
    // We use std::usize::MAX to signify that initially, no amount can be formed except for amount 0.
    let mut dp = vec![n; n + 1];
    // Base case: 0 coins are needed to make amount 0.
    dp[0] = 0;

    // 'i' is the amount for which change is to be made
    // Iterate over each amount from 1 to n to fill the dp array.
    for i in 1..=n {
        // Iterate over each coin denomination.
        for &coin in &coins {
            // If the current amount i is at least as large as the coin value, update temp.
            if i >= coin {
                // Update temp to be the minimum of its current value and dp[i - coin].
                // This step finds the minimum number of coins needed by including one more coin of the current denomination.
                dp[i] = dp[i].min(dp[i - coin] + 1);  // Adding 1 accounts for the current coin used to form the amount i.
            }
        }
    }
    // Return the minimum number of coins needed to make the amount n.
    // If no solution is found, dp[n] will remain std::usize::MAX.
    dp[n]
}

// The problem is the same as the above.
// But we want to know all the coin denominations used together with the corresponding count.
// The function should return a vector of tuples, each tuple contains the coin denomination value
// and the corresponding count of coins used, the vector should be sorted by the coin denomination
// value with the biggest denomination value at the beginning.
#[allow(dead_code)]
pub fn change_making_with_coins(n: usize, coins: Vec<usize>) -> Vec<(usize, usize)> {
    // Initialize the dp array where dp[i] represents the minimum number of coins needed for amount i.
    // We use std::usize::MAX to signify that initially, no amount can be formed except for amount 0.
    let mut dp = vec![usize::MAX; n + 1]; //vec![n; n + 1];
    // Initialize the coins_used array to keep track of the coins used for each amount.
    // coins_used[i] is a vector is a vector too and stores the coins used to form the amount i.
    let mut coins_used = vec![vec![]; n + 1];

    // Base case: 0 coins are needed to make amount 0.
    dp[0] = 0;

    // 'i' is the change amount to be made.
    // Iterate over each amount from 1 to n to fill the dp array.
    for i in 1..=n {
        // Iterate over each coin denomination.
        for &coin in &coins {
            // If the current amount 'i' is at least as large as the coin value, and dp[i] is
            // greater than dp[i - coin] + 1, then update dp[i] with dp[i - coin] + 1.
            if i >= coin && dp[i] > dp[i - coin] + 1 {
                dp[i] = dp[i - coin] + 1;
                // coins_used[i] (the coins used to form the amount i) consists of the coins used
                // to form the amount i - coin plus the current coin.
                coins_used[i] = coins_used[i - coin].clone(); // Copy the coins used for amount i - coin.
                // Add the current coin to the coins_used[i] vector.
                coins_used[i].push(coin); //
            }
        }
    }

    // If no solution is found, return an empty vector.
    if dp[n] == usize::MAX {
        return vec![];
    }

    // Count the occurrences of each coin.
    // coin_count is a vector that stores the unique coins used to form the amount n.
    let mut coin_count = vec![];
    for &coin in &coins_used[n] {
        // Check if the coin is already in the coin_count vector.
        let count = coin_count.iter().filter(|&&c| c == coin).count();
        // If the coin is not in the coin_count vector, add it.
        if count == 0 {
            coin_count.push(coin);
        }
    }

    // Convert the coin_count to a sorted vector of tuples.
    let mut result: Vec<(usize, usize)> = coin_count.into_iter().map(|coin| (coin, coins_used[n].iter().filter(|&&c| c == coin).count())).collect();
    result.sort_by(|a, b| b.0.cmp(&a.0)); // Sort the result by coin denomination in descending order.

    result
}

#[allow(dead_code)]
fn change_making_with_coins_2(amount: usize, denominations: Vec<usize>) -> Vec<(usize, usize)> {
    // Initialize the dp array where dp[i] represents the minimum number of coins for amount i
    let mut dp = vec![std::usize::MAX; amount + 1];
    dp[0] = 0; // Base case: 0 coins are needed to make amount 0

    // Array to store the last coin used to make the amount
    let mut last_coin = vec![0; amount + 1];

    // Fill the dp array
    for &coin in denominations.iter() {
        for i in coin..=amount {
            // If using the current coin results in fewer coins, update dp[i] and record the coin
            if dp[i - coin] + 1 < dp[i] {
                dp[i] = dp[i - coin] + 1;
                last_coin[i] = coin;
            }
        }
    }

    // If no solution is found, return an empty vector
    if dp[amount] == std::usize::MAX {
        return vec![];
    }

    // Reconstruct the solution
    let mut remaining_amount = amount;
    let mut result = vec![];

    // Backtrack from the target amount to find the coins used
    while remaining_amount > 0 {
        let coin = last_coin[remaining_amount];
        // If the coin is already in the result, increment its count
        if let Some((_val, count)) = result.iter_mut().find(|(val, _)| *val == coin) {
            *count += 1;
        } else {
            // Otherwise, add the coin to the result with count 1
            result.push((coin, 1));
        }
        remaining_amount -= coin;
    }

    // Sort the result by coin denomination in descending order
    result.sort_by(|a, b| b.0.cmp(&a.0));

    result
}


// The following code provides an example of using the change_making function to find the minimum number
// of coins needed to make change for the amount 11 using coin denominations [1, 3, 4].
// The expected output is 3, as the minimum number of coins needed to make change for 11 using the coin
// denominations [1, 3, 4] is 3 (one 4-cent coin and two 3-cent coins).
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_change_making() {
        assert_eq!(change_making(11, vec![1, 3, 4]), 3);
        assert_eq!(change_making(6, vec![1, 3, 4]), 2);
        assert_eq!(change_making(8, vec![1, 3, 4]), 2);
        assert_eq!(change_making(10, vec![1, 3, 4]), 3);
    }

    #[test]
    fn test_coin_change_with_coins() {
        assert_eq!(change_making_with_coins(11, vec![1, 3, 4]), vec![(4, 2), (3, 1)]);
        assert_eq!(change_making_with_coins(6, vec![1, 3, 4]), vec![(3, 2)]);
        assert_eq!(change_making_with_coins(8, vec![1, 3, 4]), vec![(4, 2)]);
        assert_eq!(change_making_with_coins(10, vec![1, 3, 4]), vec![(4, 1), (3, 2)]);
        assert_eq!(change_making_with_coins(0, vec![1, 3, 4]), vec![]);

        assert_eq!(change_making_with_coins(63, vec![1, 5, 10, 25]), vec![(25, 2), (10, 1), (1, 3)]);
        assert_eq!(change_making_with_coins(100, vec![1, 5, 10, 25]), vec![(25, 4)]);
        assert_eq!(change_making_with_coins(30, vec![1, 5, 10, 25]), vec![(25, 1), (5, 1)]);

        // Greedy algorithm will fail for this case.
        assert_eq!(change_making_with_coins(30, vec![1, 10, 25]), vec![(10, 3)]);
    }
}