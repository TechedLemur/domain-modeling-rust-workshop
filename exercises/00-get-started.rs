// Exercise 0: Get started with Rust
//
// Spend about five minutes trying this example. Return to it as a reference;
// you do not need to understand every line before moving on.
// For more detail, check out the Rust Book: https://doc.rust-lang.org/book/
//
// Tasks:
// 1. Click Run above main. Change the workshop name, count, and boolean; run again.
// 2. Change room from None to Some("Room A".to_string()). Run again.

// A struct groups named fields. Each field has a type.
// Debug lets us print the whole struct with {:?} or {:#?} (multiple lines).
#[derive(Debug)]
#[allow(dead_code)] // Silence the compiler warning about unused fields.
struct Workshop {
    name: String,               // Text.
    participants: u32,          // A non-negative whole number.
    is_beginner_friendly: bool, // true or false.
    room: Option<String>,       // Text that may be absent.
}

// main is the program's starting point.
fn main() {
    // let creates a variable. Rust can often infer its type from the value.
    let my_bool = true; // bool: true or false.

    // A few of Rust's number types:
    let my_number: u32 = 30; // Non-negative integer; type chosen explicitly.
    let my_negative_number = -30; // Inferred as i32.
    let my_fractional_number = 3.5; // Inferred as f64.

    // Variables cannot be reassigned by default. Add mut to allow changes.
    // Optional: remove mut, read the error, then restore it.
    let mut my_mutable_number = 30;
    println!("Before update: {my_mutable_number}");
    my_mutable_number = 40;
    println!("After update: {my_mutable_number}");

    // "Hello" has type &str. .to_string() creates the String used by our fields.
    // You do not need to understand the distinction yet.
    // More: https://doc.rust-lang.org/book/ch08-02-strings.html#creating-a-new-string
    let my_text = "Hello".to_string();

    // Rust has no general null value. Option<T> holds Some(value) or None (absent).
    // None alone cannot tell Rust the type of the missing value, so we specify it.
    let my_option: Option<String> = None;
    let another_option = Some("Room A".to_string()); // Inferred as Option<String>.

    // println! prints values; {:?} shows Option's Some(...) or None.
    println!(
        "Values: {my_bool}, {my_number}, {my_negative_number}, {my_fractional_number}, {my_text}"
    );
    println!("Options: {my_option:?}, {another_option:?}");

    // TODO: Change these values, then run again.
    let workshop = Workshop {
        name: "Domain Modeling in Rust".to_string(),
        participants: 30,
        is_beginner_friendly: true,
        room: None,
    };

    // Use a dot to read a field, or Debug to display the whole struct.
    println!("Workshop: {}", workshop.name);
    println!("All fields:\n{workshop:#?}");
}
