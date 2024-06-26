use std::fmt;

#[derive(Debug)]
pub struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    /* Constructor */
    pub fn new(x: T, y: U) -> Point<T, U> {
        Point { x, y }
    }

    /** Getter */
    pub fn get_x(&self) -> &T {
        &self.x
    }
    pub fn get_y(&self) -> &U {
        &self.y
    }
}

impl<X1, Y1> Point<X1, Y1> {
    pub fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point::new(self.x, other.y)
    }
}
