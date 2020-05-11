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

    // 4. ownership & functions
    let s = String::from("function ownership string");// s comes into scope
    println!("passing s: String as arg to the function takes_ownership(s)");
    takes_ownership(s);// s's value moves into the function...and so is droped with function scope
    // println!("{}", s); // error because String(heap) does not implement "copy" trait & droped after running in function
    
    let x = 5;// x comes into scope
    println!("passing x: i32 as arg to the function takes_ownership(s)");
    makes_copy(x);// x would move into the function, but i32 is Copy, so it’s okay to still use x afterward
    println!("x: i32 after calling with function is {}", x); 

    // 5. Return Values and Scope
    println!("Return Values and Scope...");
    let str1 = gives_ownership(); // gives_ownership moves its return value into str1
    println!("str1 is {}", str1);
    let str2 = String::from("take and gives back a string");
    println!("str2 is {}", str2);
    let str3 = takes_and_gives_back(str2);// s2 is moved into takes_and_gives_back, which also moves its return value into s3
    println!("str3 is {}", str3);

    // 6. taking ownership of tuples
    let txt1 = String::from("Text 1");
    let (txt2, length) = calculate_len(txt1);
    // println!("{}", txt1); // error because txt2 takes txt1's content ownership
    println!("The length of '{}' is {}.", txt2, length);
} 

fn takes_ownership(some_str: String){ // some_string comes into scope
    println!("{}", some_str);
}// Here, some_string goes out of scope and `drop` is called. The backing memory is freed.

fn makes_copy(some_int: i32){ // some_integer comes into scope
    println!("x: i32 in makes_copy() is {}", some_int);
} // Here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String {
    let some_string = String::from("gives ownership string");
    some_string
}

fn takes_and_gives_back(str: String) -> String {
    str
} 

fn calculate_len(s: String) -> (String, usize){
    let length = s.len();
    (s, length)
}