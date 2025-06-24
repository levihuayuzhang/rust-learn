#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // &self -> self: &Self
    fn area(&self) -> u32 {
        self.height * self.width
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, target: &Rectangle) -> bool {
        self.width >= target.width && self.height >= target.width
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    // let scale = 2;
    // let rect1 = Rectangle {
    //     width: dbg!(30 * scale),
    //     height: 50,
    // };

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    // println!("rect1 is {rect1:#?}"); // stdout, reference, not take ownership
    // dbg!("rect1 is {}", rect1); // print in stderr, take ownership of expression and return the resultant value's ownership
    dbg!(&rect1);

    println!(
        "The area of the rectangle is {} square pixels!",
        // area(&rect1)
        rect1.area()
    );

    if rect1.width() {
        println!(
            "The rectangle has a nonzero width which is {}.",
            rect1.width
        );
    }

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    dbg!(Rectangle::square(70));
}
