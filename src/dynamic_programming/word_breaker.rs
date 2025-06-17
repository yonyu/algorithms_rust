// Given a string, segmenting it into a space-separated sequence of one or more dictionary words.
use std::collections::HashSet;

/// Naive recursive approach
/// Time Complexity: O(2^n)
pub fn word_break_naive(s: &str, dict: &HashSet<String>) -> bool {
    if s.len() == 0 {
        return true;
    }

    //let dictionary: HashSet<String> = dict.into_iter().collect();
    for i in 1..=s.len() {
        let prefix = &s[0..i];
        let suffix = &s[i..];

        if dict.contains(prefix) && word_break_naive(suffix, dict) {
            return true;
        }
    }

    false
}

pub fn word_break_dp(s: &str, words: Vec<String>) -> bool {
    if s.is_empty() {
        return true;
    }

    let n = s.len();
    let dict: HashSet<String> = words.into_iter().collect();
    // dp[i]: Can we segment the string s[0..i]? (index from 0 to i-1)
    let mut dp = vec![false; n + 1];
    // Base case
    dp[0] = true; // empty string can be segmented.

    for i in 1..=n {
        for j in 0..i {
            if dp[j] && dict.contains(&s[j..i]) {
                dp[i] = true;
                break;
            }
        }
    }

    dp[s.len()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_break_native() {
        let s = "thisisatest";

        let words = ["this", "is", "a", "test"];
        let dict:HashSet<String> = words.into_iter().map(String::from).collect();
        let actual = word_break_naive(s, &dict);
        assert!(actual);
    }

        #[test]
    fn test_word_break_dp() {
        let s = "thisisatest";

        //let words = ["this", "is", "a", "test"];
        //let dict:HashSet<String> = words.into_iter().map(String::from).collect();
        let words = vec![String::from("this"), "is".to_string(), "a".to_string(), "test".to_string()];
        let actual = word_break_dp(s, words);
        assert!(actual);
    }
}