// Exercise 1: Spot the problem
//
// Imagine we're building an order system. We need to track whether an order
// has been paid and shipped, along with its payment and tracking information.
// Using booleans and optional fields seems reasonable, but what combinations
// does that let us create?
//
// Rust checks each field's type, but these types don't express the rules that
// connect the fields. Let's find orders that compile but shouldn't exist.
//
// Rules:
// - An order must be paid before shipping.
// - Paid orders need a payment ID; unpaid orders cannot have one.
// - Shipped orders need a tracking number; unshipped orders cannot have one.
//
// Task: create two more orders that compile but break these rules.
// Click Run above `main` and explain which rule each order breaks. There are no tests.
//
// `Option<String>` holds either `Some("value".to_string())` or `None` (no value).

#[derive(Debug)]
#[allow(dead_code)] // Hide warnings about unused fields.
struct Order {
    id: String,
    is_paid: bool,
    is_shipped: bool,
    payment_id: Option<String>,
    tracking_number: Option<String>,
}

fn main() {
    // This order has shipped without payment.
    let impossible_order_1 = Order {
        id: "123".to_string(),
        is_paid: false,
        is_shipped: true,
        payment_id: None,
        tracking_number: Some("tracking-456".to_string()),
    };

    // TODO: Uncomment both templates and replace each todo! with a value.
    // Uncomment their `println!` calls below to see both orders.
    // let impossible_order_2 = Order {
    //     id: "456".to_string(),
    //     is_paid: todo!("Fill in the missing field"),
    //     is_shipped: todo!("Fill in the missing field"),
    //     payment_id: todo!("Fill in the missing field"),
    //     tracking_number: todo!("Fill in the missing field"),
    // };

    // let impossible_order_3 = Order {
    //     id: "789".to_string(),
    //     is_paid: todo!("Fill in the missing field"),
    //     is_shipped: todo!("Fill in the missing field"),
    //     payment_id: todo!("Fill in the missing field"),
    //     tracking_number: todo!("Fill in the missing field"),
    // };

    println!("Impossible order 1: {:?}", impossible_order_1);
    // TODO: Uncomment both print calls after completing the orders.
    // println!("Impossible order 2: {:?}", impossible_order_2);
    // println!("Impossible order 3: {:?}", impossible_order_3);
}
