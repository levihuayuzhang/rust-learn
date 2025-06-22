#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    // let rect1 = Rectangle {
    //     width: 30,
    //     height: 50,
    // };

    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    // println!("rect1 is {rect1:#?}"); // stdout, reference, not take ownership
    // dbg!("rect1 is {}", rect1); // print in stderr, take ownership of expression and return the resultant value's ownership
    dbg!(&rect1);

    println!(
        "The area of the rectangle is {} square pixels!",
        area(&rect1)
    );
}

fn area(retangle: &Rectangle) -> u32 {
    retangle.height * retangle.width
}
