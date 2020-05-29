// 1. structs are user defined datatypes
// 2. stored in heap memory

// defining a struct
#[derive(Debug)]
struct Food {
    restaurant : String, 
    item : String, 
    size : u8,
    price : u16,
    available : bool
} // new datatype Food is defined (blueprint)
#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}
// Tuple structs 
#[derive(Debug)]
struct Rgb(u8, u8, u8);
#[derive(Debug)]
struct Point(u8, u8, u8);

fn main() {
    // creating instance of Food struct
    let pizza = Food{
        restaurant : "Pizza Hut".to_string(),
        item : String::from("Chicken Fajita"),
        size : 9,
        price : 800,
        available : true
    };
    // mutable struct 
    let mut karahi = Food{
        available : true,
        restaurant : String::from("BBQ tonight"),
        // taking field value from another instance
        price : pizza.price,
        item : "Chicken Ginger".to_string(),
        size : 1
    };

    let biryani = Food{
        restaurant: String::from("Student Biryani"),
        item: String::from("Beef Biryani"),
        ..karahi // Creating Instances From Other Instances With Struct Update Syntax
    };
    println!("Karahi: {:#?}", karahi);
    karahi.price = 1100; // mutable struct value is changed
    println!("Karahi {} price is {}", karahi.item, karahi.price);
    println!("Biryani: {:#?}", biryani);
    println!("{} price is {}", biryani.item, karahi.price);

    
    println!("Struct with functions...");
    func_struct(pizza); // here pizza moved to func_struct scope
    // println!("{:#?}", pizza); // error borrowing moved value

    println!("Struct in function return...");
    println!("{:#?}", struct_in_fn());

    let username = String::from("anasahmed700");
    let email = String::from("anasahmed700@gmail.com");
    println!("User details: {:#?}", build_user(email, username));

    let white = Rgb(255, 255, 255);
    let origin = Point(0, 0, 0);
    println!("RGB Color values: {:?} Coordinates: {:?}", white, origin)
} 
// using struct with function
fn func_struct(data: Food){
    println!("restaurant => {}", data.restaurant);
    println!("item => {}", data.item);
    println!("price => {}", data.price);
}
// struct in function return
fn struct_in_fn() -> Food{
    let chai = Food{
        available : true,
        restaurant : String::from("Baba ka dhaba"),
        price : 100,
        item : "Doodh patti".to_string(),
        size : 2
    };
    chai
}

fn build_user(email: String, username: String) -> User{
    User{
        // Using the Field Init Shorthand when Variables and Fields Have the Same Name
        email,
        username,
        active: true,
        sign_in_count: 1
    }
}
