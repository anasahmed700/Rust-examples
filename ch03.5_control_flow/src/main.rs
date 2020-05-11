fn main() {
    // 1: if else-if & else
    let number = -10;
    if number < 0 {
        println!("number is negative!");
    } else if number == 0 {
        println!("number is zero");
    } else {
        println!("number is positive!");
    }

    // 2: using if in a let statement
    let condition = true;
    let x = if condition { 5 } else { 6 }; // both if-else arms should return the same datatypes
    println!("value of x is {}", x);

    // 3: Repeatation
    // 3.1: loop
    // loop {
        // println!("infinity!");
    // }
    let mut loop_counter = 0;
    let result = loop {
        loop_counter += 1;
        if loop_counter == 10 {
            break loop_counter * 2;
        }
    };
    println!("loop result is {}", result);

    // 3.2 while loop
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }
    // iterate array through while loop
    println!("using while loop to iterate array...");
    let arr = [10,20,30,40,50];
    let mut index = 0;
    while index < 5 {
        println!("array element {} is {}",index, arr[index]);
        index += 1;
    }
    
    // 3.3 for loop
    println!("using for loop to iterate array...");
    for element in arr.iter(){
        println!("{}" , element);
    }
    // using from..to range and reverse function for compound variables
    for elem in (1..4).rev() {
        println!("{}", elem);
    }

}
