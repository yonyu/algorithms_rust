/*
to find the longest palindromic substring.
 */

/// Expand Around Center (O(n²))
/// 
/// Every palindrome has a center.
/// 
/// Expand outward from each character (odd-length palindromes) and each pair of adjacent characters (even-length palindromes).
pub fn longest_palindrome(s: &str) -> String {
    if  s.is_empty() {
        return String::new();
    }
    let chars: Vec<char> = s.chars().collect();
    let len = chars.len();
    if len == 1 {
        return chars.iter().collect();
    }

    let mut start: usize = 0;
    let mut max_len = 1;

    // Helper function to expand around center
    // if there are a palindrome, return the left and right index of the longest palindrome.
    // if no palindrome at all, return the passed-in left and right
    fn expand_around_center(chars: &[char], left: usize, right: usize) -> (usize, usize) {
        let (mut l, mut r) = (left, right);
        while l > 0 && r + 1 < chars.len() && chars[l - 1] == chars[r + 1] {
            l -= 1;
            r += 1;
        }

        (l, r)
    }

    for i in 0..len {
        // Odd-length palindrome (single center: i)
        let (l1, r1) = expand_around_center(&chars, i, i);
        if r1 - l1 + 1 > max_len {
            start = l1;
            max_len = r1 - l1 + 1;
        }

        // Even-length palindrome (double center: i, i+1)
        if i + 1 < len && chars[i] == chars[i + 1] {
            let (l2, r2) = expand_around_center(&chars, i, i + 1);
            if r2 - l2 + 1 > max_len {
                start = l2;
                max_len = r2 - l2 + 1;
            }
        }
    }

    //max_len as i32
    chars[start..start + max_len].iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_palindrome() {
        let test_cases = vec!["babad", "cbbd", "rarr", "racecar", "rarrrr"];
        let palindromes = vec!["bab", "bb", "rar", "racecar", "rrrr"];
        let results = test_cases.iter().map(|s| longest_palindrome(s)).collect::<Vec<_>>();
        assert_eq!(results, palindromes);
    }
}