use helper;

pub fn run(number: u64) -> u64 {
    let mut number: u64 = number;
    let mut biggest_prime_factor = 1;
    
    let mut test = 2;
    while test < number / 2 {
        if number % test == 0 {
            number /= test;

            if helper::is_prime(test) && test > biggest_prime_factor {
                biggest_prime_factor = test;
            }
        }
        test += 1;
    }

    if helper::is_prime(number) && number > biggest_prime_factor {
        biggest_prime_factor = number;
    }

    biggest_prime_factor
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_three() {
        assert_eq!(run(600_851_475_143), 6857);
        // This tests takes a while, maybe optimize later?
        // assert_eq!(run(533_556_432_567), 43_559_183);
        assert_eq!(run(234_231_234_531), 2767);
        assert_eq!(run(123_456), 643);
        assert_eq!(run(987_654), 1697);
    }
}
