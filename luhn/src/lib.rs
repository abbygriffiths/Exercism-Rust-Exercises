/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    if !code.chars().all(|c| c.is_numeric() || c.is_whitespace()) {
        return false;
    }

    let code: String = code.chars().filter(|c| c.is_numeric()).collect();
    if code.len() <= 1 {
        return false;
    }

    let sum = code
        .chars()
        .rev()
        .filter(|c: &char| c.is_numeric())
        .enumerate()
        .filter_map(|(i, c)| {
            if i % 2 == 1 {
                let double = c.to_digit(10).unwrap() * 2;
                if double > 9 {
                    Some(double - 9)
                } else {
                    Some(double)
                }
            } else {
                c.to_digit(10)
            }
        })
        .sum::<u32>();

    sum % 10 == 0
}
