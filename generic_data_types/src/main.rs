fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['a', 'b', 'c', 'd'];
    let result = largest(&char_list);
    println!("The largest char is {result}");

    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());

    let fp: Point<f32> = Point { x: 3.2, y: 5.5 };
    println!("Distance from origin {}", fp.distance_from_origin())
}
