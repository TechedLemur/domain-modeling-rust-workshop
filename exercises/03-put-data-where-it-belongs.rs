// Click Run above `main` for the demo; Run Tests above `mod tests` checks all tests.
// Use Run Test above a single test while working.

#![allow(dead_code)]

// Exercise 3: Put data where it belongs
//
// We now have one order state, but a `Paid` order can still have no payment ID.
// How can we make the state and its required information stay together?
//
// Enum variants can carry data of their own. `Paid { payment_id: String }`
// requires a payment ID whenever we construct that variant. Matching on it
// also lets us read that data. Here, `Order` keeps the shared ID while each
// variant carries the information its state needs.
//
// Business rules:
// - Created orders have no payment or tracking information.
// - Paid orders require a payment ID.
// - Shipped orders keep their payment ID and need a tracking number.
// - `Cancelled` means before payment: no payment ID or tracking number.
//
// `Paid` is already implemented. Use it as a guide for `Shipped`.
//
// Tasks:
// 1. Add the required fields to `Shipped`. Use `Paid` as a guide; do not use `Option`.
// 2. Implement `shipped_order` using its arguments.
// 3. Complete `describe` by matching the `Shipped` variant and reading its data.
//    Include the order ID, payment ID, tracking number, and the word "shipped".
//    Follow the `Paid` example; choose your own wording.
//
// Done: tests pass and Shipped has required payment and tracking fields.
//
// Optional: add tracking_number(&Order) -> Option<&str> using match. Return a
// tracking number only for shipped orders. Borrow the string with `.as_str()`.
// In the next exercise we restrict transitions between these states.

#[derive(Debug)]
struct Order {
    id: String,
    status: OrderStatus,
}

#[derive(Debug, PartialEq)]
enum OrderStatus {
    Created,
    Paid {
        payment_id: String,
    },
    Shipped {
        // TODO: Model the information required for a shipped order.
    },
    Cancelled,
}

// Use this function as a guide for `shipped_order`.
fn paid_order(id: String, payment_id: String) -> Order {
    Order {
        id,
        status: OrderStatus::Paid { payment_id },
    }
}

fn shipped_order(id: String, payment_id: String, tracking_number: String) -> Order {
    todo!("Construct an Order whose Shipped variant holds the required data")
}

fn describe(order: &Order) -> String {
    match &order.status {
        OrderStatus::Created => format!("Order {} has been created", order.id),
        // This pattern gives us the `payment_id` field.
        OrderStatus::Paid { payment_id } => {
            format!("Order {} was paid with {}", order.id, payment_id)
        }
        // TODO: Replace .. with the fields you need to read, as in Paid above.
        OrderStatus::Shipped { .. } => {
            todo!("Include the order ID, payment ID, and tracking number")
        }
        OrderStatus::Cancelled => format!("Order {} was cancelled before payment", order.id),
    }
}

fn main() {
    let paid = paid_order("123".to_string(), "payment-456".to_string());
    println!("{}", describe(&paid));

    let shipped = shipped_order(
        "789".to_string(),
        "payment-012".to_string(),
        "tracking-345".to_string(),
    );
    println!("{}", describe(&shipped));
}

// You do not need to change the tests below.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shipped_order_keeps_its_identity_and_state() {
        let order = shipped_order(
            "order-123".to_string(),
            "payment-456".to_string(),
            "tracking-789".to_string(),
        );
        assert_eq!(order.id, "order-123");
        assert!(matches!(order.status, OrderStatus::Shipped { .. }));
    }

    #[test]
    fn describes_shipped_with_payment_and_tracking_information() {
        // Different inputs check that the description uses the supplied data.
        for (id, payment, tracking) in [
            ("123", "payment-456", "tracking-789"),
            ("987", "payment-654", "tracking-321"),
        ] {
            let order = shipped_order(id.to_string(), payment.to_string(), tracking.to_string());
            let description = describe(&order);
            assert!(description.contains(id), "Missing order ID: {description}");
            assert!(
                description.contains(payment),
                "Missing payment ID: {description}"
            );
            assert!(
                description.contains(tracking),
                "Missing tracking number: {description}"
            );
            assert!(
                description.to_lowercase().contains("shipped"),
                "Missing shipped state: {description}"
            );
        }
    }

    #[test]
    fn describes_paid_with_its_payment_id() {
        let order = paid_order("123".to_string(), "payment-456".to_string());
        let description = describe(&order);
        assert!(
            description.contains("123"),
            "Missing order ID: {description}"
        );
        assert!(
            description.contains("payment-456"),
            "Missing payment ID: {description}"
        );
        assert!(
            description.to_lowercase().contains("paid"),
            "Missing paid state: {description}"
        );
    }
}
