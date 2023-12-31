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

    // Vectors -> resizable array of elements allocated on the heap. Use it when you need a list of dynamic size
    // Create a vector using the vec! macro
    let mut nums = vec![1, 2, 3];

    nums.push(4);
    // println!("{}", nums); // error ^^^^ `Vec<{integer}>` cannot be formatted with the default formatter
    // Use :? for debug mode in formatter
    println!("{:?}", nums);

    nums.pop(); // remove last element
    println!("{:?}", nums);

    // another way to create vectors
    let mut vec = Vec::new(); // when using the "vec!" macro what that actually does is calin the Vec::new() constructor
    vec.push("test");
    vec.push("string");
    println!("{:?}", vec);

    vec.reverse();
    println!("{:?}", vec);

    let mut vect = Vec::<i32>::with_capacity(2);
    vect.push(4);
    vect.push(4);
    vect.push(4);
    println!("{:?}", vect.capacity());
    println!("{:?}", vect);

    let v: Vec<i32> = (0..5).collect(); // construct a vector with an iterator that has the elements from [0, 5)
    println!("{:?}", v);

    // Slices -> region of an array / vector that can be any length.
    // Slices can not be stored directly into a variable or passed as function arguments
    // Crate a slice
    let sv: &[i32] = &v; // point the slice to the address of memory where the slice starts -> point sv to where our slice starts (the vector). The reference to the slice is called a "fat pointer"
    println!("{:?}", sv); // prints out the content of vector, because we point to the beginning of the vector

    // If we want we can point to some custom slice
    let sv: &[i32] = &v[2..4]; // point directly to the data in vector v - not owning reference!
    println!("{:?}", sv); // [2, 3]

    // An ordinary reference is a non-owning pointer to a single value
    // A reference to a slice is a non-owning pointer to a range of consecutive values.

    // Strings and &str
    // Strings are very similar to vectors (stored as vector of bytes)
    // Strings are allocated on the heap, growable, NOT NULL terminated!

    let name = String::from("Tyler"); // 1st example of allocating a string
    let course = "Rust".to_string(); // 2nd example of allocating a string
    let new_name = name.replace("Tyler", "Ty");

    println!("{}", name);
    println!("{}", course);
    println!("{}", new_name);

    // &str = "string slice" or "stir" -> reference & borrow the text from a variable - "fat pointer" - contains both the address and the actual data and its length.
    // you can not modify a string slice.
    let str1 = "hello"; // &str
    let str2 = str1.to_string(); // converted &str to string
    let str3 = &str2; // converted string "str2" to string slcie "&str2"
    // Strings vs String Slices:
    // String slices are more appropriate for function arguments, when the caller should be allowed to pass either kind of string (string | string literal).
    // String slice DOES NOT allocate memory on the heap, while the string DOES.

    println!("{}", str1); // hello
    println!("{}", str2); // hello
    println!("{}", str3); // hello

    // Like vectors, strings have a lot of methods associated to them
    // compare strings with "==". Not equal with "!="
    println!("{}", "ONE".to_lowercase() == "one");

    // String literals
    // Strings and string slices are ALWAYS going to be a VALID UTF-8 sequence.
    // Sometimes you might NOT want a VALID UTF-8 sequence, and this is where you USE "string literals"
    let rust = "\x52\x75\x73\x74"; // sequnece of string literals
    println!("{}", rust); // Rust
}
