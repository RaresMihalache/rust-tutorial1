fn main() {

    // Integers
    let x: i8 = 10;
    println!("{}", x);

    let _y: u8 = 10; // remove warnings of unused variable

    let decimal = 02_55;
    let hex = 0xff;
    let octal = 0o377;
    let binary = 0b1111_1111;

    println!("{}", decimal);
    println!("{}", hex);
    println!("{}", octal);
    println!("{}", binary);

    let byte = b'A';
    println!("{}", byte);

    // Floats
    let _x = 2.0; // f64 default
    let _y: f32 = 1.0;

    // Booleans
    let _t = true;   // Rust compiler is smart enough to infer the type
    let _f: bool = false;

    // Character
    let c = 'c';

    println!("{}", c);
}
