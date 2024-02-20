use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    factors
        .iter()
        .filter(|&&x| x != 0)
        .flat_map(|&factor| (factor..limit).step_by(factor as usize))
        .collect::<HashSet<u32>>()
        .iter()
        .sum()
}
