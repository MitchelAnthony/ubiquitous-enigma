pub mod solution {
    pub fn one() -> u32 {
        let max = 1000;
        let mut sum = 0;

        for number in 1..max {
            if number % 5 == 0 || number % 3 == 0 {
                sum += number;
            }
        }

        println!("The sum of all the multiples of 3 or 5 below {} is {}", max, sum); 
    
        sum
    }

    pub fn two() -> u32 {
        let max = 4000000;
        let mut first = 1;
        let mut second = 2;
        let mut sum = 0;

        while second < max {
            if second % 2 == 0 {
                sum += second;
            }

            let new = first + second;
            first = second;
            second = new;
        }

        println!("The sum of all the even-valued terms is {}", sum);

        sum
    }

    pub fn three(number: u64) -> u64 {
        let mut number: u64 = number;
        let mut biggest_prime_factor = 1;
        
        let mut test = 2;
        while test < number / 2 {
            if number % test == 0 {
                number /= test;

                if is_prime(test) && test > biggest_prime_factor {
                    biggest_prime_factor = test;
                }
            }
            test += 1;
        }

        if is_prime(number) && number > biggest_prime_factor {
            biggest_prime_factor = number;
        }

        biggest_prime_factor
    }

    pub fn is_prime(number: u64) -> bool {
        let mut prime = true;

        for test in 2..number / 2 {
            if number % test == 0 {
                prime = false;
                break;
            }
        }

        prime
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        assert_eq!(solution::one(), 233168);
    }

    #[test]
    fn test_two() {
        assert_eq!(solution::two(), 4613732);
    }

    #[test]
    fn test_three() {
        assert_eq!(solution::three(600_851_475_143), 6857);
        assert_eq!(solution::three(533_556_432_567), 43_559_183);
        assert_eq!(solution::three(234_231_234_531), 2767);
    }

    #[test]
    fn test_is_prime() {
        // All primes
        assert_eq!(solution::is_prime(11), true);
        assert_eq!(solution::is_prime(107), true);
        assert_eq!(solution::is_prime(397), true);
        assert_eq!(solution::is_prime(463), true);
        assert_eq!(solution::is_prime(977), true);

        // // All non-primes
        assert_eq!(solution::is_prime(10), false);
        assert_eq!(solution::is_prime(138), false);
        assert_eq!(solution::is_prime(405), false);
        assert_eq!(solution::is_prime(558), false);
        assert_eq!(solution::is_prime(674), false);
    }
}
