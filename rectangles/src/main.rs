#[derive(Debug)]
struct Rectangle {
    w: u32,
    h: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        // self.w * self.h
        1 + 2
    }

    fn can_hold(&self, rect: &Rectangle) -> bool {
        self.w > rect.w && self.h > rect.h
    }

    fn square(size: u32) -> Self {
        Self {
            w: size,
            h: size,
        }
    }
}


fn main() {

    let rect1 = Rectangle {
        w: 30,
        h: 50,
    };
    let rect2 = Rectangle {
        w: 10,
        h: 40,
    };
    let rect3 = Rectangle {
        w: 60,
        h: 45,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );


    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    println!("-----");
    println!("the rect: {:?}, {}, {}", rect1, rect1.w, rect1.h);
    println!("rect2: {:?}", rect2);
    println!("rect3: {:?}", rect3);
    println!("-----");
    println!("Calling associated function, square");
    let square = Rectangle::square(13);
    println!("our square Rectangle value: {:?}", square);
}
