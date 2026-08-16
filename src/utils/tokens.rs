//! Token counting utilities for context budgeting

/// Utility functions for token estimation
pub struct TokenUtils;

impl TokenUtils {
    /// Estimate token count for text (rough approximation)
    /// 
    /// This uses a simple heuristic: ~4 characters per token for English text.
    /// For more accurate counting, use a proper tokenizer.
    pub fn estimate_token_count(text: &str) -> usize {
        if text.is_empty() {
            return 0;
        }

        // Simple heuristic: average of character-based and word-based estimates
        let char_estimate = text.len() / 4;
        let word_estimate = text.split_whitespace().count() * 4 / 3;
        
        (char_estimate + word_estimate) / 2
    }

    /// Count tokens more accurately using tiktoken-style approximation
    pub fn count_tokens_approximate(text: &str) -> usize {
        if text.is_empty() {
            return 0;
        }

        let mut count = 0;
        let mut in_word = false;
        let mut word_len = 0;

        for c in text.chars() {
            if c.is_alphanumeric() || c == '_' {
                if !in_word {
                    in_word = true;
                    word_len = 0;
                }
                word_len += 1;
                
                // Every 8 characters in a word is roughly a token
                if word_len % 8 == 0 {
                    count += 1;
                }
            } else if c.is_whitespace() {
                if in_word {
                    // End of word: count remaining characters
                    count += (word_len as f64 / 4.0).ceil() as usize;
                    in_word = false;
                }
                // Whitespace tokens
                count += 1;
            } else {
                if in_word {
                    count += (word_len as f64 / 4.0).ceil() as usize;
                    in_word = false;
                }
                // Punctuation and special chars
                count += 1;
            }
        }

        // Handle last word if any
        if in_word {
            count += (word_len as f64 / 4.0).ceil() as usize;
        }

        count.max(1)
    }

    /// Check if text exceeds token budget
    pub fn exceeds_budget(text: &str, budget: usize) -> bool {
        Self::estimate_token_count(text) > budget
    }

    /// Truncate text to fit within token budget
    pub fn truncate_to_budget(text: &str, budget: usize) -> String {
        if budget == 0 {
            return String::new();
        }

        // Binary search for the right truncation point
        let mut low = 0;
        let mut high = text.len();
        let mut result = text.to_string();

        while low < high {
            let mid = (low + high) / 2;
            let truncated = &text[..mid];
            
            if Self::estimate_token_count(truncated) <= budget {
                low = mid + 1;
                result = truncated.to_string();
            } else {
                high = mid;
            }
        }

        result
    }

    /// Calculate tokens per character ratio for a sample
    pub fn calculate_ratio(sample: &str, actual_tokens: usize) -> f64 {
        if sample.is_empty() {
            return 0.0;
        }
        actual_tokens as f64 / sample.len() as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_estimate_token_count() {
        // Rough estimates - these are approximations
        assert!(TokenUtils::estimate_token_count("") == 0);
        assert!(TokenUtils::estimate_token_count("hello") > 0);
        assert!(TokenUtils::estimate_token_count("hello world") > TokenUtils::estimate_token_count("hello"));
    }

    #[test]
    fn test_exceeds_budget() {
        assert!(!TokenUtils::exceeds_budget("hello", 100));
        assert!(TokenUtils::exceeds_budget(&"a".repeat(1000), 100));
    }

    #[test]
    fn test_truncate_to_budget() {
        let text = "This is a longer piece of text that should be truncated";
        let truncated = TokenUtils::truncate_to_budget(text, 5);
        assert!(truncated.len() <= text.len());
        assert!(TokenUtils::estimate_token_count(&truncated) <= 5);
    }
}
