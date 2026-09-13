// Click Run above main for the demo; Run Tests above mod tests checks all tests.
// Use Run Test above a single test while working.

#![allow(dead_code)]

// Solution 3: Put data where it belongs
//
// Business rules:
// - Created orders have no payment or tracking information.
// - Paid orders require a payment ID.
// - Shipped orders keep their payment ID and need a tracking number.
// - Cancelled means before payment: no payment ID or tracking number.
//
// Paid is already implemented. Use it as a guide for Shipped.
// Order keeps its ID; each variant holds the data its state requires.
//
// Tasks:
// 1. Add the required fields to Shipped. Use Paid as a guide; do not use Option.
// 2. Implement shipped_order using its arguments.
// 3. Complete describe by matching the Shipped variant and reading its data.
//    Include the order ID, payment ID, tracking number, and the word "shipped".
//    Follow the Paid example; choose your own wording.
//
// Done: tests pass and Shipped has required payment and tracking fields.
//
// Optional: add tracking_number(&Order) -> Option<&str> using match. Return a
// tracking number only for shipped orders. Borrow the string with .as_str().
// In the next exercise we restrict transitions between these states.

// Includes the optional tracking_number helper and its tests.

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
        payment_id: String,
        tracking_number: String,
    },
    Cancelled,
}

// Construct Paid with its required payment ID.
fn paid_order(id: String, payment_id: String) -> Order {
    Order {
        id,
        status: OrderStatus::Paid { payment_id },
    }
}

fn shipped_order(id: String, payment_id: String, tracking_number: String) -> Order {
    Order {
        id,
        status: OrderStatus::Shipped {
            payment_id,
            tracking_number,
        },
    }
}

fn describe(order: &Order) -> String {
    match &order.status {
        OrderStatus::Created => format!("Order {} has been created", order.id),
        // This pattern gives us the payment_id field.
        OrderStatus::Paid { payment_id } => {
            format!("Order {} was paid with {}", order.id, payment_id)
        }
        OrderStatus::Shipped {
            payment_id,
            tracking_number,
        } => {
            format!(
                "Order {} was shipped with tracking_number: {}, and payment_id: {}",
                order.id, tracking_number, payment_id
            )
        }
        OrderStatus::Cancelled => format!("Order {} was cancelled before payment", order.id),
    }
}

// Match on the status and get the tracking number
fn tracking_number(order: &Order) -> Option<&str> {
    match &order.status {
        OrderStatus::Created | OrderStatus::Paid { payment_id: _ } | OrderStatus::Cancelled => None,
        OrderStatus::Shipped {
            payment_id: _,
            tracking_number,
        } => Some(tracking_number),
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
    println!("Paid order tracking: {:?}", tracking_number(&paid));
    println!("Shipped order tracking: {:?}", tracking_number(&shipped));
}

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

// Tests for the optional helper.
#[cfg(test)]
mod tracking_number_tests {
    use super::*;

    #[test]
    fn returns_tracking_only_for_shipped_orders() {
        let shipped = shipped_order("a".into(), "payment-1".into(), "tracking-2".into());
        assert_eq!(tracking_number(&shipped), Some("tracking-2"));
        for status in [
            OrderStatus::Created,
            OrderStatus::Paid {
                payment_id: "payment-1".into(),
            },
            OrderStatus::Cancelled,
        ] {
            assert_eq!(
                tracking_number(&Order {
                    id: "b".into(),
                    status
                }),
                None
            );
        }
    }
}
