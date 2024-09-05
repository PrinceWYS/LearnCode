struct Struct {
    e: i32
}

fn main() {
    // "mut" := mutable
    let mut x = 5;
    println!("x = {}", x);
    x = 6;
    println!("x = {}", x);

    // use "_" to define unused varibles
    let _y = 6;

    // a is immutable, b is mutable
    let (a, mut b) = (true, false);
    println!("a = {}, b = {}", a, b);
    b = true;
    println!("a = {}, b = {}", a, b);
    assert_eq!(a, b);

    let (aa, bb, cc, dd, e);
    (aa, bb) = (1, 2);
    [cc, .., dd, _] = [1, 2, 3, 4, 5];
    Struct { e, .. } = Struct { e: 5 };
    assert_eq!((aa, bb, cc, dd, e), (1, 2, 1, 4, 5));

    // constant
    // cannot use mut
    // cannot use let
    const MAX_POINTS: u32 = 100_000;
    println!("MAX_POINTS = {}", MAX_POINTS);

    //shadowing
    // allow same name variable; later will shadow the previous one
    let z = 5;
    let z = z + 1;
    {
        let z = z * 2;
        println!("In the innner scope z = {}", z);
    }
    println!("z = {}", z);
    // different type
    let spaces = "    ";
    let spaces = spaces.len();
    println!("spaces = {}", spaces);

}
