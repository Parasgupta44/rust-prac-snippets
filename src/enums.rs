#![allow(dead_code)]
enum Direction {
    Up(Point),
    Down(Point),
    Left(Point),
    Right(Point),
}

struct Point {
    x: i32,
    y: i32,
}

// enum Option {

// }

fn div(x: f64, y: f64) -> Option<f64> {
    if y == 0.0 {
        None
    } else {
        Some(x / y)
    }
}

fn main() {
    let v = Direction::Up(Point { x: 0, y: 1 });
    let res = div(5.0, 7.0);
    match res {
        Some(x) => println!("{:.7}", x),
        None => println!("Zero error"),
    }
}
