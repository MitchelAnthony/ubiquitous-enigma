pub fn run(max: u32) -> u32 {
    let mut sum_of_squares = 0;
    let mut square_of_sum = 0;

    for number in 1..=max {
        sum_of_squares += number * number;
        square_of_sum += number;
    }

    (square_of_sum * square_of_sum) - sum_of_squares
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_six() {
        assert_eq!(run(10), 2640);
        assert_eq!(run(20), 41_230);
        assert_eq!(run(100), 25_164_150);
    }
}