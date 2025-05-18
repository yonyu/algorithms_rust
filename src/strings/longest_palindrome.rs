/*
to find the longest palindromic substring.
 */

pub fn longest_palindrome(s: &str) -> i32 {
    let chars: Vec<char> = s.chars().collect();
    let len = chars.len();
    if len == 0 {
        return 0;
    }

    let mut start: usize; // = 0;
    let mut max_len = 1;

    // Helper function to expand around center
    fn expand_around_center(chars: &[char], left: usize, right: usize, len: usize) -> (usize, usize) {
        if left == right && left == 0 { return (left, right); }
        let (mut l, mut r) = (left, right);
        while l >= 0 && r < len && chars[l] == chars[r] {
            if r + 1 >= len || l == 0 { break; }
            l -= 1;
            r += 1;
        }
        (l + 1, r - 1) // return the valid boundaries
    }

    for i in 0..len {
        // Odd-length palindromes (center at i)
        let (l1, r1) = expand_around_center(&chars, i, i, len);
        if r1 - l1 + 1 > max_len {
            start = l1;
            max_len = (r1 - l1 + 1) as usize;
        }

        // Even-length palindromes (center between i and i+1)
        if i < len - 1 {
            let (l2, r2) = expand_around_center(&chars, i, i + 1, len);
            if r2 - l2 + 1 > max_len {
                start = l2;
                max_len = (r2 - l2 + 1) as usize;
            }
        }
    }

    max_len as i32
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_palindrome() {
        let s = "babad";
        let result = longest_palindrome(s);
        assert_eq!(result, 3);
    }
}