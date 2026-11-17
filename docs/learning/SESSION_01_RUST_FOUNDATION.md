# Session 1 Rust Foundation

## Learning objective

Implement Paritas's first protocol-independent domain type and use the Rust
compiler and tests to verify its behavior.

## Workspace model

A Cargo workspace manages several related packages, called crates, with one
lockfile and shared settings. This workspace currently contains:

- `paritas-core`, a library crate containing authorization concepts that do not
  depend on REST, GraphQL, or MCP.
- `paritas-cli`, a binary crate that will become the `paritas` executable. It
  will depend on `paritas-core` as soon as it consumes the first core type.

The dependency points from the CLI to the core. Keeping the core unaware of the
CLI and protocol adapters makes its policy rules reusable and easy to test.

## Exercise

Open `crates/paritas-core/src/decision.rs` and complete `Decision` with exactly
these variants:

- `Allowed`
- `Denied`
- `Unknown`
- `Error`

The type must implement these standard traits:

- `Debug`, so test failures and diagnostics can print it.
- `Clone` and `Copy`, because this small value has no owned data.
- `PartialEq` and `Eq`, so policy rules and tests can compare decisions.

Use Rust's `derive` attribute rather than writing those trait implementations
manually.

Do not add serialization yet. We will introduce Serde separately so its purpose
and wire-format consequences remain visible.

## Test-driven workflow

Run the tests before editing the enum:

```bash
cargo test --workspace
```

The build should fail because the tests already express the missing behavior.
Read the first compiler error, make the smallest useful change, and rerun the
tests. Once they pass, run all quality gates:

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

## Reflection questions

1. Why is `Decision` an enum instead of a string or Boolean?
2. Why must `Unknown` remain different from `Denied`?
3. What does `Copy` change when a value is passed to a function?

## Session outcome

The exercise is complete when the focused decision tests and all workspace
quality gates pass. The same format, lint, and test commands run in GitHub
Actions on every push and pull request.
