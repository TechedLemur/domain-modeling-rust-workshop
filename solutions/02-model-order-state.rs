// Click Run above `main` for the demo; Run Tests above `mod tests` checks all tests.
// Use Run Test above a single test while working.

#![allow(dead_code)]

// Solution 2: Model order state
//
// In exercise 1, separate flags let us say an order was shipped but unpaid.
// What if we represented its state as a single choice instead?
//
// An enum lists alternatives, called variants. An `OrderStatus` value is exactly
// one of `Created`, `Paid`, or `Shipped`, so the conflicting flags are gone.
// A `match` expression handles each alternative and produces a value. Each arm
// uses `pattern => expression`; Rust checks that we cover every possibility.
//
// Let's use `match` to describe the states, then add another. The payment and
// tracking fields are still separate; we'll explore what that leaves possible.
//
// Tasks:
// 1. Complete `describe_status` so the remaining arms return "paid" and
//    "shipped". Run the tests.
// 2. Add `Cancelled` to `OrderStatus` (cancellation before payment). Check the
//    compiler error in `describe_status`, then handle it by returning "cancelled".
//    Add a test for that case, following the existing tests.
//
// Done: all tests pass, including your new cancellation test.
//
// Before moving on: the state is now a single choice, but `payment_id` is still
// optional. Can this model guarantee that every paid order has a payment ID?
// We'll address this in exercise 3.

#[derive(Debug, PartialEq)]
enum OrderStatus {
    Created,
    Paid,
    Shipped,
    Cancelled,
}

#[derive(Debug)]
struct Order {
    id: String,
    status: OrderStatus,
    payment_id: Option<String>,
    tracking_number: Option<String>,
}

fn describe_status(status: &OrderStatus) -> &str {
    match status {
        OrderStatus::Created => "created",
        OrderStatus::Paid => "paid",
        OrderStatus::Shipped => "shipped",
        OrderStatus::Cancelled => "cancelled",
    }
}

fn main() {
    // These examples show how to construct each state.
    let orders = [
        Order {
            id: "123".to_string(),
            status: OrderStatus::Created,
            payment_id: None,
            tracking_number: None,
        },
        Order {
            id: "456".to_string(),
            status: OrderStatus::Paid,
            payment_id: Some("payment-789".to_string()),
            tracking_number: None,
        },
        Order {
            id: "012".to_string(),
            status: OrderStatus::Shipped,
            payment_id: Some("payment-345".to_string()),
            tracking_number: Some("tracking-678".to_string()),
        },
    ];

    for order in &orders {
        println!("Order {} is {}", order.id, describe_status(&order.status));
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn describes_created() {
        assert_eq!(describe_status(&OrderStatus::Created), "created");
    }

    #[test]
    fn describes_paid() {
        assert_eq!(describe_status(&OrderStatus::Paid), "paid");
    }

    #[test]
    fn describes_shipped() {
        assert_eq!(describe_status(&OrderStatus::Shipped), "shipped");
    }

    #[test]
    fn describes_cancelled() {
        assert_eq!(describe_status(&OrderStatus::Cancelled), "cancelled");
    }
}
