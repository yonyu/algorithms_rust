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

/// Dynamic Programming (O(n²))
/// 
/// Use a table to store whether a substring is a palindrome.
/// 
/// Build the solution iteratively using previously computed results.
/// 
/// Intuition
/// Dynamic programming helps us store results of subproblems to avoid redundant computations. 
/// The key idea is:
/// 
/// If a substring s[i..j] is a palindrome, then:
/// 
/// s[i] == s[j]
/// 
/// The inner substring s[i+1..j-1] must also be a palindrome.
/// 
/// Thought Process
/// Define a DP table dp[i][j]:
/// 
/// true if s[i..j] is a palindrome.
/// 
/// false otherwise.
/// 
/// Base Cases:
/// 
/// Every single character is a palindrome (dp[i][i] = true).
/// 
/// Two consecutive identical characters form a palindrome (dp[i][i+1] = true).
/// 
/// Transition:
/// 
/// If s[i] == s[j] and dp[i+1][j-1] is true, then dp[i][j] = true.
/// 
/// Iterate over all substring lengths:
/// 
/// Start with length 2, then 3, up to n.
/// 
/// Track the longest palindrome found.
pub fn longest_palindrome_dp(s: &str) -> String {
    if s.is_empty() {
        return String::new();
    }
    let chars: Vec<char> = s.chars().collect();
    let n = chars.len();
    let mut dp = vec![vec![false; n]; n];
    let mut start = 0;
    let mut max_len = 1;

    // All substrings of length 1 are palindromes
    for i in 0..n {
        dp[i][i] = true;
    }

    // Check for substrings of length 2
    for i in 0..n - 1 {
        if chars[i] == chars[i + 1] {
            dp[i][i + 1] = true;
            start = i;
            max_len = 2;
        }
    }

    // Check for substrings of length 3 or more
    for length in 3..=n {
        for i in 0..n - length + 1 {
            let j = i + length - 1;
            if chars[i] == chars[j] && dp[i + 1][j - 1] {
                dp[i][j] = true;
                if length > max_len {
                    start = i;
                    max_len = length;
                }
            }
        }
    }

    chars[start..start + max_len].iter().collect()
}

