use std::collections::{HashMap, HashSet};

pub struct TextParser {
    preserve_paragragh: bool,
    custom_separators: HashSet<char>,

    dictionary: HashSet<String>,
    word_segmentation_enabled: bool,
    memo: HashMap<String, Option<Vec<String>>>,
}

impl TextParser {
    pub fn new() -> Self {
        Self { preserve_paragragh: true, custom_separators: HashSet::new(), dictionary: HashSet::new(), word_segmentation_enabled: false, memo: HashMap::new(), }
    }
        
    /// Segment concatenated words using dynamic programming
    /// Time Complexity: O(n²) where n is the length of the string
    /// Space Complexity: O(n²) for memoization
    pub fn segment_words(&mut self, text: &str) -> String {
        let lines: Vec<&str> = text.lines().collect();
        let mut result: Vec<String> = Vec::new();

        for line in lines {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                result.push(String::new());
            } else {
                // Process each "word" (space-separated token) for segmentation
                let tokens: Vec<&str> = trimmed.split_whitespace().collect();

                let mut segmented_tokens: Vec<String> = Vec::new();

                for token in tokens {
                    if let Some(segmented) = self.word_break_dp(token) {
                        segmented_tokens.push(segmented.join(" "));
                    } else {
                        segmented_tokens.push(token.to_string());
                    }
                }
                
                result.push(segmented_tokens.join(" "));
            }

        }

        result.join("\n")
    }
   
    /// Dynamic Programming approach for word break problem
    /// Time Complexity: O(n²) where n is string length
    /// Space Complexity: O(n²) with memoization
    fn word_break_dp(&mut self, s: &str) -> Option<Vec<String>> {
        let s_lower = s.to_lowercase();
        
        // Check memoization cache
        if let Some(cached_result) = self.memo.get(&s_lower) {
            return cached_result.clone();
        }
        
        let n = s_lower.len();
        if n == 0 {
            return Some(vec![]);
        }
        
        // dp[i] stores the word segmentation for s[0..i] if possible
        let mut dp: Vec<Option<Vec<String>>> = vec![None; n + 1];
        dp[0] = Some(vec![]);
        
        for i in 1..=n {
            for j in 0..i {
                if let Some(ref prev_words) = dp[j] {
                    let word = &s_lower[j..i];
                    if self.dictionary.contains(word) {
                        let mut new_segmentation = prev_words.clone();
                        // Preserve original case from input
                        new_segmentation.push(s[j..i].to_string());
                        dp[i] = Some(new_segmentation);
                        break; // Take first valid segmentation
                    }
                }
            }
        }
        
        let result = dp[n].clone();
        self.memo.insert(s_lower, result.clone());
        result
    }

    /// Alternative DP approach that finds the segmentation with minimum number of words
    /// Time Complexity: O(n²)
    fn word_break_min_cuts(&self, s: &str) -> Option<Vec<String>> {
        let s_lower = s.to_lowercase();
        let n = s_lower.len();
        
        if n == 0 {
            return Some(vec![]);
        }
        
        // dp[i] = minimum number of words needed to segment s[0..i]
        let mut dp = vec![n + 1; n + 1];
        let mut parent = vec![0; n + 1];
        dp[0] = 0;
        
        for i in 1..=n {
            for j in 0..i {
                let word = &s_lower[j..i];
                if self.dictionary.contains(word) && dp[j] + 1 < dp[i] {
                    dp[i] = dp[j] + 1;
                    parent[i] = j;
                }
            }
        }
        
        if dp[n] == n + 1 {
            return None; // Cannot segment
        }
        
        // Reconstruct the segmentation
        let mut result = Vec::new();
        let mut i = n;
        while i > 0 {
            let j = parent[i];
            result.push(s[j..i].to_string()); // Preserve original case
            i = j;
        }
        
        result.reverse();
        Some(result)
    }

    pub fn default_dictionary() -> HashSet<String> {
        let common_words = vec![
            // Articles, pronouns, prepositions
            "a", "an", "and", "are", "as", "at", "be", "by", "for", "from", "has", "he", "in", 
            "is", "it", "its", "of", "on", "that", "the", "to", "was", "will", "with", "you",
            // Common verbs
            "can", "could", "do", "does", "did", "get", "go", "had", "have", "his", "how", "i",
            "if", "may", "not", "or", "said", "say", "she", "should", "so", "they", "we", "were",
            "what", "when", "where", "who", "would", "your",
            // Common nouns and adjectives  
            "about", "after", "all", "also", "am", "been", "before", "being", "between", "both",
            "but", "came", "come", "each", "first", "good", "great", "here", "him", "into",
            "just", "know", "last", "like", "long", "made", "make", "many", "more", "most",
            "much", "must", "new", "no", "now", "only", "other", "our", "out", "over", "own",
            "same", "see", "some", "such", "take", "than", "them", "these", "think", "this",
            "through", "time", "two", "up", "use", "very", "want", "water", "way", "well",
            "work", "world", "year", "years",
            // Example domain words (extend as needed)
            "hello", "world", "computer", "program", "code", "text", "parse", "word", "sentence"
        ];

        common_words.into_iter()
        .map(|item| item.to_string())
        .collect()
    }
}