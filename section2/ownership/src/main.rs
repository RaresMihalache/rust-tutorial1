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

    // More moves
    let s = String::from("takes");
    takes_ownership(s); // "strin" from inside the function will take ownership of "s" string
    // println!("{}", s); // will get "borrow of moved values: `s`" error

    let val = 1;
    make_copy(val);

    let str1: String = give_ownership();
    println!("{}", str1);

    let str3: String = take_and_give(str1);
    println!("{}", str3);

    if true {
        let str4 = str3;
        // println!("{}", str4);
    } else {
        let str5 = str3;
        println!("{}", str5);
    }

    let mut str1 = String::from("Tyler");
    let mut str2: String;
    loop {
        str2 = str1; // ownership problem: str1 will lose reference after first iteration
    }


}

fn takes_ownership(s: String) {
    let strin = s;
    println!("{}", strin);
}

fn make_copy(one: i32) {
    let val1 = one;
    println!("{}", val1);
}

fn give_ownership() -> String {
    "given".to_string()
}

fn take_and_give(str2: String) -> String {
    str2
}
// var is dropped - freed from memory
// s is dropped