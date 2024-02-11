pub fn rotate(input: &str, key: u8) -> String {
    let mut result = "".to_string();
    for c in input.chars() {
        if !c.is_alphabetic() {
            result += &c.to_string();
        }

        if c.is_uppercase() {
            result += &(65 + (c.to_ascii_uppercase() as u8 + key - 65) % 26).to_string();
        } else {
            result += &(97 + (c.to_ascii_lowercase() as u8 + key - 97) % 26).to_string();
        }
    }

    result
}
