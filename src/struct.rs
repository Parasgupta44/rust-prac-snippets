struct Point {
    h: u32,
    w: u32,
}
impl Point {
    fn area(&self) -> u32 {
        self.w * self.h
    }
    fn new(h: u32, w: u32) -> Point {
        Point { h: h, w }
    }
    fn show(&self) {
        println!("{}.........{}", area(&self), self.area());
    }
}

fn area(pt: &Point) -> u32 {
    pt.w * pt.h
}

fn main() {
    let pt = Point { w: 4, h: 8 };
    println!("{}.........{}", area(&pt), pt.area());
    let obj = Point::new(8, 44);
    println!("{}.........{}", area(&obj), obj.area());
    obj.show();
}
