use std::fmt::{Debug, Display};
use generic_data_types::{Summary, Tweet, NewsArticle};

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

struct Point2<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point2<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point2<X2, Y2>) -> Point2<X1, Y2> {
        Point2 {
            x: self.x,
            y: other.y,
        }
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
    println!("Distance from origin {}", fp.distance_from_origin());

    let p1 = Point2 { x: 5, y: 5.3 };
    let p2 = Point2 { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    let tweet = returns_summarizable(true);
    notify(&tweet);
    let article = returns_summarizable(false);
    notify(&article);
}

// pub fn notify(item: &impl Summary) {
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify_2(item: &(impl Summary + Display)) {
    todo!("")
}

pub fn notify_2_1<T: Summary + Display>(item: &T) {
    todo!("")
}

fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
    todo!("")
}

fn some_function_2<T, U>(t: &T, u: &U) -> i32
    where
        T: Display + Clone,
        U: Clone + Debug,
{
    todo!("")
}

fn returns_summarizable(is_tweet: bool) -> impl Summary {
    // if is_tweet {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from(
                "of course, as you probably already know, people"
            ),
            reply: false,
            retweet: false,
        }
    todo!("Wouldn't work in this form of expression")
    // } else {
    //     NewsArticle {
    //         headline: String::from("Penguins win the Stanley Cup Championship!"),
    //         location: String::from("Pittsburgh, PA, USA"),
    //         author: String::from("Iceburgh"),
    //         content: String::from(
    //             "The Pittsburgh Penguins once again are the best \
    //         hockey team in the NHL."),
    //     }
    // }
}
