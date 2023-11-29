fn main() {
    let var = 1; // var is now valid for the rest of the main function
    // 1 value is i32 - it has a fixed size -> pushed onto the stack.
    let mut s = "hello".to_tring(); // created on the heap
    s.push_str(", world");
}
// var is dropped - freed from memory
// s is dropped