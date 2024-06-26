use std::fmt;

#[derive(Debug)] // The debug trait automatically implement a Trait to print and thus debug our struct.
                 // here because only <T> x and why must be of same data type !!!
pub struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    /* Constructor */
    pub fn new(x: T, y: T) -> Point<T> {
        Point { x, y }
    }

    /* Getter */
    pub fn get_x(&self) -> &T {
        &self.x
    }

    pub fn get_y(&self) -> &T {
        &self.y
    }
}

/* Implement Point only for x and y being f32 */
impl Point<f32> {
    pub fn distance_from_origin(&self) -> f32 {
        (&self.x.powi(2) + &self.y.powi(2)).sqrt()
    }
}

// Implementing Display trait for Point<T>, it is like "overwritting the print" in C for our own Struct
// It is for user-facing output purpose
impl<T: fmt::Display> fmt::Display for Point<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
