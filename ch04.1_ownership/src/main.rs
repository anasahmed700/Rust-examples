fn main() {
    // types of string (&str and String)
    // 1. hard coded string literal (&str) are immutable which stores memory on the stack which known at compile time
    let mut _primitive_str = "Hello Literals"; 
    // _primitive_str = _primitive_str + "some"; // can't concatenate

    // 2. string complex (String) are mutable which allocate memory at runtime on the heap which unknown at compile time
    let mut complex_str = String::from("Hello Complex");
    println!("{}", complex_str);
    complex_str.push_str(" World!"); // push_str() appends a literal to a String
    println!("{}", complex_str);
    complex_str = complex_str + " with concatenation"; // also can concatenate
    println!("{}", complex_str);
    
    // 2 Memory & allocation
    if true{
        let s = String::from("new string");// s is valid from this point forward which stored in heap memory
        println!("{}", s);
    } //When a variable goes out of scope, Rust calls a special function called "drop" to cleans up the heap memory for variable 

    let x = 5;
    let y = x; // binding y with x
    println!("x = {} y = {}",x, y); // runs smoothly 

    // 3. ownership moved
    let s1 = String::from("hello"); // String store in two parts: stack(pointer,len,capacity) heap(string contents)
    let s2 = s1; // here data from s1 stack is copied to s2 but not from heap that's why s1 is invalid now (s1 moved to s2)
    println!("{}", s2); // new owner s2
    // println!("{}", s1); // gets error
    
    // deeply copy the heap data 
    let x1 = String::from("World");
    let x2 = x1.clone(); // here is copied both stack & heap data 
    println!("x1 = {} x2 = {}", x1, x2);
    
} 

