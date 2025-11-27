pub fn add(left: u64, right: u64) -> u64 {
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

#[cfg(test)]
mod tests {
    // use core::panic;

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
}
