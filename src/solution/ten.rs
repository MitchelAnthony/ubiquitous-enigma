use primal_sieve;

pub fn run(limit: usize) -> usize {
    let mut sum = 0;

    let sieve = primal_sieve::Sieve::new(limit);

    for p in sieve.primes_from(1).take_while(|x| *x < limit) {
        sum += p;
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ten() {
        assert_eq!(run(10), 17);
        assert_eq!(run(50), 328);
        assert_eq!(run(100), 1060);
        assert_eq!(run(1000), 76_127);
        assert_eq!(run(2_000_000), 142_913_828_922);
    }
}
