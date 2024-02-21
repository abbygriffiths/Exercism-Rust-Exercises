use std::collections::HashSet;

use counter::Counter;
use unicode_segmentation::UnicodeSegmentation;

pub fn anagrams_for<'a>(word: &'a str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let lowercase = word.to_lowercase();
    let char_set_word = lowercase.graphemes(true).collect::<Counter<_>>();

    possible_anagrams
        .iter()
        .filter(|&w| {
            let lower = w.to_lowercase();
            lower.graphemes(true).collect::<Counter<_>>() == char_set_word && lower != lowercase
        })
        .map(|&w| w)
        .collect()
}
