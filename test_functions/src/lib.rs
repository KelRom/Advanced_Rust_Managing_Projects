pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

fn is_even(number: i32) -> bool {
    match number % 2 {
        0 => true,
        _ => false,
    }
}

pub struct Person {
    name: String,
}

impl Person {
    pub fn new(name: String) -> Person {
        if name.is_empty() {
            panic!("A Person needs a name!");
        }

        Person { name }
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

    // Should panic will pass if the test panics and fail if it does not
    #[test]
    #[should_panic(expected = "A Person needs a name!")]
    fn person_test() {
        let new_person = Person::new(String::from(""));
        assert_ne!(new_person.name, String::from("Kelvin"));
    }

    // Fails because it expected to panic and it didn't
    #[test]
    #[should_panic]
    fn person_test2() {
        let new_person = Person::new(String::from("Kelvin"));
        assert_eq!(new_person.name, String::from("Kelvin"));
    }

    #[test]
    fn person_test3() {
        let new_person = Person::new(String::from("Kelvin"));
        assert_eq!(new_person.name, String::from("Kelvin"));
    }
}
