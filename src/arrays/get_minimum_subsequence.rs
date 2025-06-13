/// This is a FANNG interview question.
/// To find the minimum subsequence that maintains the same sum of absolute differences between 
/// adjacent elements.
/// 
/// Example 1:
/// Given an array 5 4 0 3 3 1, take sum of absolute differences between adjacent 
/// pairs i.e |5-4|+|4-0|+|0-3|+|3-3|+|3-1| = 10
/// The task is to remove as many elements from the array such that the sum remains same.
/// 
/// soln 1=> 5 4 0 3 1 : in this case sum of absolute differences between adjacent 
/// pairs is same as 10.
/// soln 2 => 5 0 3 1 : in this case as well sum of absolute differences between 
/// adjacent pairs is same as 10.
///
/// soln 2 is acceptable answer as it has minimum elements
///
/// Example 2:
/// 6 4 4 3 3 2
/// answer : 6 4 3 2
pub fn get_min_subsequence(arr: &[i32]) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    let n = arr.len();
    if n <= 1 {
        return result;
    }
    
    let b:[u64; 5];

    // Always keep the first element
    result.push(arr[0]);

    // Process middle elements with greedy approach
    for i in 1..n-1 {
        let prev = arr[i-1];
        let curr = arr[i];
        let next = arr[i+1];

        if curr == prev {
            continue;
        }

        // Keep local extrema (turning points) (in case [1, 2, 2], the first 2 is a turning point)
        let is_local_max = curr > prev && curr >= next;
        let is_local_min = curr < prev && curr <= next;

        if is_local_max || is_local_min {
            result.push(curr);
        }
    }

    // Keep the last element if it is not equal to its previous element
    if arr[arr.len()-1] != arr[arr.len()-2] {
        result.push(arr[arr.len()-1])
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_min_subsequence_works_1() {
        assert_eq!(get_min_subsequence(&[5, 4, 0, 3, 3, 1]), vec![5, 0, 3, 1]);
    }
    #[test]
    fn get_min_subsequence_works_2() {
        assert_eq!(get_min_subsequence(&[6, 4, 4, 3, 3, 2]), vec![6, 4, 3, 2]);
    }
}