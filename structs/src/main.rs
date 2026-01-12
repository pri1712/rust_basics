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
    println!("rect1 is {:#?}",rect1);
    let area = rect1.calc_area();
    println!("Area: {}", area);
}

impl Rectangle {
    fn calc_area(self: &Rectangle) -> u32 {
        self.height * self.width
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}
