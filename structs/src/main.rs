use std::io;
fn main() {
    let mut height = String::new();
    io::stdin().read_line(&mut height).expect("Failed to read height");
    let mut width = String::new();
    io::stdin().read_line(&mut width).expect("Failed to read width");
    let rect1 = Rectangle {
        height : height.trim().parse().expect("Failed to parse height"),
        width : width.trim().parse().expect("Failed to parse width")
    };
    let area = calc_area(&rect1);
    println!("Area: {}", area);
}

fn calc_area(rect1: &Rectangle) -> u32 {
    return rect1.height * rect1.width;
}

struct Rectangle {
    width: u32,
    height: u32
}
