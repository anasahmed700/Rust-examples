use std::io;

pub fn take_input() -> String{
    let mut input = String::new(); 
    // taking input from user 
    io::stdin().read_line(&mut input).ok();
    input.trim().to_string() // returns String type
}