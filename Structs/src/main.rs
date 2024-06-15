mod main2;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}


fn main() {
    let width1 = 30;
    let height1 = 50;
    println!(
        "The area of the rectangle is {} square pixels",
        area(width1, height1)
    );

    // Tuples ---------
    let rect1 = (30, 50);
    println!(
        "The area of the rectangle is {} square pixels",
        area_tuple(rect1)
    );

    // Structs -------
    let rect1 = Rectangle {
        width: 30,
        height: 50
    };
    println!(
        "The area of the rectangle is {} square pixels",
        area_struct(&rect1)
    );

    println!("Rect1 is {:?}", rect1);

    let scale = 2;
    let rect = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    dbg!(&rect);

    println!("The area of the rectangle is {}", rect.area())
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn  area_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
