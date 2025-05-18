/*
Given an input string and a dictionary of words, find out if the input string
can be segmented into a space-separated sequence of dictionary words.

Example: s = "leetcode", word_dict = ["leet", "code"]
icecream -> ice cream
mango    -> man go
applepenapple -> apple pen apple
catsandog -> cats and dog
          -> cat, sand, dog

Approach:
Initial intuition:
Greedy: try to match the longest word in the dictionary first, then the second longest, and so on.
Potential problems: might not be able to find a solution, even if there is one.

Solution:
Use dynamic programming to solve this problem.

Let dp[i] be true if the first i characters in s can be segmented into words in the dictionary.
the formula is:
    dp[i] = dp[j] && sub[j+1, i]
    where j < i, if dp[j] is true, and sub[j+1, i] is in the dictionary.
    dp[0] = true, empty string can always be segmented (base case).
    j is the starting index of the substring.
 */
use std::collections::{HashSet, HashMap};

pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
    // Convert the dictionary into a HashSet for faster lookups
    let word_set: std::collections::HashSet<String> = word_dict.into_iter().collect();
    let n = s.len();

    // DP array that will store whether a substring can be segmented
    // dp[i] is true if s[0..i] (the first i characters) can be segmented into words in the word_dict
    let mut dp = vec![false; n + 1];
    // Empty string can always be segmented (base case)
    dp[0] = true; // empty string

    // Iterate over each character (from the first to the last) at i=[1...n] in the string `s`
    for i in 1..=n { // i is the ending index of the substring
        // Iterate over each index j before `i`
        for j in 0..i { // j is the starting index of the substring
            // If the substring s[j..i] is in the dictionary and dp[j] is true, set dp[i] to true
            if dp[j] && word_set.contains(&s[j..i]) {
                dp[i] = true;
                break; // No need to check further once we know dp[i] is true
            }
        }
    }

    // The value of dp[s.len()] tells us if the entire string can be segmented
    dp[n]
}

fn word_break_with_words(s: String, word_dict: Vec<String>) -> Option<Vec<String>> {
    // Convert the dictionary into a HashSet for faster lookups
    let word_set: HashSet<String> = word_dict.into_iter().collect();

    // DP array that will store whether a substring can be segmented
    let mut dp = vec![false; s.len() + 1];
    // Empty string can always be segmented (base case)
    dp[0] = true;

    // Map to store valid end indices to their corresponding start index
    let mut backtrack: HashMap<usize, Vec<usize>> = HashMap::new();

    // Iterate over each index i in the string `s`
    for i in 1..=s.len() {
        // Iterate over each index j before `i`
        for j in 0..i {
            // If the substring s[j..i] is in the dictionary and dp[j] is true,
            // set dp[i] to true and track j in backtrack for i
            if dp[j] && word_set.contains(&s[j..i]) {
                dp[i] = true;
                backtrack.entry(i).or_insert_with(Vec::new).push(j);
            }
        }
    }

    // If dp[s.len()] is false, it means the string cannot be segmented
    if !dp[s.len()] {
        return None;
    }

    // Helper function for backtracking to collect words
    fn collect_words(s: &str, backtrack: &HashMap<usize, Vec<usize>>, end: usize) -> Vec<Vec<String>> {
        if end == 0 {
            return vec![vec![]];
        }

        let mut result = Vec::new();
        if let Some(starts) = backtrack.get(&end) {
            for &start in starts {
                let words = collect_words(s, backtrack, start);
                for mut word_list in words {
                    word_list.push(s[start..end].to_string());
                    result.push(word_list);
                }
            }
        }
        result
    }

    // Collect all the possible word segmentations by backtracking from the end of the string
    let mut result = collect_words(&s, &backtrack, s.len());

    // Since we collect words from end to start, we need to reverse the individual word lists
    for word_list in &mut result {
        word_list.reverse();
    }

    // Return the first found segmentation (you could return all if needed)
    Some(result[0].clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_break() {
        let s = "leetcode".to_string();
        let word_dict = vec!["leet".to_string(), "code".to_string()];
        let result = word_break(s, word_dict);
        assert_eq!(result, true);
    }

    #[test]
    fn test_word_break_with_words() {
        let s = "applepenapple".to_string();
        let word_dict = vec!["apple".to_string(), "pen".to_string()];
        let result = word_break_with_words(s, word_dict);
        assert_eq!(result, Some(vec!["apple".to_string(), "pen".to_string(), "apple".to_string()]));
    }

    #[test]
    fn test_word_break_with_words_2() {
        let s = "catsanddog".to_string();
        let word_dict = vec!["cat".to_string(), "cats".to_string(), "dog".to_string(), "sand".to_string(), "and".to_string()];
        let result = word_break_with_words(s, word_dict);
        assert_eq!(result, Some(vec!["cat".to_string(), "sand".to_string(), "dog".to_string()]));
    }
}