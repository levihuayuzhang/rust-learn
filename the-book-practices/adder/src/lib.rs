pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn add_two(a: u64) -> u64 {
    internal_adder(a, 2)
}

fn internal_adder(left: u64, right: u64) -> u64 {
    left + right
}

#[derive(Debug)]
struct Retangle {
    width: u32,
    height: u32,
}

impl Retangle {
    fn can_hold(&self, other: &Retangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }
}

pub fn greeting(name: &str) -> String {
    format!("Hello {name}!")
    // String::from("Hello!") // introduce a bug
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        // if value < 1 || value > 100 {
        // // bug
        // if value < 1 {
        //     panic!("Guess value must be between 1 and 100, got {value}.");
        // }
        if value < 1 {
            panic!("Guess value must be greater than or equal to 1, got {value}");
            // panic!("Guess value must be less than or equal to 100, got {value}"); // bug
        } else if value > 100 {
            panic!("Guess value must be less than or equal to 100, got {value}");
            // panic!("Guess value must be greater than or equal to 1, got {value}"); // bug
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    // #[test]
    // fn another() {
    //     panic!("Make this test fail!");
    // }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Retangle {
            width: 8,
            height: 7,
        };

        let smaller = Retangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Retangle {
            width: 8,
            height: 7,
        };

        let smaller = Retangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{result}`"
        );
    }

    #[test]
    #[should_panic(expected = "less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    #[ignore]
    fn it_works() -> Result<(), String> {
        let result = add(2, 2);

        if result == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four."))
        }
    }

    #[test]
    fn internal() {
        let result = internal_adder(2, 2);
        assert_eq!(result, 4);
    }
}
