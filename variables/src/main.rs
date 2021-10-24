use std::io;

fn main() {
    /*
        Immutable
    */
    // let x = 5;
    // println!("The value of x is: {}", x);

    // x = 6;
    // println!("The value of x is: {}", x);

    /*
        Mutable
    */
    // let mut x = 5;
    // println!("The value of x is: {}", x);

    // x = 6;
    // println!("The value of x is: {}", x);

    /*
        Shadowing
    */
    // let x = 5;
    // for _ in 1..5 {
    //     let x = x + 1;
    //     println!("x is now: {}", x);
    // }
    // println!("x outside: {}", x);

    /*
        Array out of bound access
    */

    let a: [u8; 5] = [1, 2, 3, 4, 5];

    println!("Please enter an array index");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {} is {}", index, element);
}
