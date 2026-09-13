// Click Run above main for the demo; Run Tests above mod tests checks all tests.
// Use Run Test above a single test while working.

#![allow(dead_code)]

// Exercise 2: Model order state
//
// OrderStatus replaces the two flags with one state: created, paid, or shipped.
// This removes the "shipped but unpaid" flag combination. The order keeps its ID.
//
// Tasks:
// 1. Complete describe_status. Use the Created match arm as a guide.
//    Return "paid" and "shipped" for the other states. Name every variant;
//    do not use a wildcard (_) arm. Run the tests.
// 2. Add Cancelled to OrderStatus (cancellation before payment). Check the
//    compiler error in describe_status, then handle it by returning "cancelled".
//    Add a test for that case, following the existing tests.
// 3. Create an order with status Paid and no payment ID in main.
//    Why should a paid order require a payment ID, and why does this model allow None?
//
// Done: every state has an explicit match arm, all tests pass (including your
// cancellation test), and you can explain the remaining invalid combination.

#[derive(Debug, PartialEq)]
enum OrderStatus {
    Created,
    Paid,
    Shipped,
    // TODO (task 2): Add Cancelled after the existing tests pass.
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
        OrderStatus::Paid => todo!("Describe the paid state"),
        OrderStatus::Shipped => todo!("Describe the shipped state"),
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

    // Task 3: Create an order with status Paid and no payment ID.
    // Order derives Debug, so print it with {:?}, for example:
    // println!("{:?}", order);
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

    // TODO (task 2): Add a test for Cancelled after adding the variant.
}
