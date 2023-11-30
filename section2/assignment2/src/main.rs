fn main() {
    let mut vector1 = vec![1, 3, 5, 7];
    let b: bool = vec_function(&vector1);
    println!("{:?}", b);
    vector1.push(15);
    println!("{:?}", vector1);

    let mut value = 2;
    value = add_two(value);
    println!("{}", value);

}

fn vec_function(val: &Vec<i8>) -> bool{
    if val[0] == 1 {
        true
    } else {
        false
    }
}

fn add_two(addition: i8) -> i8 {
    addition + 2
}