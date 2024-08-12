/*
200. Number of Islands
https://leetcode.com/problems/number-of-islands/description/

Given an m x n 2D binary grid which represents a map of '1's (land) and '0's (water), return
the number of islands.

An island is surrounded by water and is formed by connecting adjacent lands horizontally or
vertically. You may assume all four edges of the grid are all surrounded by water.

The function signature is:
fn num_islands(grid: Vec<Vec<char>>) -> i32;

Example 1:

Input: grid = [
  ["1","1","1","1","0"],
  ["1","1","0","1","0"],
  ["1","1","0","0","0"],
  ["0","0","0","0","0"]
]
Output: 1

Example 2:

Input: grid = [
  ["1","1","0","0","0"],
  ["1","1","0","0","0"],
  ["0","0","1","0","0"],
  ["0","0","0","1","1"]
]
Output: 3

Constraints:

    m == grid.length
    n == grid[i].length
    1 <= m, n <= 300
    grid[i][j] is '0' or '1'.
 */

// recursively call dfs explicitly for all four directions. the benefit of doing so is that we
// can avoid creating a new variable every time dfs is called.
//
// the best speed:   // 5 ms,  Beats 95.53%
// and memory usage: // 8.94 MB, Beats 87.15%
#[allow(dead_code)]
pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
    let mut grid = grid;
    let mut count = 0;

    // Depth-first search to mark connected land
    fn dfs(grid: &mut Vec<Vec<char>>, i: isize, j: isize) {
        // Check for out-of-bounds and water cells
        if i < 0 || j < 0 || i >= grid.len() as isize || j >= grid[0].len() as isize || grid[i as usize][j as usize] == '0' {
            return;
        }

        // Mark the current cell as visited by setting it to '0'
        grid[i as usize][j as usize] = '0';

        // Move in all four possible directions: up, down, left, right
        dfs(grid, i + 1, j); // move down
        dfs(grid, i - 1, j); // move up
        dfs(grid, i, j + 1); // move right
        dfs(grid, i, j - 1); // move left
    }

    // Iterate through each cell in the grid
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            // If a land cell ('1') is found, start a DFS to mark the entire island
            if grid[i][j] == '1' {
                count += 1; // Increment the island count
                dfs(&mut grid, i as isize, j as isize); // Mark the entire island
            }
        }
    }

    // Return the total number of islands found
    count
}

// By using an explicit stack, this approach avoids the risk of stack overflow, which is common with
// deep recursion on large grids.
#[allow(dead_code)]
pub fn num_islands_1(mut grid: Vec<Vec<char>>) -> i32 {
    let mut count = 0;
    let rows = grid.len();
    let cols = grid[0].len();

    fn dfs(grid: &mut Vec<Vec<char>>, start_i: usize, start_j: usize) {
        let mut stack = vec![(start_i, start_j)];

        while let Some((i, j)) = stack.pop() {
            // Mark the cell as visited by setting it to '0'
            grid[i][j] = '0';

            // Check all 4 possible directions (down, up, right, left)
            if i + 1 < grid.len() && grid[i + 1][j] == '1' {
                stack.push((i + 1, j));
            }
            if i > 0 && grid[i - 1][j] == '1' {
                stack.push((i - 1, j));
            }
            if j + 1 < grid[0].len() && grid[i][j + 1] == '1' {
                stack.push((i, j + 1));
            }
            if j > 0 && grid[i][j - 1] == '1' {
                stack.push((i, j - 1));
            }
        }
    }

    for i in 0..rows {
        for j in 0..cols {
            if grid[i][j] == '1' {
                count += 1;
                dfs(&mut grid, i, j);
            }
        }
    }

    count
}

// use a local variable dfs inside dfs function to avoid passing grid as a parameter.
#[allow(dead_code)]
pub fn num_islands_2(grid: Vec<Vec<char>>) -> i32 {
    let mut grid = grid;
    let mut count = 0;

    fn dfs(grid: &mut Vec<Vec<char>>, i: i32, j: i32) {

        let adj : [[i32; 2]; 4] = [
            [0, 1],
            [0, -1],
            [1, 0],
            [-1, 0],
        ];
        if i < 0 || j < 0 || i >= grid.len() as i32 || j >= grid[0].len() as i32 || grid[i as usize][j as usize] == '0' {
            return;
        }
        grid[i as usize][j as usize] = '0';
        for [dx, dy] in adj.into_iter() {
            dfs(grid, i + dx, j + dy);
        }
    }

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == '1' {
                count += 1;
                dfs(&mut grid, i as i32, j as i32);
            }
        }
    }
    count
}

const ADJ : [(i32, i32); 4] = [
    (0, 1),
    (0, -1),
    (1, 0),
    (-1, 0),
];

