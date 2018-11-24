pub fn run(max: u32) -> u32 {
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

    // println!("The sum of all the even-valued terms is {}", sum);

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two() {
        assert_eq!(run(4_000_000), 4_613_732);
    }
}
