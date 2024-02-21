pub fn build_proverb(list: &[&str]) -> String {
    if list.is_empty() {
        return "".to_string();
    }

    let mut verses: Vec<String> = list
        .windows(2)
        .map(|pair| format!("For want of a {} the {} was lost.", pair[0], pair[1]))
        .collect();

    verses.push(format!("And all for the want of a {}.", list[0]));
    verses.join("\n")
}
