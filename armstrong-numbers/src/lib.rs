pub fn is_armstrong_number(num: u64) -> bool {
    let digits: Vec<u64> = num
        .to_string()
        .chars()
        .filter_map(|c| c.to_digit(10))
        .map(|x| x as u64)
        .collect();

    let n = digits.len() as u32;
    let sum: u64 = digits.iter().map(|&x| x.pow(n)).sum();

    sum == num
}
