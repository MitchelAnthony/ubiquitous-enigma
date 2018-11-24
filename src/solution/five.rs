use primal_sieve;
use std::collections::HashMap;

pub fn run(max: usize) -> usize {
    let mut product = 1;
    let sieve = primal_sieve::Sieve::new(max);
    let mut factor_map = HashMap::new();

    // Find and save prime factors
    for number in (1..=max).rev() {
        let factors = sieve.factor(number).unwrap();
        for i in &factors {
            let count = factor_map.entry(i.0).or_insert(0);
            if *count < i.1 {
                *count = i.1;
            }
        }
    }
    // println!("{:?}", factor_map);

    // Multiply prime factors
    for (factor, exponent) in &factor_map {
        product *= factor.pow(*exponent as u32);
    }

    product
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_five() {
        assert_eq!(run(10), 2520);
        assert_eq!(run(20), 232_792_560);
        assert_eq!(run(30), 2_329_089_562_800);
    }
}
