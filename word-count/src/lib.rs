use std::collections::HashMap;
use unicode_segmentation::UnicodeSegmentation;

/// Count occurrences of words.
pub fn word_count(words: &str) -> HashMap<String, u32> {
    let mut counts = HashMap::new();

    words.to_lowercase().unicode_words().for_each(|word| {
        counts
            .entry(word.to_string())
            .and_modify(|count| *count += 1)
            .or_insert(1);
    });

    counts
}
