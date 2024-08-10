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

pub fn knapsack(items: &Vec<Item>, capacity: i32) -> (i32, Vec<i32>) {
    let n = items.len();
    let mut dp = vec![vec![0; 1 + capacity as usize]; n+1];

    // for i in 0..=n {
    //     dp[i][0] = 0;
    // }
    //
    // for j in 0..=capacity {
    //     dp[0][j as usize] = 0;
    // }

    for i in 1..=n {
        for j in 1..=capacity {
            if items[i-1].weight > j {
                dp[i][j as usize] = dp[i-1][j as usize];
            } else {
                dp[i][j as usize] = dp[i-1][j as usize].max(items[i-1].value + dp[i-1][j as usize - items[i-1].weight as usize]);
                let t = i as i32;
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
        assert_eq!(result.1, [1, 2, 4])
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
        assert_eq!(result.1, [1, 3])
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
        assert_eq!(result.1, [3, 4])
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
        assert_eq!(result.1, [3, 5])
    }
}
