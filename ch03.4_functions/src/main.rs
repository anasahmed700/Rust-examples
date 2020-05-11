fn main() {
    // 1: statements & expressions
    // let x = (let y = 3);  // a variable can't an statement
    let y = {
        // statements can contains expression
        let x = 3;// this whole line is statement But only 3 is expression
        x + 1 // expression always return a value
    };// statement
    println!("value of y is: {}", y); // statements always perform some action
    
    // 2:
    another_fn(5, 10);

    // 3: 
    let (value1, value2) = square(4, 5.7);
    println!("Returned values are {} & {}", value1, value2);
}
// 2: function with parameter
    // function signature
fn another_fn(x: i32, y: i16) {// function defination
    println!("Another function with values {} & {}", x, y);
}

// 3: returning functions
                        // -> (returning datatypes)
fn square(x: u32, y: f32) -> (u32, f32){
    let result1 = x * x;
    let result2 = y * y;
    (result1, result2) // returning tuples
}