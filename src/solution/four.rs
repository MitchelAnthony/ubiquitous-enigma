use helper;

pub fn run(lower: u32, upper: u32) -> u32 {
    let mut biggest_palindrome = 0;

    for outer in lower..=upper {
        for inner in lower..=upper {
            let test = outer * inner;
            if test < biggest_palindrome {
                continue;
            }
            
            if helper::is_palindrome(test) {
                biggest_palindrome = test;
            }
        }
    }

    biggest_palindrome
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_four() {
        assert_eq!(run(10, 99), 9009);
    }
}
