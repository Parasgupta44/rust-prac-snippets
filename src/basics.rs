fn basics() {
    println!("Hey, is this Rust?");

    // Variables (can be mutable)
    let var = 5;
    let mut var_mut = 5;
    var_mut = 8;
    println!("{}, {}", var_mut, var);
    // Integer types - > i8, u8, i16, u16, i32, u32, i64, u64
    // f32, f64
    // chars
    let c: char = 'x';
    // let c = 'x';
    //--------------------------
    // Tuples
    let tup: (i32, f32, char) = (44, 48.45, 'x');
    let (_, _, _) = tup; // destructure
    println!("{}, {}, {}:- {:?}..... {}", tup.0, tup.1, tup.2, tup, c);
    //make sure for ? tuple not too long!
    //---------------------
    //Arrays
    let a: [i32; 4] = [1, 2, 3, 4];
    let a0 = a[0];

    println!("{}, {}", a0, a.len());
    // Slicing
    let sl = &a[2..4];
    println!("{:?}.... {:?}", a, sl);

    //----------------
    //Strings
    let s = "not_literal";
    let st = "not_literal".to_string();
    let ss = String::from("Value!!");
    println!("{:?}.... {:?}.... {}", s, ss, st + &ss);
}
