use std::f32;

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

pub fn is_palindrome(number: u32) -> bool {
    let mut test = number;
    let mut palindrome = 0;
    let mut magnitude = (test as f32).log10().floor() as u32;

    while test > 0 {
        palindrome += (test % 10) * 10u32.pow(magnitude);
        test /= 10;

        magnitude = if magnitude > 1 {
            magnitude - 1
        } else {
            0
        };
    }

    number == palindrome
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_prime() {
        // All primes
        assert_eq!(is_prime(11), true);
        assert_eq!(is_prime(107), true);
        assert_eq!(is_prime(397), true);
        assert_eq!(is_prime(463), true);
        assert_eq!(is_prime(977), true);

        // // All non-primes
        assert_eq!(is_prime(10), false);
        assert_eq!(is_prime(138), false);
        assert_eq!(is_prime(405), false);
        assert_eq!(is_prime(558), false);
        assert_eq!(is_prime(674), false);
    }

    #[test]
    fn test_is_palindrome() {
        // Palindromes
        assert_eq!(is_palindrome(191), true);
        assert_eq!(is_palindrome(5665), true);
        assert_eq!(is_palindrome(98789), true);

        // Non-palindromes
        assert_eq!(is_palindrome(1234), false);
        assert_eq!(is_palindrome(4343), false);
        assert_eq!(is_palindrome(56789), false);
        assert_eq!(is_palindrome(456754), false);
        assert_eq!(is_palindrome(778899), false);
    }
}