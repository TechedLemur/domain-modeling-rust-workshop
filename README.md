# Domain Modeling in Rust: Making Illegal States Unrepresentable

Can an order be shipped before it is paid? Can a paid order have no payment ID?
With a few booleans and optional fields, both are easy to represent, but they break the business rules.

In this hands-on workshop, we'll improve an order model step by step. We'll use
enums to express its states, put required data in the right variants, and define
methods for the allowed transitions. Along the way, we'll use pattern matching
to work with the model and modules to protect it from invalid construction.
The aim is to express business rules in code so the compiler helps enforce them.

You should be comfortable programming in another language, but no Rust experience
is required. We start with a short introduction to the syntax used in the exercises.
GitHub Codespaces provides Rust and the editor tools, so you can work in your browser.

We'll alternate short introductions, hands-on exercises, and solution discussions. Optional challenges and examples on generic typestate and JSON
validation are available if you finish early or want to explore further afterward.

## Start in GitHub Codespaces

On the repository's GitHub page, select **Code → Codespaces → Create codespace**.
Wait for the development container setup to finish, then open a terminal.
Rust and the editor extensions are configured automatically; no local installation
is needed. Setup builds all exercise and solution binaries.

Open [exercise 0](exercises/00-get-started.rs), wait for Rust Analyzer to
finish loading, and click **Run** above `fn main()`.

## Exercises

Each exercise is a single Rust file with its own starting point. Read its comments
and make your changes in that file. You can start the next exercise even if you
have not finished the previous one.

| Exercise                     | File                                                                         | In the editor                                                           |
| ---------------------------- | ---------------------------------------------------------------------------- | ----------------------------------------------------------------------- |
| 0. Get started with Rust     | [00-get-started.rs](exercises/00-get-started.rs)                             | Click **Run** above `main` and change the example values                |
| 1. Spot the problem          | [01-spot-the-problem.rs](exercises/01-spot-the-problem.rs)                   | Click **Run** above `main` and explain the invalid orders               |
| 2. Model order state         | [02-model-order-state.rs](exercises/02-model-order-state.rs)                 | Click **Run Tests** above `mod tests`                                   |
| 3. Put data where it belongs | [03-put-data-where-it-belongs.rs](exercises/03-put-data-where-it-belongs.rs) | Click **Run Tests** above `mod tests`                                   |
| 4. Model valid transitions   | [04-model-valid-transitions.rs](exercises/04-model-valid-transitions.rs)     | Click **Run Tests** above `mod tests`                                   |
| 5. Protect the domain        | [05-protect-the-domain.rs](exercises/05-protect-the-domain.rs)               | Click **Run Tests** above `mod tests`, then try the privacy experiments |

For exercises 2–5, use **Run Test** above an individual test while working, then
run the whole test module. Click **Run** above `main` to see your implementation
used in a demonstration. Running a demonstration successfully does not replace
checking all tests and completing the instructions in the comments.

Exercises 0–1 are exploratory and have no automated tests. Exercises 2–5 contain
unfinished `todo!()` expressions: demonstrations and some tests will panic until
you implement those parts. This is expected at the start.

### Terminal fallback

If the editor actions are unavailable, run these commands from the repository root:

```sh
cargo run --bin ex00
cargo run --bin ex01
cargo test --bin ex02
cargo test --bin ex03
cargo test --bin ex04
cargo test --bin ex05
```

Run only the command for your current exercise. Use `cargo run --bin ex03` for its
demonstration or `cargo check --bin ex03` to check compilation without running it;
replace the target name as needed.

## Further exploration

These complete examples are optional. Each includes its own model, tests, and a
small challenge. Click **Run** above `main` or **Run Tests** above `mod tests`.

| Topic                      | File                                                     | Terminal command                |
| -------------------------- | -------------------------------------------------------- | ------------------------------- |
| Generic typestate          | [01-generic-typestate.rs](bonus/01-generic-typestate.rs) | `cargo run --bin typestate`     |
| JSON and domain validation | [02-json-boundary.rs](bonus/02-json-boundary.rs)         | `cargo run --bin json-boundary` |

Use `cargo test --bin typestate --bin json-boundary` to check both examples.
Cargo downloads Serde automatically during Codespaces setup.

## Repository layout

```text
Cargo.toml       One package with explicitly named binary targets
exercises/       One .rs file per exercise (ex00–ex05)
bonus/           Runnable examples and optional challenges
solutions/       Solution files (solution01–solution05)
.devcontainer/   GitHub Codespaces configuration
```

Solutions 1–5 are available as `solution01` through `solution05`.
Run one with `cargo run --bin solution03`, or use the editor actions.
Solution 3 includes the optional tracking helper. Solution 5 includes PaymentId;
the TrackingNumber extension and exercise 4's refund extension are left for further practice.

For workshop maintenance, `cargo build --bins` builds all targets and
`cargo test --bins --no-run` checks that all test harnesses compile without running
the intentionally unfinished exercises.
