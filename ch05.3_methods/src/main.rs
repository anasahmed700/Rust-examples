#[derive(Debug)]
struct Food {
    item: String,
    price: u32,
    size: u8,
}

fn main() {
    let biryani = Food {
        item: String::from("Chicken Biryani"),
        size: 2,
        price: 300,
    };

    biryani.billing("Hassan".to_string());

    println!("Calling associated function...");
    let pizza = Food::new("Fahita".to_string(), 1500, 16);
    pizza.billing("Pizza Boi".to_string());
    pizza.appearance();
}
// method define
impl Food {
    // keyword 'self' represents instance data
    fn billing(&self, rider: String) {
        // additional parameter 'rider'
        println!("We are in Billing method...");
        println!("Food price: {}", self.price); // field value
        println!("Rider name: {}", rider);  // parameter value
        println!("Food: {:#?}", self); // self indicates the object(pizza) itself
    }

    // associated function define
    fn new(item: String, price: u32, size: u8) -> Food {
        Food {
            item,  //: item, (using init shorthand) only can be use if field & parameter names are same as here
            price, //: price,
            size,
        }
    }
    fn appearance(&self) {
        println!(
            "The {} looks great, price: {}, size: {}",
            self.item, self.price, self.size
        )
    }
}
