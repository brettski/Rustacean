struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area (&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

/*
fn area2(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
*/

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };

    println!(
        "The area of the rectangle is {} square pixels.", rect1.area()
    );

    println!("Can Rectangle3 hold Rectangle1? {}", rect3.can_hold(&rect1));
    println!("And can rectangle3 hold rectangle 2? {}", rect3.can_hold(&rect2));
}
