// Solution 4: Model valid transitions
//
// We've modeled the data each state needs. Now let's model how an order
// moves between states: created orders can be paid or cancelled, and paid
// orders can be shipped. In this workflow, shipping and cancellation are final.
//
// Each state gets its own struct. An `impl` block adds methods to a type, so we
// can offer `pay` on `CreatedOrder` and `ship` only on `PaidOrder`. A method taking
// `self` consumes the old value and returns the next state; callers can no
// longer use the old order after the transition.
//
// Let's build those transitions. The `Order` enum still groups the state types
// so `describe` can accept any of them. Direct construction is still possible;
// we'll close that gap in exercise 5.
//
// Tasks:
// 1. Implement `pay`, `cancel`, and `ship`. Preserve the order ID in every transition
//    and the payment ID when shipping. Only created orders can cancel directly.
// 2. Replace `_` in `describe` with `Shipped` and `Cancelled` `match` arms.
//    Include the order ID and state name; shipped orders also need a tracking number.
//    Choose your own wording, using "shipped" or "cancelled" for the state.
// 3. Click Run Tests above `mod tests`, then Run above `main` to see both paths.
// 4. Try shipping a `CreatedOrder` or reusing it after `pay`. Read the compiler
//    errors, then undo those experiments.

#![allow(dead_code)]

// The optional refund extension is not included.

#[derive(Debug, PartialEq)]
struct CreatedOrder {
    id: String,
}

#[derive(Debug, PartialEq)]
struct PaidOrder {
    id: String,
    payment_id: String,
}

#[derive(Debug, PartialEq)]
struct ShippedOrder {
    id: String,
    payment_id: String,
    tracking_number: String,
}

#[derive(Debug, PartialEq)]
struct CancelledOrder {
    id: String,
}

// This enum lets us handle orders in different states through one type.
// For example, a `Vec<Order>` can contain both paid and shipped orders.
#[derive(Debug, PartialEq)]
enum Order {
    Created(CreatedOrder),
    Paid(PaidOrder),
    Shipped(ShippedOrder),
    Cancelled(CancelledOrder),
}

// An `impl` block defines methods for a type.
// Taking `self` consumes the old order; the returned value represents its new state.
impl CreatedOrder {
    fn pay(self, payment_id: String) -> PaidOrder {
        PaidOrder {
            id: self.id,
            payment_id,
        }
    }

    fn cancel(self) -> CancelledOrder {
        CancelledOrder { id: self.id }
    }
}

impl PaidOrder {
    fn ship(self, tracking_number: String) -> ShippedOrder {
        ShippedOrder {
            id: self.id,
            payment_id: self.payment_id,
            tracking_number,
        }
    }
}

fn describe(order: &Order) -> String {
    match order {
        Order::Created(order) => {
            format!("Order {} has been created", order.id)
        }

        Order::Paid(order) => {
            format!("Order {} has been paid", order.id)
        }

        Order::Shipped(shipped_order) => format!(
            "Order {} has been shipped, tracking number: {}",
            shipped_order.id, shipped_order.tracking_number
        ),
        Order::Cancelled(cancelled_order) => {
            format!("Order {} has been cancelled", cancelled_order.id)
        }
    }
}

// Optional: add a refund transition from PaidOrder. Keep the order and payment
// IDs and the supplied refund ID. Choose a type, update `Order` and `describe`,
// and test that all IDs are preserved. Assume the refund has already succeeded.
// Shipped orders cannot be cancelled in this workflow.

fn main() {
    // When we know the state, we keep the concrete type.
    let created = CreatedOrder {
        id: "123".to_string(),
    };

    let paid = created.pay("payment-456".to_string());

    // Because `paid` is a `PaidOrder`, Rust knows that ``ship`()` is available.
    let shipped = paid.ship("tracking-789".to_string());

    // Wrap the state so `describe` can handle it.
    // Optional: try Order::Paid(shipped). Explain the error, then undo the change.
    let order = Order::Shipped(shipped);

    println!("{}", describe(&order));

    // Cancel a separate order.
    let cancelled = CreatedOrder {
        id: "999".to_string(),
    }
    .cancel();
    println!("{}", describe(&Order::Cancelled(cancelled)));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn created_order_can_be_paid() {
        let created = CreatedOrder {
            id: "123".to_string(),
        };

        let paid = created.pay("payment-456".to_string());

        assert_eq!(
            paid,
            PaidOrder {
                id: "123".to_string(),
                payment_id: "payment-456".to_string(),
            }
        );
    }

    #[test]
    fn paid_order_can_be_shipped() {
        let paid = PaidOrder {
            id: "123".to_string(),
            payment_id: "payment-456".to_string(),
        };

        let shipped = paid.ship("tracking-789".to_string());

        assert_eq!(
            shipped,
            ShippedOrder {
                id: "123".to_string(),
                payment_id: "payment-456".to_string(),
                tracking_number: "tracking-789".to_string(),
            }
        );
    }

    #[test]
    fn created_order_can_be_cancelled() {
        let created = CreatedOrder {
            id: "123".to_string(),
        };

        let cancelled = created.cancel();

        assert_eq!(
            cancelled,
            CancelledOrder {
                id: "123".to_string(),
            }
        );
    }

    #[test]
    fn shipped_description_contains_order_id_tracking_and_state() {
        for (id, tracking) in [
            ("order-alpha", "tracking-987"),
            ("order-beta", "tracking-654"),
        ] {
            // Construct directly so this test does not depend on `pay` or `ship`.
            let order = Order::Shipped(ShippedOrder {
                id: id.to_string(),
                payment_id: "payment-321".to_string(),
                tracking_number: tracking.to_string(),
            });
            let description = describe(&order);

            assert!(description.contains(id), "Missing order ID: {description}");
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
    fn cancelled_description_contains_order_id_and_state() {
        for id in ["order-gamma", "order-delta"] {
            let order = Order::Cancelled(CancelledOrder { id: id.to_string() });
            let description = describe(&order);

            assert!(description.contains(id), "Missing order ID: {description}");
            assert!(
                description.to_lowercase().contains("cancelled"),
                "Missing cancelled state: {description}"
            );
        }
    }
}
