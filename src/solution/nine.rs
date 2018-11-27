pub fn run(sum: usize) -> usize {
    let mut product = 1;

    // Use Euclid's formula to generate triples
    for n in 1..sum {
        for m in n..(sum + 1) {
            let a = (m * m) - (n * n);
            let b = 2 * m * n;
            let c = (m * m) + (n * n);

            if a + b + c == sum {
                product = a * b * c;
                break;
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
        assert_eq!(run(30), 780);
        assert_eq!(run(90), 14_760);
        assert_eq!(run(154), 120_120);
        assert_eq!(run(684), 3_779_100);
        assert_eq!(run(1000), 31_875_000);
    }
}
