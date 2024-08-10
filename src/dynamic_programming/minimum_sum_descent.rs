/*
The book's isbn13 is 978-0-13-231681-1, isbn10 is 0132316811
P. 291
8. Minimum-sum descent: Some positive integers are arranged in an equilateral
triangle with n numbers in its base like the one shown in the figure below for
n = 4. The problem is to find the smallest sum in a descent from the triangle
apex to its base through a sequence of adjacent numbers (shown in the figure
by the circles). Design a dynamic programming algorithm for this problem
and indicate its time efficiency.

Example:

                    2

                  5   4

               1   4    7

            8    6    9    6

The minimum sum in a descent from the apex to the base is 2 + 5 + 1 + 6 = 14.

Find the path with the minimum sum in a descent from the apex to the base.
For the example, the return value is [2, 5, 1, 6].

1. The first thing is to define a data structure to represent
the numbers in equilateral triangle. We can use a 2D array to represent the
triangle. The triangle is a jagged array where each row has one more element
than the previous row. The triangle can be represented as a vector of vectors
where each inner vector represents a row of the triangle.
Because it is an equilateral triangle, the number of elements in each row is
equal to the row number starting from 1.

Given a flat vector of integers, we can convert it into a triangle representation
by iterating over the vector and creating a new row whenever the number of
elements in the row is equal to the row number.

For example, the triangle in the example can be represented as:

let triangle = vec![vec![2], vec![5, 4], vec![1, 4, 7], vec![8, 6, 9, 6]];

The code for converting the vector into a triangle representation is as follows:

let mut triangle = Vec::new();
let mut row = 1;
let mut index = 0;
let mut n = 0; // number of rows in the triangle
for i in 1..=nums.len() {
    if i == row * (row + 1) / 2 {
        triangle.push(nums[index..i].to_vec());
        n = row;
        index = i;
        row += 1;
    }
 }

2. The dynamic programming algorithm for this problem is as follows:
let dp[i][j] be the minimum sum in a descent from the apex to the base through
the sequence of adjacent numbers starting at the apex and ending at the number
at row i and column j.

3. Given the index[i][j], its two possible previous numbers are index[i-1][j-1]
and index[i-1][j]. The minimum sum for index[i][j] is the sum of the number at
index[i][j] and the minimum of the two possible previous numbers.

The minimum sum for the apex is the number at the apex itself.
The base case is dp[0][0] = triangle[0][0].

The boundary cases:
The first column of row i is dp[i][0] = dp[i-1][0] + triangle[i][0] for 0 < i < n, j=0.
The last column of row i is dp[i][i] = dp[i-1][i-1] + triangle[i][i] for 0 < i < n, j=i.

The recurrence relation is:
dp[i][j] = triangle[i][j] + min(dp[i-1][j-1], dp[i-1][j]) for 0 < j < i < n

4. To get the path with the minimum sum, we need to backtrack from the minimum
value in the last row to the apex. We can keep track of the previous number
that led to the minimum sum for each number in the triangle. This can be done
by storing the index of the previous number in a separate 2D array.

5. The time complexity of the dynamic programming algorithm is O(n^2) where n
is the number of rows in the triangle. This is because we need to compute the
minimum sum for each number in the triangle by considering the two possible
previous numbers in the row above. The algorithm iterates over each row and
each number in the row to compute the minimum sum.

6. The function signature for the algorithm is as follows:
fn calculate_minimum_sum_descent(nums: Vec<i32>) -> Vec<i32>
*/
pub fn calculate_minimum_sum_descent(nums: Vec<i32>) -> Vec<i32> {
    // convert the input vector into the triangle representation
    let mut triangle: Vec<Vec<i32>> = Vec::new(); // equilateral triangle
    let mut row = 1;
    let mut index = 0;
    let mut rows: usize = 0; // total rows in triangle

    for i in 1..= nums.len() {
        if i == row * (row + 1) / 2 {
            triangle.push(nums[index..i].to_vec());
            rows = row;

            index = i;
            row += 1;
        }
    }

    let mut paths: Vec<Vec<Vec<i32>>> = Vec::new();
    for row in 0..rows {
        paths.push(Vec::new());
        for col in 0..triangle[row].len() {
            paths[row].push(vec![0; row+1]);
        }
    }

    let mut dp: Vec<Vec<i32>> = Vec::new();
    for row in 0..rows {
        dp.push(vec![i32::MAX; triangle[row].len()])
    }

    dp[0][0]= triangle[0 as usize][0 as usize];
    paths[0][0] = vec![triangle[0 as usize][0 as usize]];

    // boundary conditions
    // the first node in a row: dp[i][0] = dp[i-1][0] + triangle[i][0]; i=1, 2, rows-1
    // the last node in a row: dp[i][(dp[i].len()-1)] = dp[i-1][(dp[i-1].len()-1)] + triangle[i][(dp[i].len()-1)]

    for i in 1..rows {
        dp[i][0] = dp[i - 1][0] + triangle[i][0];
        paths[i][0] = paths[i - 1][0].clone();
        paths[i][0].push(triangle[i][0]);

        let last: usize = dp[i].len() - 1;
        let prev_last = dp[i - 1].len() - 1;

        dp[i][last] = dp[i - 1][prev_last] + triangle[i][last];
        paths[i][last] = paths[i - 1][prev_last].clone();
        paths[i][last].push(triangle[i][last]);
    }

    for i in 1..rows {
        for j in 1..triangle[i].len() - 1 {
            let temp: i32;
            let temp_sequence: Vec<i32>;
            if dp[i - 1][j - 1] < dp[i - 1][j] {
                temp = dp[i - 1][j - 1];
                temp_sequence = paths[i - 1][j - 1].clone();
            } else {
                temp = dp[i - 1][j];
                temp_sequence = paths[i - 1][j].clone();
            }
            dp[i][j] = dp[i][j].min(temp + triangle[i][j]);
            if dp[i][j] > temp {
                paths[i][j] = temp_sequence;
                paths[i][j].push(triangle[i][j]);
            }
        }
    }

    let mut min_index = 0;
    let mut min_value = i32::MAX;

    for i in 0..rows {
        if dp[rows-1][i as usize] < min_value {
            min_index = i;
            min_value = dp[rows-1][i as usize];
        }
    }

    paths[rows-1][min_index].clone()
}

