fn main() {

    // If statements
    let one = 1;
    if one > 10 {
        println!("True");
    } else if one == 1{
        println!("Equal");
    } else {
        println!("False");
    }

    // Loop: can have labels to loops using "'" character. To break from such a loop use the "break;" statement
    let mut num = 0;
    'counter: loop {
        println!("Count: {}", num);
        let mut decrease = 5;

        loop {
            println!("Decreasing! {}", decrease);
            if decrease == 4 {
                break;
            }
            if num == 2 {
                break 'counter;
            }

            decrease -= 1; // decrease = decrease - 1
        }
        num += 1;
    }

    // While loops
    let mut num = 0;
    while num < 5 {
        println!("Num {}", num);
        num += 1;
    }

    // For loop
    let vec: Vec<i8> = (0..10).collect();

    for element in vec {
        println!("{}", element);
    }
    println!("");

    // Another for loop
    for element in (1..4).rev() {
        println!("{}", element);
    }
    println!("Boom!");
}
