use std::cmp::PartialOrd;
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
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
    let number_list = vec![23, 34, 12, 44];

    println!("{}", largest(&number_list));

    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 10.0 };
    println!("{}, {}", integer.x(), float.x());
}
