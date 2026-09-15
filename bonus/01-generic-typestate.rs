// Bonus 1: Generic typestate (advanced, optional)
//
// In exercise 4, separate types gave each order state its own allowed methods.
// `CreatedOrder` could be paid, and `PaidOrder` could be shipped. That worked well,
// but each type declared the same order ID field.
//
// Could we share the common parts while still letting the compiler restrict
// which methods are available? Here we explore another way to express the same
// rules: one generic Order<State> type, with data and methods for each state.
// `Order<Created>` replaces `CreatedOrder`; `Order<Paid>` replaces `PaidOrder`.
//
// As you read, compare the two approaches. Notice which methods apply to every
// state and which apply to just one. The business rules stay the same.
//
// Click Run above `main` or Run Tests above `mod tests`.
// Try calling `ship` before `pay`, or using `created` after `pay`. Read the errors,
// then undo the changes. The same restrictions from exercise 4 still apply.
// Optional: add Cancelled and cancel(self) only on Order<Created>. Test that the
// order ID survives. Try cancelling a shipped order, then undo the experiment.

mod order {
    // Generics let one definition work with different types.
    // `State` is a type parameter, like `T` in `Option<T>`; the name is our choice.
    // `Order<Created>` and `Order<Paid>` are different types built from this definition.
    // You can read more about generics in the Rust Book: https://doc.rust-lang.org/book/ch10-01-syntax.html
    #[derive(Debug)]
    pub struct Order<State> {
        id: String,
        state: State,
    }

    #[derive(Debug)]
    pub struct Created;

    #[derive(Debug)]
    pub struct Paid {
        payment_id: String,
    }

    #[derive(Debug)]
    pub struct Shipped {
        payment_id: String,
        tracking_number: String,
    }

    // `impl<State>` introduces a type parameter: these methods work for any `State`.
    impl<State> Order<State> {
        pub fn id(&self) -> &str {
            &self.id
        }
    }

    // Here `Created` is a specific type, so only `Order<Created>` gets these methods.
    impl Order<Created> {
        pub fn new(id: String) -> Self {
            Self { id, state: Created }
        }

        // Return a different type: consume `Order<Created>` and build `Order<Paid>`.
        pub fn pay(self, payment_id: String) -> Order<Paid> {
            Order {
                id: self.id,
                state: Paid { payment_id },
            }
        }
    }

    // Only `Order<Paid>` has `ship`; other states cannot call it.
    impl Order<Paid> {
        pub fn ship(self, tracking_number: String) -> Order<Shipped> {
            Order {
                id: self.id,
                state: Shipped {
                    payment_id: self.state.payment_id,
                    tracking_number,
                },
            }
        }
    }

    impl Order<Shipped> {
        pub fn payment_id(&self) -> &str {
            &self.state.payment_id
        }

        pub fn tracking_number(&self) -> &str {
            &self.state.tracking_number
        }
    }
}

use order::{Created, Order};

fn main() {
    // `::<Created>` selects the state type when calling `new`.
    // Try `order::Shipped` instead of `Created`; read the error, then undo the change.
    let created = Order::<Created>::new("order-123".to_string());
    println!("Created {}", created.id());
    let paid = created.pay("payment-456".to_string());
    println!("Paid {}", paid.id());
    let shipped = paid.ship("tracking-789".to_string());
    println!(
        "Shipped {}: payment {}, tracking {}",
        shipped.id(),
        shipped.payment_id(),
        shipped.tracking_number()
    );
}

#[cfg(test)]
mod tests {
    use super::{Created, Order};

    #[test]
    fn transitions_preserve_identity_and_state_data() {
        let created = Order::<Created>::new("order-alpha".to_string());
        assert_eq!(created.id(), "order-alpha");
        let paid = created.pay("payment-beta".to_string());
        assert_eq!(paid.id(), "order-alpha");
        let shipped = paid.ship("tracking-gamma".to_string());
        assert_eq!(shipped.id(), "order-alpha");
        assert_eq!(shipped.payment_id(), "payment-beta");
        assert_eq!(shipped.tracking_number(), "tracking-gamma");
    }
}
