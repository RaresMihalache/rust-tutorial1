fn main() {
    // let var = 1; // var is now valid for the rest of the main function
    // // 1 value is i32 - it has a fixed size -> pushed onto the stack.
    // let mut s = "hello".to_tring(); // created on the heap
    // s.push_str(", world");

    // Move mechanism
    let x = vec!["tyler".to_string()];
    let y = x.clone(); // solve the below error of ownership by making a clone
    println!("{:?}", x); // error: borrow of moved value: `x`
    println!("{:?}", y);

    // Copy mechanism
    let x = 1;
    let y = x;
    println!("x = {}, y = {}", x, y); // this will work - copy is implemented on types that are stored on the stack. (ints, floats, booleans, chars)
    // A tuple can also have the copy trait if every element inside it implements copy.
}
// var is dropped - freed from memory
// s is dropped