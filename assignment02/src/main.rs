fn main() {
    let int = 10;
    let flt = 25.35;
    let mut some_string = String::from("Anas Ahmed");

    println!("integer: {}, float: {}, string: {}", int, flt, some_string);

    // calling myfunc & storing return value in variable
    let flt_returned = myfunc(int, flt, &mut some_string);
    println!("float returned by myfunc: {}", flt_returned);
    
}

fn myfunc(int: i32, flt: f64, some_string: &mut String) -> f64 {
    some_string.push_str(" Hello!"); // string append with mutable reference
    println!("string is appended to {}", some_string);

    for i in 0..int {
        println!("iteration {}", i);// int iteration
    }

    flt*flt // return float square
}
