// primitive datatypes
fn main() {
    // scalar datatypes
    let age: u32 = 27; // unsigned integer
    let temperature = -5; // signed integer by default is i32
    let percentage = 80.4; // float type by default is f64
    let grade = 'A'; // char datatype
    let passed = true; // bool datatype
    println!(
        "{} {} {} {} {}",
        age, temperature, percentage, grade, passed
    );

    // Number system in Rust
    let decimal = 100;
    let hexa_decimal = 0xff;
    let octal = 0o77;
    let binary = 0b10101010;
    let ascii = b'A';

    println!("Decimal is {}", decimal);
    println!("Hexa Decimal(0xff) is {}", hexa_decimal);
    println!("Octal(0o77) is {}", octal);
    println!("Binary(0b10101010) is {}", binary);
    println!("ASCII(A) is {}", ascii);

    // compound datatypes
    // 1: Tuples 
    println!("Tuple datatype can store multiple types of data");
    let student:(i32, char, f64) = (25, 'A', 54.2);// type annotation (type, type, type)
    // for pretty print compound elements 
    println!("{:#?}", student);
    // for regular print
    println!("{:?}", student);
    // to access a single element 
    println!("{}", student.0);
    println!("{}", student.1);
    println!("{}", student.2);

    println!("Destructuring tuple...");
    let (x, y, z) = student;
    println!("{}", x);
    println!("{}", y);
    println!("{}", z);

    // 2: Arrays 
    println!("Array can store multiple elements with same datatype...");
    let lottery_number: [u32;5] = [4,6,7,9,2]; // type annotation [type; array-length]
    println!("{:?}", lottery_number);
    
    let coin = [5; 10]; // to store same value in array [value; array-length]
    println!("{:?}", coin);

    let days = ['M', 'T', 'W']; // accessing array elements
    println!("{:#?}", days[1]);

    let aftari = ["Mix chat", "Samosay", "Pakorey", "Dahi Baray"];

    // iterator
    for num in aftari.iter() { 
        println!("Iteration {} ", num);
    }

    for num in (1..=10).rev() {
        println!("on index {}", num);
    }
    

    // slicing 
    let games = "Cricket & football";
    let game1 = &games[0..7];
    println!("game1 is {}", game1);
    let game2 = &games[10..=17];
    println!("game2 is {}", game2);


    // Panic handling
    for num in 0..5 {
        println!("After index {} is {:?}", num, aftari.get(num));
    }

    let mut score: u8 = 0;
    loop {
        println!("Your score is {}", score);
        score += 1;
        for _delay in 0..100000 {
            
        }
    }
}
