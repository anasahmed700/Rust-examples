fn main() {
    // 4. ownership & functions
    let s = String::from("function ownership string");// s comes into scope
    println!("passing s: String as arg to the function takes_ownership(s)");
    takes_ownership(s);// ownership of s moves to takes_ownership()...and so is droped with function scope
    // println!("{}", s); // error because String(heap) does not implement "copy" trait & droped after running in function
    
    let x = 5;
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
    println!("Taking ownership from a function arg...");
    let txt1 = String::from("Text 1");
    let (txt2, length) = calculate_len(txt1);
    // println!("txt1 is {}", txt1); // error because txt2 takes txt1's content ownership
    println!("The txt2 is '{}' & its len is {}.", txt2, length);

    println!("creating reference without taking ownership...");
    let s1 = String::from("Hello");
    // ampersands "&" are references that allow you to refer to some value without taking ownership of it.
    let (s2, len) = ref_calculate_len(&s1); 
    println!("s1 is {}", s1); // runs without loosing content ownership
    println!("s2 is '{}' & its len is {}.", s2, len); 
}
fn takes_ownership(some_str: String){ // some_str takes the ownership from argument
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
    str // returns ownership
} 

fn calculate_len(s: String) -> (String, usize){
    let length = s.len();
    (s, length)
}
    //        function parameters borrowing
fn ref_calculate_len(s: &String) -> (&String, usize) { // s is a reference to a String not its owner
    let length = s.len();
    (s, length)
}// Here, s goes out of scope. But because it does not have ownership of what it refers to, nothing happens.
