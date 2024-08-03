use std::cmp::max;

/*
2. Code Question 2

Amazon has a cluster of n servers. Some are in the OFF state while others
are ON. The developers responsible for maintaining the servers have access
to a special operation to change the states. In one operation, they can
choose a contiguous sequence of servers and flip their states, i.e., ON to
OFF and vice versa. Due to operational constraints, this operation can be
performed on the cluster a maximum of k times.

Given a binary string server_states, of length n, where ‘1’ represents
ON, ‘0’ represents OFF, and an integer k, find the maximum possible
number of consecutive ON servers after at most k operations.

Example

Suppose server_states = "1001", and k = 2)    Amazon SDE2 online assessment 2024

    2. Code Question 2

    Amazon has a cluster of n servers. Some are in the OFF state while others
    are ON. The developers responsible for maintaining the servers have access
    to a special operation to change the states. In one operation, they can
    choose a contiguous sequence of servers and flip their states, i.e., ON to
    OFF and vice versa. Due to operational constraints, this operation can be
    performed on the cluster a maximum of k times.

    Given a binary string server_states, of length n, where ‘1’ represents
    ON, ‘0’ represents OFF, and an integer k, find the maximum possible
    number of consecutive ON servers after at most k operations.

    Example

    Suppose server_states = "1001", and k = 2.

    The indices[1…2] can be chosen and flipped in one operation as follows:

    0  1  2  3                    0  1  2  3
    *--o--o--*        --->        *--*--*--*
    1  0  0  1                    1  1  1  1
       |__|
       flip

    In one operation, flip two consecutive 0's into 1's.

    Thus in one operation, the maximum number of consecutive ON servers is 4.
    It is optimal to stop after 1 operation:
    0010         --->       server_states = "1001"
    1            --->       k = 1

    Sample Output
    4

    Function Description

    Complete the function getMaxConsecutiveON in the editor below.

    getMaxConsecutiveON has the following parameters:

    string server_states: states of the servers, ‘1’ represents ON state, ‘0’ represents OFF
    int k: the maximum number of operations that can be performed

    Returns

    int: the maximum number of consecutive ON servers after a maximum of k operations are performed

    Constraints:

    1 ≤ n ≤ 2 * 10^5
    1 ≤ k ≤ 2 * 10^5
    server_states contains only 0s and 1s


    Input Format For Custom Testing:

    The first line contains a binary string server_states.
    The second line contains the integer k, the maximum number of operations allowed.

    Sample Case 0:

    Sample Input For Custom Testing

    STDIN                   FUNCTION
    -----------             ---------
    00010          --->    server_states = "00010"
    1              --->    k = 1

    Sample Output:
    4

    Explanation:
    It is optimal to apply the special operations on consecutive indices (0-based indexing) as follows:

    Flip indices [0…2] giving states = “11110”
    After 1 operation, there are a maximum of 4 consecutive ON servers.


    Sample Case 1:

    Sample Input For Custom Testing:

    STDIN                   FUNCTION
    -----------             ---------
    111010101100110  --->   server_states = "111010101100110"
    2                --->   k = 2

    Sample Output:
    8

    Explanation It is optimal to apply the special operations on consecutive indices (0-based indexing) as follows:
    Flip indices [0…1] giving states = “000010101100110”
    After 2 operations, there are a maximum of 8 consecutive ON servers.

Please write the function in Rust with the following signature:

pub fn get_max_consecutive_ones(server_states: &str, k: i32) -> i32

*/
fn get_max_consecutive_on(server_states: &str, k: usize) -> i32 {
    if k == 0 || server_states.is_empty() {
        return -1;
    }

    let chars: Vec<char> = server_states.chars().collect();
    let n = server_states.len();
    let mut store: Vec<(usize, usize)> = Vec::new();

    // Step 1: Collect all sequences of consecutive '0's
    let mut i = 0;
    while i < n {
        if server_states.chars().nth(i) == Some('0') {
            let left = i;
            while i < n && server_states.chars().nth(i) == Some('0') {
                i += 1;
            }
            let right = i - 1;
            store.push((left, right));
            continue;
        }
        i += 1;
    }

    // If the number of zero sequences is less than k, we can flip all zeros
    if store.len() < k {
        return n as i32;
    }

    let mut si = 0;
    let mut ei = 0;
    let mut max_len = 0;

    // Step 2: Use a sliding window to find the maximum sequence of '1's after flipping at most k zero sequences
    while ei < store.len() {
        while ei - si + 1 == k {
            let mut left = store[si].0 as i32 - 1;
            let mut right = store[ei].1 as i32 + 1;

            // Expand left boundary while it's '1'
            while left >= 0 && chars[left as usize] == '1' {
                left -= 1;
            }

            // Expand right boundary while it's '1'
            while right < n as i32 && chars[right as usize] == '1' {
                right += 1;
            }

            max_len = max(max_len, right - left - 1);
            si += 1;
        }

        ei += 1;
    }

    max_len
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_max_consecutive_on_1() {
        let server_states = "00010";
        //let server_states = "00010";
        let k = 1;
        let result = get_max_consecutive_on(server_states, k);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_get_max_consecutive_on_2() {
        let server_states = "111010101100110";
        let k = 2;
        let result = get_max_consecutive_on(server_states, k);
        assert_eq!(result, 8);
    }
}