// use a constant array to represent the 4 directions of a cell.
#[allow(dead_code)]
fn num_islands_3(grid: Vec<Vec<char>>) -> i32 {
    let mut grid = grid;
    let mut count = 0;

    fn dfs(grid: &mut Vec<Vec<char>>, i: i32, j: i32) {
        if i < 0 || j < 0 || i >= grid.len() as i32 || j >= grid[0].len() as i32 || grid[i as usize][j as usize] == '0' {
            return;
        }
        grid[i as usize][j as usize] = '0';
        for (dx, dy) in ADJ.iter() {
            dfs(grid, i + dx, j + dy);
        }
    }

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == '1' {
                count += 1;
                dfs(&mut grid, i as i32, j as i32);
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_islands() {
        let grid = vec![
            vec!['1','1','1','1','0'],
            vec!['1','1','0','1','0'],
            vec!['1','1','0','0','0'],
            vec!['0','0','0','0','0'],
        ];
        assert_eq!(num_islands(grid), 1);

        let grid = vec![
            vec!['1','1','0','0','0'],
            vec!['1','1','0','0','0'],
            vec!['0','0','1','0','0'],
            vec!['0','0','0','1','1'],
        ];
        assert_eq!(num_islands(grid), 3);

        // to skip moving left will have 2 islands, but it is wrong
        let grid = vec![
            vec!['1','1','1','1','0'],
            vec!['1','1','0','1','0'],
            vec!['1','1','0','0','0'],
            vec!['0','1','0','0','0'],
            vec!['1','1','0','0','0'],
        ];
        assert_eq!(num_islands(grid), 1);

        // to skip moving up will have 3 islands, but it is wrong.
        let grid = vec![
            vec!['1','0','1','1','0'],
            vec!['1','1','1','0','0'],
            vec!['1','1','0','0','0'],
            vec!['0','1','0','0','0'],
            vec!['1','0','0','0','0'],
        ];
        assert_eq!(num_islands(grid), 2);

        let grid = vec![
            vec!['1', '0', '0', '0', '0', '0', '0', '0', '1', '0'],
            vec!['0', '0', '0', '0', '0', '0', '0', '1', '0', '0'],
            vec!['0', '0', '1', '0', '0', '0', '0', '0', '0', '0'],
            vec!['0', '0', '0', '0', '1', '0', '0', '0', '0', '0'],
            vec!['0', '0', '0', '0', '0', '0', '0', '0', '1', '0'],
            vec!['0', '0', '0', '0', '0', '0', '0', '0', '0', '0'],
            vec!['1', '0', '0', '0', '0', '0', '0', '0', '0', '1'],
        ];
        assert_eq!(num_islands(grid), 8);

        let grid = vec![
            vec!['1', '1', '1', '1', '1', '1', '1', '1', '1', '1'],
            vec!['1', '1', '1', '1', '1', '1', '1', '1', '1', '1'],
            vec!['1', '1', '1', '1', '1', '1', '1', '1', '1', '1'],
            vec!['1', '1', '1', '1', '1', '1', '1', '1', '1', '1'],
            vec!['1', '1', '1', '1', '1', '1', '1', '1', '1', '1'],
            vec!['1', '1', '1', '1', '1', '1', '1', '1', '1', '1'],
            vec!['1', '1', '1', '1', '1', '1', '1', '1', '1', '1'],
            vec!['1', '1', '1', '1', '1', '1', '1', '1', '1', '1'],
            vec!['1', '1', '1', '1', '1', '1', '1', '1', '1', '1'],
            vec!['1', '1', '1', '1', '1', '1', '1', '1', '1', '1'],
        ];
        assert_eq!(num_islands(grid), 1);

        let grid = vec![
            vec!['1', '0', '1', '0', '0', '0', '1', '0', '1', '0'],
            vec!['0', '1', '0', '1', '0', '1', '0', '1', '0', '1'],
            vec!['1', '0', '1', '0', '1', '0', '1', '0', '1', '0'],
            vec!['0', '1', '0', '1', '0', '1', '0', '1', '0', '1'],
            vec!['1', '0', '1', '0', '1', '0', '1', '0', '1', '0'],
            vec!['0', '1', '0', '1', '0', '1', '0', '1', '0', '1'],
            vec!['1', '0', '1', '0', '1', '0', '1', '0', '1', '0'],
            vec!['0', '1', '0', '1', '0', '1', '0', '1', '0', '1'],
            vec!['1', '0', '1', '0', '1', '0', '1', '0', '1', '0'],
            vec!['0', '1', '0', '1', '0', '1', '0', '1', '0', '1'],
        ];
        assert_eq!(num_islands(grid), 49);

        // A 100x100 grid entirely filled with '1's (a single large island)
        let grid = vec![vec!['1'; 100]; 100];
        assert_eq!(num_islands(grid), 1);
    }

    #[test]
    pub fn test_num_islands_1() {
        // A 300x300 grid entirely filled with '1's (a single large island)
        let grid = vec![vec!['1'; 300]; 300];
        assert_eq!(num_islands_1(grid), 1);
    }

    #[test]
    pub fn test_num_islands_1_2() {
        let mut grid = vec![vec!['0'; 300]; 300];
        for i in 0..300 {
            for j in 0..300 {
                if (i + j) % 2 == 0 {
                    grid[i][j] = '1';
                }
            }
        }
        assert_eq!(num_islands_1(grid), 45000);
    }

    #[test]
    pub fn test_num_islands_1_3() {
        let mut grid = vec![vec!['0'; 300]; 300];
        for i in (0..300).step_by(2) {
            grid[i] = vec!['1'; 300];
        }
        assert_eq!(num_islands_1(grid), 150);
    }
}

