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
}
