pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[cfg(test)]
mod tests {
    // tests is an inner module, thus we need to bring everything of the outer module lib.rs within this child module.
    use super::*;
    #[test]
    fn it_adds_two() {
        assert_eq!(add_two(2), 4);
    }

    // run this annotated function which will be run with the CL --> cargo test
    /*#[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }*/

    /*#[test]
    fn another() {
        panic!("Make this test fail");
    }*/

    /*#[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 10,
            height: 10,
        };
        let smaller = Rectangle {
            width: 5,
            height: 5,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 10,
            height: 10,
        };
        let smaller = Rectangle {
            width: 5,
            height: 5,
        };

        assert!(smaller.can_hold(&larger));
    }*/
}
