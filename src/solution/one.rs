pub fn run(max: u32) -> u32 {
    let mut sum = 0;

    for number in 1..max {
        if number % 5 == 0 || number % 3 == 0 {
            sum += number;
        }
    }

    println!("The sum of all the multiples of 3 or 5 below {} is {}", max, sum); 

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        assert_eq!(run(1000), 233_168);
    }
}
