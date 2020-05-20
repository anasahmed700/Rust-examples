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
} // new datatype Food is defined
fn main() {
    let pizza = Food{
        restaurant : "Pizza Hut".to_string(),
        item : String::from("Chicken Fajita"),
        size : 9,
        price : 800,
        available : true
    };
    let karahi = Food{
        restaurant : "BBQ tonight".to_string(),
        item : String::from("Chicken Ginger"),
        size : 1,
        price : 1100,
        available : true
    };

    println!("{:?}", pizza);
    println!("{:#?}", karahi);
}
