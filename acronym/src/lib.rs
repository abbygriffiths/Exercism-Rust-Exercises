use convert_case::{Case, Casing};
use unicode_segmentation::UnicodeSegmentation;

pub fn abbreviate(phrase: &str) -> String {
    phrase
        .to_case(Case::Title)
        .unicode_words()
        .map(|word| {
            (*word)
                .chars()
                .filter(|c| c.is_alphabetic())
                .take(1)
                .map(|x| x.to_uppercase().to_string())
                .collect::<String>()
        })
        .collect()
}
