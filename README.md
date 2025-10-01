# rust-lib
A basic template to create new rust libraries.

# Migrations

## Generate

```shell
cargo run --bin generate_migration --features dev-bin <master|data> "<name>"
```

# Tests

## Test coverage

```shell
cargo tarpaulin --out Html
```
