/*
 * @lc app=leetcode id=260 lang=rust
 * 260. Single Number III
 * Difficulty: Medium
 * Topics: array, bit manipulation
 * Companies:  Twitter, Facebook, Apple, Microsoft, Amazon, Bloomberg, Yahoo, Google
 *
 * Given an integer array nums, in which exactly two elements appear only once
 * and all the other elements appear exactly twice. Find the two elements that
 * appear only once. You can return the answer in any order.
 *
 * 1. You must write an algorithm that runs in linear runtime complexity and uses
 * only constant extra space.
 *
 * 2. Also, please don't use the bit operator xor, to solve this problem.
 *
 * 3. Please write code in Rust.
 */
pub fn find_single_numbers(nums: Vec<i32>) -> Vec<i32> {
    let mut xor = 0;
    for num in nums.iter() {
        xor ^= num;
    }

    let diff = xor & (-xor);
    let mut x = 0;
    for num in nums.iter() {
        if num & diff != 0 {
            x ^= num;
        }
    }
    vec![x, xor ^ x]
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_single_number_iii() {
        let nums = vec![1, 2, 1, 3, 2, 5];
        let res = find_single_numbers(nums);
        assert_eq!(res, vec![3, 5]);
    }
}