pub fn raindrops(n: u32) -> String {
    let mut result_string = "".to_string();

    let sounds = vec![(3, "Pling"), (5, "Plang"), (7, "Plong")];

    for (key, value) in &sounds {
        if n % key == 0 {
            result_string += value;
        }
    }

    if result_string.is_empty() {
        return n.to_string();
    }

    result_string
}
