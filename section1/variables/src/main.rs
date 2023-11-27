fn main() {
    // let x = 5; -> immutable
    // let mut x = 5; -> mutable
    let mut x = 5;

    println!("The value of x is {}", x);

    x = 6;
    println!("The value of x is {}", x);

    // const -> immutable. Can not put "const mut ..(something)"
    const SECONDS: i8 = 60;
    println!("The value of seconds is {}", SECONDS);
}
