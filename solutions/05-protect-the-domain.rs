// Solution 5: Protect the domain
//
// Our methods express the allowed transitions, but callers can still skip
// them and construct a `PaidOrder` directly. How can we make callers go through
// the methods instead?
//
// A module (`mod`) gives us a privacy boundary. Items and struct fields are
// private by default; `pub` exposes selected parts. A public struct can keep
// its fields private, so code outside the module must use its public methods
// to create values or read their data. This is called encapsulation.
//
// Let's protect the `order` module and give callers one starting point:
// `CreatedOrder::new`. Here, `new` is an associated function, called on the type
// without an existing order, and `Self` means `CreatedOrder` inside its `impl`.
//
// Tasks:
// 1. Click Run above `main`. The bypass constructs a paid order without calling `pay`.
// 2. Remove `pub` from every struct field inside `order`. Keep the types and their
//    methods public so callers can use the API. Run again and inspect the
//    privacy error. Then delete the marked bypass block, including its `return`.
// 3. Implement `CreatedOrder::new` as the entry point for creating orders.
//    Reach other states through `pay`, `ship`, or `cancel`. Keep all fields private.
// 4. Click Run Tests above `mod tests`, then Run above `main`. Try changing an order's
//    ID directly from `main`. Inspect the error, then undo that experiment.
//
// Done: tests pass, `main` uses the API, and both direct field access and direct
// construction from outside `order` fail. Tests alone do not prove encapsulation.
// This API records a supplied payment ID; it does not perform or verify payment.

// Includes `PaymentId`; the optional `TrackingNumber` extension is not included.
// The bypass is commented out so `main` runs through the public API.

#![allow(dead_code)]

// We use concrete state types to focus on construction. The wrapper `Order` enum
// from exercise 4 can still be added when needed.
mod order {
    // Keep the structs and their methods public.
    #[derive(Debug)]
    pub struct CreatedOrder {
        id: String,
    }

    #[derive(Debug)]
    pub struct PaidOrder {
        id: String,
        payment_id: PaymentId,
    }

    #[derive(Debug)]
    pub struct ShippedOrder {
        id: String,
        payment_id: PaymentId,
        tracking_number: String,
    }

    #[derive(Debug)]
    pub struct CancelledOrder {
        id: String,
    }

    impl CreatedOrder {
        // Call `CreatedOrder::new(...)` without an existing order.
        // `Self` here means `CreatedOrder`.
        pub fn new(id: String) -> Self {
            CreatedOrder { id }
        }

        // Read-only access: callers can inspect the ID without changing it.
        pub fn id(&self) -> &str {
            &self.id
        }

        pub fn pay(self, payment_id: PaymentId) -> PaidOrder {
            PaidOrder {
                id: self.id,
                payment_id,
            }
        }

        pub fn cancel(self) -> CancelledOrder {
            CancelledOrder { id: self.id }
        }
    }

    impl PaidOrder {
        pub fn ship(self, tracking_number: String) -> ShippedOrder {
            ShippedOrder {
                id: self.id,
                payment_id: self.payment_id,
                tracking_number,
            }
        }
    }

    impl ShippedOrder {
        pub fn id(&self) -> &str {
            &self.id
        }

        pub fn payment_id(&self) -> &str {
            self.payment_id.as_str()
        }

        pub fn tracking_number(&self) -> &str {
            &self.tracking_number
        }
    }

    impl CancelledOrder {
        pub fn id(&self) -> &str {
            &self.id
        }
    }

    // Optional: use PaymentId as a guide for implementing TrackingNumber.
    // Workshop rule: "payment-" followed by one or more digits 0–9.
    // `Result` is either `Ok(the value)` or `Err(the reason construction failed)`.
    #[derive(Debug, PartialEq)]
    pub struct PaymentId(String);

    impl PaymentId {
        pub fn new(value: String) -> Result<Self, &'static str> {
            let digits = match value.strip_prefix("payment-") {
                Some(digits) => digits,
                None => return Err("Payment ID must start with payment-"),
            };

            // `all` checks every byte; the empty check rejects "payment-" alone.
            if digits.is_empty() || !digits.bytes().all(|byte| byte.is_ascii_digit()) {
                return Err("Payment ID must end with one or more digits 0-9");
            }

            Ok(Self(value))
        }

        pub fn as_str(&self) -> &str {
            &self.0
        }
    }
}

use order::{CreatedOrder, PaymentId};

fn main() {
    // Uncomment to see the privacy error, then restore the comments.
    // let bypass = order::PaidOrder {
    //     id: "bypass".to_string(),
    //     payment_id: PaymentId::new("payment-123".to_string()).unwrap(),
    // };

    let created = CreatedOrder::new("123".to_string());
    // Private fields also prevent changing `created.id` directly.
    println!("Created order {}", created.id());

    // This example value is valid; handle `Err` when accepting external input.
    let payment_id = PaymentId::new("payment-456".to_string()).unwrap();

    let shipped = created.pay(payment_id).ship("tracking-789".to_string());
    println!(
        "Shipped order {} with {}",
        shipped.id(),
        shipped.tracking_number()
    );

    let cancelled = CreatedOrder::new("999".to_string()).cancel();
    println!("Cancelled order {}", cancelled.id());
}

// Further practice: add a validated `TrackingNumber` type and use it in `ship` and
// `ShippedOrder`. Update callers and add validation tests, following `PaymentId`.
// Valid ID formats do not prove that a payment or shipment actually occurred.

// These tests sit outside `order`, so they use the same public API as `main`.
#[cfg(test)]
mod tests {
    use super::order::{CreatedOrder, PaymentId};

    #[test]
    fn creates_an_order_with_the_supplied_id() {
        for id in ["order-alpha", "order-beta"] {
            let created = CreatedOrder::new(id.to_string());
            assert_eq!(created.id(), id);
        }
    }

    #[test]
    fn ships_through_the_public_api_and_preserves_data() {
        let shipped = CreatedOrder::new("order-alpha".to_string())
            .pay(PaymentId::new("payment-456".to_string()).unwrap())
            .ship("tracking-789".to_string());

        assert_eq!(shipped.id(), "order-alpha");
        assert_eq!(shipped.payment_id(), "payment-456");
        assert_eq!(shipped.tracking_number(), "tracking-789");
    }

    #[test]
    fn cancels_through_the_public_api_and_preserves_identity() {
        let cancelled = CreatedOrder::new("order-delta".to_string()).cancel();
        assert_eq!(cancelled.id(), "order-delta");
    }
}

// Optional validation tests: click Run Tests above this module.
#[cfg(test)]
mod payment_id_tests {
    use super::order::PaymentId;

    #[test]
    fn payment_id_accepts_valid_format_and_preserves_text() {
        for value in ["payment-0", "payment-456", "payment-00123"] {
            let id = PaymentId::new(value.to_string()).expect("valid payment ID");
            assert_eq!(id.as_str(), value);
        }
    }

    #[test]
    fn payment_id_rejects_invalid_format() {
        for value in [
            "",
            "payment-",
            "tracking-456",
            "Payment-456",
            "payment-abc",
            "payment-12x",
            "payment--12",
            " payment-12",
            "payment-12 ",
            "payment-12\n",
            "payment-１２",
        ] {
            assert!(
                PaymentId::new(value.to_string()).is_err(),
                "Accepted invalid ID: {value:?}"
            );
        }
    }
}
