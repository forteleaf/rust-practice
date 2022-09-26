fn main() {
    let rect1 = Rectangle {
        length: 50,
        width: 30,
    };
    println!(
        "The area of the rectangle is {} squre pixels.",
        area(&rect1)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.length * rectangle.width
}

// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }
struct Rectangle {
    length: u32,
    width: u32,
}
