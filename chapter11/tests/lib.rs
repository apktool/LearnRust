#[cfg(test)]
mod tests {
    use chapter11::{add, greeting};

    #[test]
    fn test_add() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    #[should_panic]
    fn test_panic() {
        panic!("Error occur")
    }

    #[test]
    fn test_result() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus tow does not equal four"))
        }
    }

    #[test]
    fn test_greeting() {
        let result = greeting();
        assert!(result.contains("Carol"),
                "Greeting did not contain name, value was `{}`", result)
    }
}