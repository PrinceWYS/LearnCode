fn main() {
    // number: signed integer(i8, i16, i32, i64, isize), unsigned integer(u8, u16, u32, u64, usize), floating point(f32, f64)
    // string: &str, String
    // boolean: bool
    // char: char
    // unit: ()
    println!("Hello, world!");

    // number
    // integer
    let a : u8 = 255;
    let b = a.wrapping_add(20);
    println!("a: {}, b: {}", a, b);

    // float
    let abc : (f32, f32, f32) = (0.1, 0.2, 0.3);
    let xyz : (f64, f64, f64) = (0.1, 0.2, 0.3);

    println!("abc (f32)");
    println!("0.1 + 0.2 = {:x}", (abc.0 + abc.1).to_bits());
    println!("      0.3 = {:x}", (abc.2).to_bits());
    println!("");

    println!("xyz (f64)");
    println!("0.1 + 0.2 = {:x}", (xyz.0 + xyz.1).to_bits());
    println!("      0.3 = {:x}", (xyz.2).to_bits());
    println!("");

    // NaN
    let x = (-42.0_f32).sqrt();
    if x.is_nan() {
        println!("x is NaN")
    }


    // caculate
    let twenty = 20;
    let twenty_one = 21;
    let twenty_two = 22i32;
    //same type can be added
    let sum = twenty + twenty_one + twenty_two;
    println!("{} + {} + {} = {}", twenty, twenty_one, twenty_two, sum);

    let one_million: i64 = 1_000_000;
    println!("{}", one_million.pow(2));

    let forty_twos = [
        42.0,
        42f32,
        42.0_f32,
    ];
    println!("{:.2}", forty_twos[1]);

    // bit operation
    let a = 2;  // 0000 0010
    let b = 3;  // 0000 0011
    println!("a & b = {}", a & b);  // 0000 0010
    println!("a | b = {}", a | b);  // 0000 0011
    println!("a ^ b = {}", a ^ b);  // 0000 0001
    println!("!b = {}", !b);        // 1111 1100
    println!("a << b = {}", a << b);  // 0000 1000
    println!("a >> b = {}", a >> b);  // 0000 0000

    let mut a = a;
    a <<= b;
    println!("a <<= b = {}", a);

    // range
    for i in 1..5 {
        println!("{}", i);
    }
    for i in 'a'..'z' {
        println!("{}", i);
    }

    // char
    let c = '中';
    println!("'中' takes {} bytes", std::mem::size_of_val(&c));

    // expression
    let y = {
        let x = 3;
        x + 1
    };
    println!("y = {}", y);

    // function
    another_function(5, 6.1)
}


fn another_function(x: i32, y: f32) {
    println!("x = {}, y = {}", x, y);
}