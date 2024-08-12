
/*
Minimum-sum descent Some positive integers are arranged in an equilateral
triangle with n numbers in its base like the one shown in the figure below for
n = 4. The problem is to find the smallest sum in a descent from the triangle
apex to its base through a sequence of adjacent numbers (shown in the figure
by the circles). Design a dynamic programming algorithm for this problem
and indicate its time efficiency.

Find the path with the minimum sum in a descent from the apex to the base
with dynamic programming.

The function signature for the algorithm is as follows:
fn calculate_minimum_sum_descent(nums: Vec<i32>) -> Vec<i32>


Example:

                    2

                  5   4

               1   4    7

            8    6    9    6

The minimum sum in a descent from the apex to the base is 2 + 5 + 1 + 6 = 14.
And therefore, the return value is [2, 5, 1, 6].
 */

/*
Explanation:
1.	Triangle Representation: The triangle is represented as a vector of vectors, where each inner
    vector represents a row of the triangle.
2.	Dynamic Programming Approach:
o	We maintain a DP table dp where dp[i][j] stores the minimum sum to reach element (i, j) from
    the apex.
o	The base case is dp[0][0] = triangle[0][0].
o	The recurrence relation is:
    dp[i][j] = triangle[i][j]+min(dp[i−1][j−1],dp[i−1][j])
    for 0 < j < i < n.
o	After filling out the DP table, we can backtrack from the minimum value in the last row to find
    the path.
3.	Time Complexity: The time complexity of this approach is O(n^2), where n is the number of rows
    in the triangle, since we need to compute the minimum sum for each element in the triangle.
*/
fn calculate_minimum_sum_descent(nums: Vec<i32>) -> Vec<i32> {
    let mut triangle = Vec::new();
    let mut row = 1;
    let mut index = 0;

    // Convert the flat vector into a triangle representation
    while index < nums.len() {
        triangle.push(nums[index..index + row].to_vec());
        index += row;
        row += 1;
    }

    let n = triangle.len();
    let mut dp = triangle.clone();  // Initialize DP table

    // Fill the DP table
    for i in 1..n {
        for j in 0..=i {
            if j == 0 {
                dp[i][j] += dp[i - 1][j];
            } else if j == i {
                dp[i][j] += dp[i - 1][j - 1];
            } else {
                dp[i][j] += dp[i - 1][j].min(dp[i - 1][j - 1]);
            }
        }
    }

    // Find the minimum sum in the last row
    let mut min_sum = dp[n - 1][0];
    let mut min_index = 0;
    for j in 1..n {
        if dp[n - 1][j] < min_sum {
            min_sum = dp[n - 1][j];
            min_index = j;
        }
    }

    // Backtrack to find the path
    let mut path = Vec::new();
    let mut j = min_index;
    for i in (0..n).rev() {
        path.push(triangle[i][j]);
        if i > 0 {
            if j > 0 && dp[i - 1][j - 1] <= dp[i - 1][j] {
                j -= 1;
            }
        }
    }
    path.reverse();
    path
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_minimum_sum_descent() {
        let result = calculate_minimum_sum_descent(vec![2, 5, 4, 1, 4, 7, 8, 6, 9, 6]);
        assert_eq!(vec![2, 5, 1, 6], result);

        let nums = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        assert_eq!(calculate_minimum_sum_descent(nums), vec![1, 2, 4, 7]);

        let nums = vec![1];
        assert_eq!(calculate_minimum_sum_descent(nums), vec![1]);

        let nums = vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1];
        assert_eq!(calculate_minimum_sum_descent(nums), vec![1, 1, 1, 1]);

        let nums = vec![7, 3, 8, 8, 1, 0, 2, 7, 4, 4, 4, 5, 2, 6, 5];
        assert_eq!(calculate_minimum_sum_descent(nums), vec![7, 3, 1, 4, 2]);

    }
}
