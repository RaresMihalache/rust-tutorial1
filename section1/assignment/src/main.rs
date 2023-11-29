fn ex1(val1: i32, val2: i32) {
    println!("{}", val1 % val2);
}

fn ex2() {
    let mut vec1 = vec![2, 4, 6, 8, 10];
    println!("{:?}", vec1);
    vec1.pop();
    vec1.push(12);

    for element in vec1 {
        println!("{}", element);
    }
}

fn concat_string(mut str1: String){
    str1 += " World";
    println!("{}", str1);
}

fn ex3() {
    concat_string("Hello".to_string());
}

fn control_flow(num: i64) {
    if num == 1 {
        println!("The value is one");
    } else if num > 50 {
        println!("The value is greater than 50");
    } else if num < 25 {
        println!("The value is less than 25");
    } else {
        println!("The value is greater than 25 but less than 50");
    }
}
fn ex4() {
    control_flow(55);
}

fn main() {
    ex1(5, 2);
    println!("");
    ex2();
    println!("");
    ex3();
    println!("");
    ex4();
}


