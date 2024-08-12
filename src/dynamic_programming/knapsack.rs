/*
isbn 978-0-13-231681-1
Knapsack Problem
The knapsack problem is a problem in combinatorial optimization. Given a set of items, each with a
weight and a value, determine the number of each item to include in a collection so that the total
weight is less than or equal to a given limit and the total value is as large as possible.

Suppose we have a knapsack with a weight limit of W and a set of n items, each with a weight w_i
and a value v_i. The goal is to maximize the total value of the items in the knapsack without
exceeding the weight limit.
 */

pub struct Item {
    pub weight: i32,
    pub value: i32,
}

#[allow(dead_code)]
pub fn knapsack(items: &Vec<Item>, capacity: i32) -> (i32, Vec<i32>) {
    let n = items.len();
    let mut dp = vec![vec![0; 1 + capacity as usize]; n+1];

    for i in 1..=n {
        for j in 1..=capacity {
            if items[i-1].weight > j {
                dp[i][j as usize] = dp[i-1][j as usize];
            } else {
                dp[i][j as usize] = dp[i-1][j as usize].max(items[i-1].value + dp[i-1][j as usize - items[i-1].weight as usize]);
            }
        }
    }

    // backtrack all items included in the knapsack, starting from the last item
    let mut packed: Vec<i32> = Vec::new();
    let mut j = capacity as usize;
    let mut i = n;
    while i > 0 {
        if dp[i][j] != dp[i-1][j] {
            packed.push(i as i32);
            j -= items[i-1].weight as usize;
        }
        i -= 1;
    }

    packed.reverse();
    (dp[n][capacity as usize], packed)
}

// implement knapsack with memoization, that is, top-down dynamic programming
#[allow(dead_code)]
pub fn knapsack_memoization(items: &Vec<Item>, capacity: i32) -> (i32, Vec<i32>) {
    let n = items.len();
    let mut memo = vec![vec![-1; capacity as usize + 1]; n + 1];

    // Initializing the memo table to 0 for i=0 and j=0 is not required
    // because it is computed in the base case of the recursive function.
    // for i in 0..=n {
    //     memo[i][0] = 0;
    // }

    // for j in 1..=capacity {
    //     memo[0][j as usize] = 0;
    // }

    fn knapsack_memoization_helper(items: &Vec<Item>, i: usize, j: usize, memo: &mut Vec<Vec<i32>>) -> i32 {
        // base case
        if i == 0 || j == 0 {
            return 0;
        }

        // if the value is already computed, return it
        if memo[i][j] != -1 {
            return memo[i][j];
        }

        // compute the optimal value without the ith item
        let mut max1 = knapsack_memoization_helper(items, i - 1, j, memo);

        // if the weight of the current item is less than or equal to the capacity, include it
        if j >= items[i - 1].weight as usize {
            let max2 = knapsack_memoization_helper(items, i - 1, j - items[i - 1].weight as usize, memo);
            max1 = max1.max(max2 + items[i - 1].value);
        }

        // store the result in the memo table
        memo[i][j] = max1;

        memo[i][j]
    }

    let max_value: i32 = knapsack_memoization_helper(&items, n, capacity as usize, &mut memo);

    // backtrack all items included in the knapsack, starting from the last item
    let mut packed: Vec<i32> = Vec::new();
    let mut i = n;
    let mut j = capacity as usize;
    while i > 0 {
        if memo[i][j] != memo[i - 1][j] {
            packed.push(i as i32);
            j -= items[i - 1].weight as usize;
        }
        i -= 1;
    }

    packed.reverse();
    (max_value, packed)
}

