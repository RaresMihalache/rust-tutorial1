fn main() {

    // BASIC DATA TYPES
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

    // COMPOUND DATA TYPES
    // Tuples -> fixed length (can not grow in size). Can consist of different types
    let tup = (500, "hi", true);
    println!("{}", tup.2);
    let (x, y, z) = tup;

    println!("{}", x); // 500
    println!("{}", y); // hi
    println!("{}", z); // true

    // Arrays -> fixed length (can not grow in size). It MUST have elements that are the same data-type.
    let array = [1, 2, 3];
    println!("{}", array[0]);

    let mut array2: [i32; 3] = [4, 5, 6];
    println!("{}", array2[0]);
    array2[0] = 10;
    println!("{}", array2[0]);
}
