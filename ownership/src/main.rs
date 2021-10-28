fn main() {
    /*
    *   string mutation
    */
    // let mut s: String = String::from("Hello");
    // s.push_str(", world!");

    // println!("{}", s);

    /*
    *   copy for primitives
    */
    // let s1 = "Hello";
    // let s2 = s1;

    // println!("{}", s1);
    // println!("{}", s2);
    
    /*
    *   different types of ownership transfers with non-primitives
    */
    let s1 = give_ownership(); // ownership of x1 transferred to s1

    let s2 = String::from("hello");
    let s3 = takes_and_gives_back_ownership(s2); // ownership taken from s2 and given back to s3

    let s4 = String::from("hello again");
    let (s5, result) = ownership_and_result(s4); // returns owner ship and a result
}

fn give_ownership() -> String {
    let x1 = String::from("Hello");
    x1
}

fn takes_and_gives_back_ownership(a_string: String) -> String {
    a_string
}

fn ownership_and_result(arg1: String) -> (String, i32) {
    let i: i32 = 20;
    (arg1, i*20)
}
