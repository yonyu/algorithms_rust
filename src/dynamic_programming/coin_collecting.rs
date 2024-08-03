use std::cmp::max;

// 0132316811, P288
pub fn coin_collecting(c: Vec<Vec<i32>>, n: i32, m: i32) -> i32 {
    let mut dp = vec![vec![0; m as usize]; n as usize];
    dp[0][0] = c[0][0];
    for i in 1..n as usize {
        dp[i][0] = dp[i - 1][0] + c[i][0];
    }

    for j in 1..m as usize {
        dp[0][j] = dp[0][j-1] + c[0][j];
    }

    for i in 1..n as usize {
        for j in 1..m as usize {
            dp[i][j] = max(dp[i-1][j], dp[i][j-1]) + c[i][j];
        }
    }

    dp[n as usize - 1][m as usize - 1]
}

pub fn coin_collecting_with_inaccessible_cells(c: Vec<Vec<i32>>, n: usize, m: usize) -> i32 {
    let mut dp = vec![vec![0; m]; n];
    dp[0][0] = c[0][0];

    // c[i][j] = -1 means cell at (i, j) is inaccessible.
    // we assign -1 to dp[i][j] if c[i][j] = -1.
    for i in 1..n {
        if c[i][0] == -1 {
            dp[i][0] = -1;
        } else {
            dp[i][0] = dp[i - 1][0] + c[i][0];
        }
    }

    for j in 1..m {
        if c[0][j] == -1 {
            dp[0][j] = -1;
        } else {
            dp[0][j] = dp[0][j - 1] + c[0][j];
        }
    }

    for i in 1..n {
        for j in 1..m {
            if c[i][j] == -1 {
                dp[i][j] = -1;
            } else {
                if dp[i - 1][j] == -1 && dp[i][j - 1] == -1 {
                    dp[i][j] = -1;
                } else if dp[i - 1][j] == -1 {
                    dp[i][j] = dp[i][j - 1] + c[i][j];
                } else if dp[i][j - 1] == -1 {
                    dp[i][j] = dp[i - 1][j] + c[i][j];
                } else {
                    dp[i][j] = max(dp[i - 1][j], dp[i][j - 1]) + c[i][j];
                }
            }
        }
    }

    dp[n-1][m-1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coin_collecting() {
        let c = vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]];
        assert_eq!(coin_collecting(c, 3, 3), 1);

        let c = vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 1]];
        assert_eq!(coin_collecting(c, 3, 3), 2);

        let c = vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 1], vec![1, 0, 0]];
        assert_eq!(coin_collecting(c, 4, 3), 2);

        let c = vec![vec![0, 0, 0, 0, 1, 0], vec![0, 1, 0, 1, 0, 0], vec![0, 0, 0, 1, 0, 1], vec![0, 0, 1, 0, 0, 1], vec![1, 0, 0, 0, 1, 0]];
        assert_eq!(coin_collecting(c, 5, 6), 5);
    }

    #[test]
    fn test_coin_collecting_with_inaccessible_cells() {
        let c = vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]];
        assert_eq!(coin_collecting_with_inaccessible_cells(c, 3, 3), 1);

        let c = vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 1]];
        assert_eq!(coin_collecting_with_inaccessible_cells(c, 3, 3), 2);

        let c = vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 1], vec![1, 0, 0]];
        assert_eq!(coin_collecting_with_inaccessible_cells(c, 4, 3), 2);

        let c = vec![vec![0, 0, 0, 0, 1, 0], vec![0, 1, 0, 1, 0, 0], vec![0, 0, 0, 1, 0, 1], vec![0, 0, 1, 0, 0, 1], vec![1, 0, 0, 0, 1, 0]];
        assert_eq!(coin_collecting_with_inaccessible_cells(c, 5, 6), 5);

        let c = vec![vec![0, -1, 0, 1, 0, 0], vec![1, 0, 0, -1, 1, 0], vec![0, 1, 0, -1, 1, 0], vec![0, 0, 0, 1, 0, 1], vec![-1, -1, -1, 0, 1, 0]];
        assert_eq!(coin_collecting_with_inaccessible_cells(c, 5, 6), 4);
    }
}