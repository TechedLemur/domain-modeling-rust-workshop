// Bonus 2: Validate JSON at the domain boundary
//
// So far, we have created values directly in Rust. In a real application, data
// also arrives from files or APIs. Our types cannot stop someone sending bad
// input, so how do we check it before it becomes part of our domain model?
//
// Imagine receiving a payment confirmation as JSON. Its payment ID might be
// missing, be a number instead of text, or contain text in the wrong format.
// This example shows where we catch each problem, using the validated `PaymentId`
// idea from exercise 5.
//
// We use Serde, a library for reading and writing Rust values in data formats.
// `serde_json` provides JSON support. Deriving `Deserialize` generates the code
// to read `PaymentInput`. There are two steps:
// 1. Parse JSON into `PaymentInput`: are the required fields strings?
// 2. Convert it into `Payment`: does the payment ID have the required format?
// Notice that parsing can succeed while domain validation still fails.
//
// Click Run above `main` or Run Tests above `mod tests`.
// Optional tasks:
// - Remove `payment_id`, change it to a number, or give it a bad format.
//   Compare the errors, then restore the examples.
// - Reject blank order IDs in `into_domain`, including whitespace.
//   Add tests for the new rule.
// Serde reference: https://serde.rs/derive.html

use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)] // Reject unknown keys, such as misspelled field names.
struct PaymentInput {
    order_id: String,
    payment_id: String,
}

#[derive(Debug)]
struct Payment {
    order_id: String,
    payment_id: PaymentId,
}

mod payment_id {
    #[derive(Debug, PartialEq)]
    pub struct PaymentId(String);

    impl PaymentId {
        pub fn new(value: String) -> Result<Self, String> {
            match value.strip_prefix("payment-") {
                Some(digits)
                    if !digits.is_empty() && digits.bytes().all(|b| b.is_ascii_digit()) =>
                {
                    Ok(Self(value))
                }
                _ => Err("payment ID must be payment- followed by digits 0-9".to_string()),
            }
        }

        pub fn as_str(&self) -> &str {
            &self.0
        }
    }
}

use payment_id::PaymentId;

impl PaymentInput {
    fn into_domain(self) -> Result<Payment, String> {
        // `?` returns early on `Err`; otherwise it gives us the validated value.
        let payment_id = PaymentId::new(self.payment_id)?;
        Ok(Payment {
            order_id: self.order_id,
            payment_id,
        })
    }
}

#[derive(Debug)]
enum ImportError {
    Json(serde_json::Error),
    Domain(String),
}

fn import_payment(json: &str) -> Result<Payment, ImportError> {
    // `map_err` wraps the error so callers can distinguish the two steps.
    let input: PaymentInput = serde_json::from_str(json).map_err(ImportError::Json)?;
    input.into_domain().map_err(ImportError::Domain)
}

fn main() {
    // `r#"..."#` lets us write JSON without escaping its quotation marks.
    for json in [
        r#"{"order_id":"order-123","payment_id":"payment-456"}"#,
        r#"{"order_id":"order-123"}"#,
        r#"{"order_id":"order-123","payment_id":456}"#,
        r#"{"order_id":"order-123","payment_id":"wrong"}"#,
    ] {
        match import_payment(json) {
            Ok(payment) => println!(
                "Imported {} for {}",
                payment.payment_id.as_str(),
                payment.order_id
            ),
            Err(ImportError::Json(error)) => println!("JSON error: {error}"),
            Err(ImportError::Domain(error)) => println!("Domain error: {error}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn imports_valid_confirmation_and_preserves_values() {
        let payment =
            import_payment(r#"{"order_id":"order-123","payment_id":"payment-00456"}"#).unwrap();
        assert_eq!(payment.order_id, "order-123");
        assert_eq!(payment.payment_id.as_str(), "payment-00456");
    }

    #[test]
    fn rejects_bad_format_after_successful_parsing() {
        for value in [
            "",
            "wrong",
            "payment-",
            "payment-12x",
            "payment--1",
            "payment-12 ",
            "payment-１２",
        ] {
            let json =
                serde_json::json!({"order_id": "order-123", "payment_id": value}).to_string();
            assert!(serde_json::from_str::<PaymentInput>(&json).is_ok());
            assert!(
                matches!(import_payment(&json), Err(ImportError::Domain(_))),
                "{json}"
            );
        }
    }

    #[test]
    fn rejects_invalid_json_structure() {
        for json in [
            "not json",
            r#"{"order_id":"order-123"}"#,
            r#"{"payment_id":"payment-456"}"#,
            r#"{"order_id":"order-123","payment_id":456}"#,
            r#"{"order_id":"order-123","payment_id":null}"#,
            r#"{"order_id":123,"payment_id":"payment-456"}"#,
            r#"{"order_id":"order-123","payment_id":"payment-456","typo":true}"#,
        ] {
            assert!(
                matches!(import_payment(json), Err(ImportError::Json(_))),
                "{json}"
            );
        }
    }
}
