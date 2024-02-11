use std::fmt::{Display, Formatter, Result};

pub struct Roman(u32);

impl Display for Roman {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let num = self.0;
        let roman_numerals = vec![
            (1000, "M"),
            (900, "CM"),
            (500, "D"),
            (400, "CD"),
            (100, "C"),
            (90, "XC"),
            (50, "L"),
            (40, "XL"),
            (10, "X"),
            (9, "IX"),
            (5, "V"),
            (4, "IV"),
            (1, "I"),
        ];

        roman_numerals
            .iter()
            .fold(
                (num, String::new()),
                |(acc_num, acc_string), &(value, symbol)| {
                    let (quotient, remainder) = (acc_num / value, acc_num % value);
                    (remainder, acc_string + &(symbol.repeat(quotient as usize)))
                },
            )
            .1
            .fmt(f)
    }
}

impl From<u32> for Roman {
    fn from(num: u32) -> Self {
        assert!(num <= 3999, "cannot create number larger than 3999");
        Self(num)
    }
}
