pub fn run(count: usize) -> usize {
    let mut triangle_sum = 0;
    for i in 1.. {
        triangle_sum += i;

        if count_divisors(triangle_sum) > count {
            break;
        }
    }

    triangle_sum
}

fn count_divisors(number: usize) -> usize {
    // Start at one because it is also divisable by itself
    let mut divisor_count = 0;
    for i in 1..=(number as f32).sqrt().floor() as usize {
        if number % i == 0 {
            divisor_count += 1;

            if i != number / i {
                divisor_count += 1;
            }
        }
    }

    divisor_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_twelve() {
        assert_eq!(run(3), 6);
        assert_eq!(run(5), 28);
        assert_eq!(run(10), 120);
        assert_eq!(run(100), 73_920);
        // This test takes a few seconds on my system, so it's disabled by default
        // assert_eq!(run(500), 76_576_500);
    }
}
