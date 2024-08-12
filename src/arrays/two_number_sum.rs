// @lc app=leetcode id=8 lang=rust
// Given an array of distinct integer values, find if there is any pair that sum up
// to a given number.
#[allow(dead_code)]
pub fn two_num_sum1(nums: &Vec<i32>, target: i32) -> bool {
    let mut seen = std::collections::HashSet::new();
    for num in nums {
        if seen.contains(&(target - num)) {
            return true;
        }
        seen.insert(num);
    }
    false
}

/*
* @lc app=leetcode id=1 lang=rust
Given an array of integers and an integer target, return indices of the two
numbers such that they add up to a specific target.

You may assume that each input would have exactly one solution, and you may not
use the same element twice.

You can return the answer in any order.

*/
#[allow(dead_code)]
pub fn two_num_sum2(nums: &Vec<i32>, target: i32) -> Vec<i32> {
    let mut seen = std::collections::HashMap::new();
    for (i, num) in nums.iter().enumerate() {
        let diff = target - num;
        if seen.contains_key(&diff) {
            return vec![*seen.get(&diff).unwrap(), i as i32];
        }
        seen.insert(num, i as i32);
    }
    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_num_sum1_1() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        let result = two_num_sum1(&nums, target); // crate::arrays::two_number_sum::
        assert_eq!(result, true);
    }

    #[test]
    fn test_two_num_sum1_2() {
        let nums = vec![3, 5, -4, 8, 11, 1, -1, 6];
        let expected = true;
        let result = two_num_sum1(&nums, 10);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_two_num_sum2_1() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        let result = two_num_sum2(&nums, target);
        assert_eq!(result, vec![0, 1]);
    }
}