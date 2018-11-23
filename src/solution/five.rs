pub fn run(min: u32, max: u32) -> u32 {
    

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_five() {
        assert_eq!(run(1, 10), 2520);
        assert_eq!(run(1, 20), 232_792_560);
    }
}
