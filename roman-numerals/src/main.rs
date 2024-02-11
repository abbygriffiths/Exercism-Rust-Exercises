use roman_numerals::Roman;

fn main() {
    let roman_passes : Roman = 69.into();
    println!("{}", roman_passes);

    let roman_fails: Roman = 4000.into();
    println!("{}", roman_fails);
}
