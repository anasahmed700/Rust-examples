fn main() { 

    // Referencing & borrowing
    let mut course = String::from("IOT");
    println!("String Content {}", course); // storage heap
    println!("Memory address Pointer {:p}", &course); // storage stack
    println!("Content Length {}", course.len()); // storage stack
    println!("Memory Capacity {}", course.capacity()); // storage stack
    course.clear(); // remove the content from heap memory

    println!("After clear");
    println!("String Content {}", course); // storage heap
    println!("Memory address Pointer {:p}", &course); // storage stack
    println!("Content Length {}", course.len()); // storage stack
    println!("Memory Capacity {}", course.capacity()); // storage stack

    course.push_str("IOT Batch 7");
    // referencing memory address 
    let new_course = &course; // only copy the pointer
    println!("After address");
    println!("String Content {}", new_course); // storage heap
    println!("new_course Memory address Pointer {:p}", &new_course); // have two addresses 1st own new_course address 
    println!("course Memory address Pointer {:p}", new_course); // 2nd reference address of course  
    println!("Content Length {}", new_course.len()); // storage stack
    println!("Memory Capacity {}", new_course.capacity()); // storage stack

    // referencing 1
    let name = String::from("Anas");
    move_ownership(&name); // String reference as argument
    println!("after move_ownership fn call {}", name); // as we use referencing 'name' is still valid after function call

    // referencing book example
    let str = String::from("Hello!");
    let len = calculate_length(&str);
    println!("str is {} & its length is {}", str, len);

    // Mutable References
    let mut food = String::from("Biryani");
    println!("Food: {}", food);
    mut_ref(&mut food);
    println!("Food: {}", food);
    food.clear();
    mut_ref(&mut food);
    println!("Food: {}", food);

    // references Scope
    let mut s = String::from("references Scope");
    let r1 = &s;
    let r2 = &s;
    println!("r1 and r2 are {} & {}", r1, r2);
    // r1 and r2 are no longer used after this point
    let r3 = &mut s;
    // let r4 = &mut s; // we can't hove more than one mutable reference
    println!("r3 is {}", r3);
    println!("r3 can be used as {}", r3);
    
}

fn move_ownership(data: &String){ // taking string reference type
    println!("from move_ownership function: {}", data);
}
fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, nothing happens.
fn mut_ref(data: &mut String){
    data.push_str(" Halwa + 7up");
}
