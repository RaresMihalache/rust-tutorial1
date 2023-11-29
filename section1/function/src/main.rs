fn main() {
    print_phase("This is my function");
    println!("{}", gcd(20, 4));
    println!("{}", multiple_return_values(false));
}

fn print_phase(phrase: &str){
    println!("{}", phrase);
}


// Return value by not putting ";" at the end
fn gcd(mut a: u64, mut b: u64) -> u64 {
    while a != 0 {
        if a < b {
            let c = a;
            a = b;
            b = c;
        }
        a = a % b;
    }
    b
}

// Multiple return values
fn multiple_return_values(flag: bool) -> bool {
    if flag == true {
        true
    } else {
        false
    }
}