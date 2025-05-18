pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

fn is_even(number: i32) -> bool {
    match number % 2 {
        0 => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*; // bring everything in the higher level into scope in order to use

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn it_fails() {
        let result: i64 = 2 - 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn test_is_even() {
        assert!(is_even(42));
        assert!(!is_even(13));
    }

    #[test]
    fn test_with_assert() {
        let result: u64 = add(2, 2);
        assert!(result == 4, "Expected 4; result was {}", result);
    }

    #[test]
    fn test_with_assert_eq() {
        let result: u64 = add(2, 2);
        assert_eq!(add(2, 2), 4);
    }

    #[test]
    fn test_with_assert_ne() {
        assert_ne!(add(2, 2), 3);
    }
}
