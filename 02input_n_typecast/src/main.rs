mod input_module;
fn main() {
    println!("Enter your first name: ");
    let fname = input_module::take_input();
    println!("Enter your last name: ");
    let lname = input_module::take_input();
    println!("My full name is {}", full_name(&fname, &lname));
    println!("{} {}", fname, lname);

    println!("Enter num1: ");
    // type casting
    let num1: i32 = input_module::take_input().parse().unwrap();
    
    println!("Enter num2: ");
    let num2: i32 = input_module::take_input().parse().expect("Please enter numbers instead of letters!");

    println!("The addition is: {}", add_values(num2, num1));
}
fn add_values(x: i32, y:i32) -> i32{
    let result = x + y;
    result
}
fn full_name(x: &String, y: &String) -> String{
    let result = format!("{} {}", x, y);
    result
}