// implement knapsack with memoization, that is, top-down dynamic programming
#[allow(dead_code)]
pub fn knapsack_memoization_2(items: &Vec<Item>, capacity: i32) -> (i32, Vec<i32>) {
    let n = items.len();
    let mut memo = vec![vec![-1; 1 + capacity as usize]; n+1];

    fn knapsack_memoization_helper(items: &Vec<Item>, capacity: i32, n: usize, memo: &mut Vec<Vec<i32>>) -> i32 {
        if n == 0 || capacity == 0 {
            return 0;
        }

        // if the value is already computed, return it
        if memo[n][capacity as usize] != -1 {
            return memo[n][capacity as usize];
        }

        if items[n-1].weight > capacity {
            memo[n][capacity as usize] = knapsack_memoization_helper(items, capacity, n-1, memo);
            return memo[n][capacity as usize];
        }

        let include = items[n-1].value + knapsack_memoization_helper(items, capacity - items[n-1].weight, n-1, memo);
        let exclude = knapsack_memoization_helper(items, capacity, n-1, memo);
        memo[n][capacity as usize] = include.max(exclude);
        memo[n][capacity as usize]
    }

    let max_value = knapsack_memoization_helper(items, capacity, n, &mut memo);

    // backtrack all items included in the knapsack, starting from the last item
    let mut packed: Vec<i32> = Vec::new();
    let mut j = capacity as usize;
    let mut i = n;
    while i > 0 {
        if memo[i][j] != memo[i-1][j] {
            packed.push(i as i32);
            j -= items[i-1].weight as usize;
        }
        i -= 1;
    }

    packed.reverse();
    (max_value, packed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_knapsack() {
        let mut items: Vec<Item> = Vec::new();

        let item = Item {
            weight: 2,
            value: 12,
        };
        items.push(item);

        let item = Item {
            weight: 1,
            value: 10,
        };
        items.push(item);

        let item = Item {
            weight: 3,
            value: 20,
        };
        items.push(item);

        let item = Item {
            weight: 2,
            value: 15,
        };
        items.push(item);

        let capacity = 5;

        let result = knapsack(&items, capacity);
        assert_eq!(result.0, 37);
        assert_eq!(result.1, [1, 2, 4]);

        let result = knapsack_memoization(&items, capacity);
        assert_eq!(result.0, 37);
        assert_eq!(result.1, [1, 2, 4]);
    }

    #[test]
    fn test_knapsack_2() {
        let mut items: Vec<Item> = Vec::new();

        // Guitar
        let item = Item {
            weight: 1,
            value: 1500,
        };
        items.push(item);

        // Stereo
        let item = Item {
            weight: 4,
            value: 3000,
        };
        items.push(item);

        // Laptop
        let item = Item {
            weight: 3,
            value: 2000,
        };
        items.push(item);

        let capacity = 4;

        let result = knapsack(&items, capacity);
        assert_eq!(result.0, 3500);
        assert_eq!(result.1, [1, 3]);

        let result = knapsack_memoization(&items, capacity);
        assert_eq!(result.0, 3500);
        assert_eq!(result.1, [1, 3]);
    }

    // grokking algorithms, 2nd Edition, page 215, isbn 9781633438538
    #[test]
    fn test_knapsack_3() {
        let mut items: Vec<Item> = Vec::new();

        // Guitar
        let item = Item {
            weight: 1,
            value: 1500,
        };
        items.push(item);

        // Stereo
        let item = Item {
            weight: 4,
            value: 3000,
        };
        items.push(item);

        // Laptop
        let item = Item {
            weight: 3,
            value: 2000,
        };
        items.push(item);

        // iPhone
        let item = Item {
            weight: 1,
            value: 2000,
        };
        items.push(item);

        let capacity = 4;

        let result = knapsack(&items, capacity);
        assert_eq!(result.0, 4000);
        assert_eq!(result.1, [3, 4]);

        let result = knapsack_memoization(&items, capacity);
        assert_eq!(result.0, 4000);
        assert_eq!(result.1, [3, 4]);
    }

    //exercise 8.2, page 296, isbn 978-0-13-231681-1
    #[test]
    fn test_knapsack_4() {
        let mut items: Vec<Item> = Vec::new();

        let item = Item {
            weight: 3,
            value: 25,
        };
        items.push(item);

        let item = Item {
            weight: 2,
            value: 20,
        };
        items.push(item);

        let item = Item {
            weight: 1,
            value: 15,
        };
        items.push(item);

        let item = Item {
            weight: 4,
            value: 40,
        };
        items.push(item);

        let item = Item {
            weight: 5,
            value: 50,
        };
        items.push(item);

        let capacity = 6;

        let result = knapsack(&items, capacity);
        assert_eq!(result.0, 65);
        assert_eq!(result.1, [3, 5]);

        let result = knapsack_memoization(&items, capacity);
        assert_eq!(result.0, 65);
        assert_eq!(result.1, [3, 5]);
    }
}
