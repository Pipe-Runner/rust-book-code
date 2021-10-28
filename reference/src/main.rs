fn main() {
    /**
     * Without reference
    */
    // let s = String::from("James");
    // look(s);
    // println!("{}", s); // ownership lost, won't work as expected

    /**
     * With reference
    */
    // let s = String::from("James");
    // look_again(&s); // passing a mutable reference
    // println!("{}", s);

    /**
     * With mutable reference
    */
    let mut s = String::from("James");
    println!("{}", s);
    change(&mut s);
    println!("{}", s);
}

fn look(arg1: String) {
    println!("{}", arg1);
}

fn look_again(arg1: &String /* Accepting a mutable reference */ ) {
    println!("{}", arg1);
}

fn change(arg1: &mut String) {
    arg1.push_str(" Gamma");
}