/// Manacher’s Algorithm (O(n))
/// 
/// A specialized algorithm that transforms the string to efficiently find palindromes in linear time.
/// 
/// Intuition
/// 
/// Manacher’s Algorithm optimizes palindrome detection by leveraging symmetry properties:
/// 
/// 1. Transform the string: Insert special characters (#) between letters to handle even-length 
/// palindromes uniformly.
/// 2. Use a palindrome radius array (p): Track the length of the palindrome centered at each 
/// position.
/// 3. Expand efficiently: Instead of checking every character, use previously computed results 
/// to skip redundant checks.
/// 
/// Thought Process
/// 1. Preprocess the string:
/// 
/// Convert "racecar" → "#r#a#c#e#c#a#r#" to handle even-length cases.
/// 
/// This ensures every palindrome has a single center.
/// 
/// 2. Iterate through the transformed string:
/// 
/// Maintain a center (c) and right boundary (r) of the longest palindrome found so far.
/// 
/// If a new character is inside the boundary, use mirror properties to initialize its palindrome length.
/// 
/// Expand outward only when necessary.
/// 
/// 3. Extract the longest palindrome:
/// 
/// Convert back to the original format.
pub fn longest_palindrome_manacher(s: &str) -> String {
    if s.is_empty() {
        return String::new();
    }

    // Step 1: Transform the string to handle even-length palindromes uniformly
    //         After transformation, both odd and even-length input string becomes odd-length
    let transformed: String = format!("#{}", s.chars().map(|c| format!("{}#", c)).collect::<String>());
    println!("transformed: {transformed}");

    let chars: Vec<char> = transformed.chars().collect();

    let n = chars.len();
    // p[i] = radius of palindrome centered at position i (inclusive of center)
    let mut p = vec![0; n]; // Palindrome radius array
    // Variables for tracking the longest palindrome
    let mut center = 0;     // The center of the longest palindrome found so far
    let mut right = 0;      // The rightmost boundary of the longest palindrome discovered
    let mut max_len = 0;    // The length of the longest palindromic substring found
    let mut max_center = 0; // The center index of the longest palindromic substring found

    // Step 2: Apply Manacher's Algorithm
    for i in 0..n {
        // Mirror index: The symmetric position of 'i' with respect to 'center'
        let mirror = if 2 * center >= i { 2 * center - i } else { 0 };

        // If within the right boundary, use previously computed values
        if i < right {
            p[i] = p[mirror].min(right - i);
        }

        // Attempt to expand palindrome centered at i
        // let mut a = i + (p[i] + 1);
        // let mut b = i - (p[i] + 1);
        // while a < n && b >= 0 && chars[a] == chars[b] {
        //     p[i] += 1;
        //     a += 1;
        //     b -= 1;
        // }

        // Attempt to expand palindrome around center i as far as possible
        // This is similar to expand-around-center but more efficient due to symmetry
        while i + p[i] + 1 < n && i >= p[i] + 1 && chars[i + p[i] + 1] == chars[i - p[i] - 1] {
            p[i] += 1;
        }

        // Update center and right boundary if this palindrome centered at i expands past right.
        // (adjust center based on expanded palindrome.)
        if i + p[i] > right {
            center = i;
            right = i + p[i];
        }

        // Track the longest palindrome
        if p[i] > max_len {
            max_len = p[i];
            max_center = i;
        }
    }

    // Step 3: Extract the longest palindrome from original string
    let start = (max_center - max_len) / 2;
    s[start..start + max_len].to_string()    
    
    // if s.is_empty() {
    //     return String::new();
    // }
    // let chars: Vec<char> = s.chars().collect();
    // let n = chars.len();
    // let mut start = 0;
    // let mut max_len = 0;
    // let mut center = 0;
    // let mut right = 0;
    // let mut p = vec![0; n];
    // 
    // for i in 0..n {
    //     let mirror = 2 * center - i;
    //     if i < right {
    //         p[i] = std::cmp::min(right - i, p[mirror]);
    //     }
    // 
    //     // Attempt to expand palindrome centered at i
    //     let mut a = i + (p[i] + 1);
    //     let mut b = i - (p[i] + 1);
    //     while a < n && b >= 0 && chars[a] == chars[b] {
    //         p[i] += 1;
    //         a += 1;
    //         b -= 1;
    //     }
    // 
    //     // If palindrome centered at i expands past right,
    //     // adjust center based on expanded palindrome.
    //     if i + p[i] > right {
    //         center = i;
    //         right = i + p[i];
    //     }
    // 
    //     // Update longest palindrome if necessary
    //     if p[i] > max_len {
    //         max_len = p[i];
    //         start = (i - p[i]) / 2;
    //     }
    // }
    // 
    // chars[start..start + max_len].iter().collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_palindrome() {
        let test_cases = vec!["a", "babad", "cbbd", "rarr", "racecar", "rarrrr"];
        let palindromes = vec!["a", "bab", "bb", "rar", "racecar", "rrrr"];
        let results = test_cases.iter().map(|s| longest_palindrome(s)).collect::<Vec<_>>();
        assert_eq!(results, palindromes);
    }

    #[test]
    fn test_longest_palindrome_dp() {
        let test_cases = vec!["a", "babad", "cbbd", "rarr", "racecar", "rarrrr"];
        let palindromes = vec!["a", "bab", "bb", "rar", "racecar", "rrrr"];
        let results = test_cases.iter().map(|s| longest_palindrome_dp(s)).collect::<Vec<_>>();
        assert_eq!(results, palindromes);
    }


    #[test]
    fn test_longest_palindrome_manacher() {
        let test_cases = vec!["a", "babad", "cbbd", "rarr", "racecar", "rarrrr"];
        let palindromes = vec!["a", "bab", "bb", "rar", "racecar", "rrrr"];
        
        let s = "babad";
        let _result = longest_palindrome_manacher(s);
        let results = test_cases.iter().map(|s| longest_palindrome_manacher(s)).collect::<Vec<_>>();
        assert_eq!(results, palindromes);
    }    
}