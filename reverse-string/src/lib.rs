use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    if input.is_empty() {
        return "".to_string();
    }
    input.graphemes(true).rev().collect()
}
