fn main() {
    let scale = 2;

    let rect1 = Rectangle {
        width: dbg!(scale * 2),
        height: 4
    };

    // let area = get_area(&rect1);
    let area = rect1.area();

    println!("{}", area);

    // :? is a specifier for Debug
    println!("{:?}", rect1);

    dbg!(&rect1);
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(self: &Self) -> u32 { // or &self in shorthand
        self.height * self.width
    }
}

fn get_area(rectangle: &Rectangle) -> u32 {
    rectangle.height * rectangle.width
}
