fn by_value(mut a: i32, b: i32) {
    a += 10;
    println!("{}.....{}", a, b);
}

fn main() {
    // Borrowing
    let s = String::from("value");
    let p = &s;
    println!("{}....{}", s, p);

    //In funs
    let a = 5;
    let b = 44;
    by_value(a, b);
    println!("{}.....{}", a, b);
}
