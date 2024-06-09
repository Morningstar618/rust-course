mod shapes {
    pub struct Circle {
        radius: i32,
    }

    impl Circle {
        pub fn new(radius: i32) -> Circle {
            Circle { radius }
        }

        pub fn new_1(radius: i32) -> Result<Circle, String> {
            if radius > 0 {
                Ok(Circle { radius })
            } else {
                Err(String::from("Radius should be positive"))
            }
        }

        pub fn new_2(radius: i32) -> Circle {
            match radius {
                -10..=0 => panic!("radius should be positive"),
                ..=-10 => panic!("is lesser than -10"),
                _ => Circle { radius },
            }
        }

        pub fn contains(&self, other: &Circle) -> bool {
            self.radius > other.radius
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Error;

    use crate::shapes::Circle;

    #[test]
    fn contains_test() {
        let circle1 = Circle::new(5);
        let circle2 = Circle::new(2);

        let result = circle1.contains(&circle2);
        assert_eq!(result, true, "Test failed");
    }

    #[test]
    fn should_not_create_circle() -> Result<(), String> {
        // This test should fail.
        let some_circle = Circle::new_1(-1)?;
        Ok(())
    }

    #[test]
    #[should_panic(expected = "is lesser than -10")]
    fn should_not_create_circle_and_panic() {
        let some_circle = Circle::new_2(-11);
    }

    #[test]
    #[ignore]
    fn huge_test() {}
}
