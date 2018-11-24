use primal_sieve;

pub fn run(number: usize) -> usize {
    // Approx nth prime
    // n log n + n log log n
    let approx = 
        (number as f32 * (number as f32).ln() +
        number as f32 * (number as f32).ln().ln()).ceil() as usize;

    let sieve = primal_sieve::Sieve::new(approx);
    
    sieve.nth_prime(number)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seven() {
        assert_eq!(run(6), 13);
        assert_eq!(run(15), 47);
        assert_eq!(run(100), 541);
        assert_eq!(run(1000), 7919);
        assert_eq!(run(10_001), 104_743);
    }
}