// Use 3D array to store the paths from the apex to any [i][j] in the triangle
pub fn calculate_minimum_sum_descent_2(nums: Vec<i32>) -> Vec<i32> {
    // Convert the vector of integers into a triangle representation
    let mut triangle = Vec::new();
    let mut row = 1;
    let mut index = 0;
    let mut n = 0;
    for i in 1..=nums.len() {
        if i == row * (row + 1) / 2 {
            triangle.push(nums[index..i].to_vec());
            n = row;
            index = i;
            row += 1;
        }
    }

    let mut paths:Vec<Vec<Vec<i32>>> = Vec::new();
    for row in 0..n {
        paths.push(Vec::new());
        for col in 0..triangle[row].len() {
            paths[row].push(vec![0; row+1]);
        }
    }

    let mut dp: Vec<Vec<i32>> = Vec::new();
    for row in 0..n {
        dp.push(vec![0; row+1]);
    }

    // Initialize the base case
    dp[0][0] = triangle[0][0];
    paths[0][0] = vec![triangle[0][0]];

    // Compute the boundary cases of the first column and the last column for each row
    for i in 1..n {
        dp[i][0] = dp[i-1][0] + triangle[i][0];
        paths[i][0] = paths[i-1][0].clone();
        paths[i][0].push(triangle[i][0]);

        dp[i][i] = dp[i-1][i-1] + triangle[i][i];
        paths[i][i] = paths[i-1][i-1].clone();
        paths[i][i].push(triangle[i][i]);
    }

    // Compute dp[i][j] and paths[i][j] for 0 < j < i < n
    for row in 1..n {
        for col in 1..row {
            if dp[row-1][col-1] < dp[row-1][col] {
                dp[row][col] = dp[row-1][col-1] + triangle[row][col];
                paths[row][col] = paths[row-1][col-1].clone();
                paths[row][col].push(triangle[row][col]);
            } else {
                dp[row][col] = dp[row-1][col] + triangle[row][col];
                paths[row][col] = paths[row-1][col].clone();
                paths[row][col].push(triangle[row][col]);
            }
        }
    }

    // Find the minimum sum in the last row
    let mut min_sum = dp[n-1][0];
    let mut min_path = paths[n-1][0].clone();
    for col in 1..n {
        if dp[n-1][col] < min_sum {
            min_sum = dp[n-1][col];
            min_path = paths[n-1][col].clone();
        }
    }

    min_path
}

// backtrack to find the path with the minimum sum
pub fn calculate_minimum_sum_descent_3(nums: Vec<i32>) -> Vec<i32> {
    // Step 1: Convert the input vector to a triangle representation.
    let mut triangle = Vec::new();
    let mut row = 1;
    let mut index = 0;
    let mut n = 0; // number of rows in the triangle

    // Build the triangle from the flat vector.
    for i in 1..=nums.len() {
        if i == row * (row + 1) / 2 {
            triangle.push(nums[index..i].to_vec());
            n = row;
            index = i;
            row += 1;
        }
    }

    // Step 2: Initialize dp array and start dynamic programming.
    let mut dp = triangle.clone();
    //let mut path = vec![(vec![0; row] for row in 1..=n)];
    let mut path: Vec<Vec<usize>> = (1..=n).map(|row| vec![0; row]).collect();

    for i in 1..n {
        for j in 0..=i {
            if j == 0 { // First element in the row
                dp[i][j] += dp[i - 1][j];
                path[i][j] = j;
            } else if j == i { // Last element in the row
                dp[i][j] += dp[i - 1][j - 1];
                path[i][j] = j - 1;
            } else { // Middle elements in the row
                if dp[i - 1][j - 1] < dp[i - 1][j] {
                    dp[i][j] += dp[i - 1][j - 1];
                    path[i][j] = j - 1;
                } else {
                    dp[i][j] += dp[i - 1][j];
                    path[i][j] = j;
                }
            }
        }
    }

    // Step 3: Backtrack to find the path with the minimum sum.
    let mut min_index = 0;
    let mut min_sum = dp[n - 1][0];
    for j in 1..n {
        if dp[n - 1][j] < min_sum {
            min_sum = dp[n - 1][j];
            min_index = j;
        }
    }

    // Trace the path from the base to the apex.
    let mut result = vec![triangle[n - 1][min_index]];
    for i in (0..n - 1).rev() {
        min_index = path[i + 1][min_index];
        result.push(triangle[i][min_index]);
    }

    // Reverse the result to get the path from apex to base.
    result.reverse();
    result
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
