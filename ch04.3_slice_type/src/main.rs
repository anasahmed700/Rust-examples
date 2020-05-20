fn main() {
    let name = String::from("Anas Ahmed");
    let fname = &name[0..4]; // [start..end] range syntax
    let lname = &name[5..];

    println!("{} {}", fname, lname);


    let mut s = String::from("hello world");
    let word = first_word(&s);
    println!("{}", word);
    s.clear();
    // println!("{}", word);

    let my_string = String::from("Hey dude!");
    // first_word works on slices of `String`s
    let word = first_word(&my_string[..]); // changing String to string literal by slicing
    println!("{}", word);
    
    let my_string_literal = "Hi string literals";
    // first_word works on slices of string literals
    let word = first_word(&my_string_literal[..]);
    println!("{}", word);
    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(&my_string_literal);
    println!("{}", word);

}

fn first_word(s: &str) -> &str{
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
