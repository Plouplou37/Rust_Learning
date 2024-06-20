#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    /* Methods */

    // using self: &Self means that the method is only reading value of the self instance. it does not consume its value!
    fn area(self: &Self) -> u32 {
        self.width * self.height
    }

    fn width(self: &Self) -> bool {
        self.width > 0
    }

    fn can_hold(self: &Self, other_rect: &Rectangle) -> bool {
        // Check if self can hold other_rect based on dimensions
        let can_hold_width = self.width >= other_rect.width;
        let can_hold_height = self.height >= other_rect.height;

        // Return true only if both width and height conditions are satisfied
        can_hold_width && can_hold_height
    }

    /* Associated Function */
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    /*let rect = Rectangle {
        width: 30,
        height: 50,
    };

    dbg!(&rect);
    println!("The area of rect {:?} is {}", rect, rect.area());

    println!("height is {:#?}", rect);

    if rect.width() {
        println!("The rectange has a nonzero width which is {}", rect.width);
    }*/

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 49,
    };

    println!(
        "rect1 can hold rect2 ? here is the answer --> {}",
        rect2.can_hold(&rect1)
    );

    let square = Rectangle::square(10);
    dbg!(&square);
}
