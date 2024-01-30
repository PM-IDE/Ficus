FROM ficus_tests_base:latest

ENTRYPOINT $cargo test --manifest-path /pmide/ficus/src/rust/ficus_backend/Cargo.toml --release