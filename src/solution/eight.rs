use std::fs;

pub fn run(count: usize) -> u64 {
    let mut product = 1;
    let contents = fs::read_to_string("./src/resources/eight.txt")
        .expect("Error reading file!");
    
    for index in 0..(contents.len() - count) {
        let test_product = string_to_product(&contents[index..(index+count)]);
        if test_product > product {
            product = test_product;
        }
    }

    product
}

fn string_to_product(digits: &str) -> u64 {
    let mut product = 1;

    for d in digits.chars() {
        product *= d.to_digit(10).unwrap() as u64;
    }

    product
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eight() {
        assert_eq!(run(4), 5832);
        assert_eq!(run(8), 7_838_208);
        assert_eq!(run(11), 940_584_960);
        assert_eq!(run(13), 23_514_624_000);
    }
}
