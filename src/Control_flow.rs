fn main() {
    let f = true;
    //binding
    let n = if f { 50 } else { 76 };
    println!("n: {}", n);

    //loop
    let mut c = 0;
    loop {
        println!("running");
        c += 1;
        if c > 5 {
            break;
        }
    }
    //labels
    'a: loop {
        println!("a");
        'b: loop {
            println!("b");
            break 'b;
        }
        break 'a;
    }
    // simple while
    let mut n = 4;
    while n != 0 {
        n -= 1;
    }
    //simple for => .., ..=
    let ar = vec![10, 20, 30, 40];
    for i in ar {
        println!("i: {}", i);
    }
    //match
    let xx = 15;
    match xx {
        1 => println!("One"),
        2 | 3 | 5 | 7 | 11 => println!("Prime"),
        13..=19 => println!("YO"),
        _ => println!("Default",),
    }
}
