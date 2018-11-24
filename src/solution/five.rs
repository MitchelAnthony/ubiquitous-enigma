use primal_sieve;

pub fn run(max: usize) -> usize {
    let sieve = primal_sieve::Sieve::new(max);

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_five() {
        assert_eq!(run(10), 2520);
        assert_eq!(run(20), 232_792_560);
    }
}
