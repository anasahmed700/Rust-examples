
const PI: f64 = 3.164; // constants are globally accessable & can be declare

fn main() {
    // 1: VARIABLES
    // let x = 5;  // variables are immutable by default in rust
    let mut x = 5; // adding mut keyword to make variable mutable
    println!("The value of x is: {}", x);
    x = 6;
    println!("The new value of x is: {}", x);

    // 2: CONSTANTS can't be mutable by using 'mut'
    const MAX_POINTS: u32 = 100_000; // const VAR_NAME: datatype = value
    println!("The constant value of max point is: {}", MAX_POINTS);
    println!("The constant value of PI is: {}", PI);

    // 3: SHADOWING by using let with the same variable name
    let x = 5;
    let x = x + 1; // 6
    let x = x * 2; // 12
    println!("Value of x after shadowing is: {}", x);
    let x:f64 = 4.5;
    println!("Value & datatype of x after shadowing is: {}", x);

    // 4: type restriction
    let mut spaces = "   ";
    // spaces = spaces.len(); // different datatypes can't be assigned in same variable
    println!("{}", spaces);
    spaces = "123";
    println!("{}", spaces);
}
