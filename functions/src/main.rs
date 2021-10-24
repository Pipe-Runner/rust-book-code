fn main() {
    another_function(5);

    let x = five();
    let y = plus_one(x as i32);

    println!("The final number {}", y);
}

// function hoisting supported
fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}

/*
    A function as an expression
*/
fn five() -> u8 {
    5
}

/*
    A function with args and return
*/
fn plus_one(x: i32) -> i32 {
    x + 1
}
