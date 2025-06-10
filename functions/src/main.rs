fn main() {
    another_funtion(5, 'h');

    let x = five();

    println!("The value of x is: {x}");
}

fn another_funtion(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}.");
}

fn five() -> i32 {
    5
}
