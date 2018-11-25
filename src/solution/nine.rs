pub fn run(sum: usize) -> usize {
    let mut product = 1;

    for n in 1..sum {
        for m in n..(sum + 1) {
            let a = (m * m) - (n * n);
            let b = 2 * m * n;
            let c = (m * m) + (n * n);

            if a + b + c == sum {
                product = a * b * c;
            }
        }
    }

    product
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nine() {
        assert_eq!(run(12), 60);
        assert_eq!(run(1000), 31_875_000);
    }
}
