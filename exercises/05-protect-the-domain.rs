// Exercise 5: Protect the domain
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
// 1. Click Run above `main`. The 'bypass' constructs a paid order without calling `pay`, which should not be allowed.
// 2. Remove `pub` from every struct field inside the `order` module (you need to do this for each struct).
//    Keep the types and their methods public so callers can use the API.
//    Run again and inspect the privacy error. Then delete the marked 'bypass' block, including its `return`.
// 3. Implement `CreatedOrder::new` as the entry point for creating orders.
//    Reach other states through `pay`, `ship`, or `cancel`. Keep all fields private.
// 4. Click Run Tests above `mod tests`, then Run above `main`. Try changing an order's
//    ID directly from `main`. Inspect the error, then undo that experiment.
//
// Done: tests pass, `main` uses the API, and both direct field access and direct
// construction from outside the `order` module fail.

#![allow(dead_code)]

// We use concrete state types to focus on construction. The wrapper `Order` enum
// from exercise 4 can still be added when needed.
mod order {
    // TODO: Make all fields in these four order structs private.
    // Keep the structs and their methods public.
    #[derive(Debug)]
    pub struct CreatedOrder {
        pub id: String,
    }

    #[derive(Debug)]
    pub struct PaidOrder {
        pub id: String,
        pub payment_id: String,
    }

    #[derive(Debug)]
    pub struct ShippedOrder {
        pub id: String,
        pub payment_id: String,
        pub tracking_number: String,
    }

    #[derive(Debug)]
    pub struct CancelledOrder {
        pub id: String,
    }

    impl CreatedOrder {
        // Construct a created order with the supplied ID
        // (`Self` here means `CreatedOrder`).
        pub fn new(id: String) -> Self {
            todo!("Construct a created order with the supplied ID")
        }

        // Read-only access: callers can inspect the ID without changing it.
        pub fn id(&self) -> &str {
            &self.id
        }

        pub fn pay(self, payment_id: String) -> PaidOrder {
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
            &self.payment_id
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

use order::{CreatedOrder, PaidOrder};

fn main() {
    // Bypass block: remove this entire block after observing the privacy error.
    // The early `return` skips the unfinished constructor. An unreachable-code
    // warning below is expected until you remove this block.
    let bypass = PaidOrder {
        id: "bypass".to_string(),
        payment_id: "unverified".to_string(),
    };
    println!("Constructed directly: {bypass:?}");
    return;
    // End of bypass block.

    let created = CreatedOrder::new("123".to_string());
    // Task 4: try `created.id = "changed".to_string();` (make created mutable first).
    println!("Created order {}", created.id());
    let shipped = created
        .pay("payment-456".to_string())
        .ship("tracking-789".to_string());
    println!(
        "Shipped order {} with {}",
        shipped.id(),
        shipped.tracking_number()
    );

    let cancelled = CreatedOrder::new("999".to_string()).cancel();
    println!("Cancelled order {}", cancelled.id());
}

// Optional: validated identifiers
// Try this after the main task. Use `PaymentId` and its tests as a guide.
// Ask for help if newtypes or `Result` are unfamiliar.
// 1. Replace payment strings with `PaymentId` in fields and `pay`. Preserve the type
//    through `ship`; use `as_str()` to read its text. Update `main` and tests.
// 2. Add `TrackingNumber` for "tracking-" followed by one or more digits 0–9.
//    Use it in fields and `ship`. Add tests for valid values and invalid formats.
// 3. Try passing `PaymentId` to `ship`. Inspect the type error, then undo the change.

// These tests sit outside the `order` module, so they use the same public API as `main`.
#[cfg(test)]
mod tests {
    use super::order::CreatedOrder;

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
            .pay("payment-456".to_string())
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
