# local build & test rust only
```shell
cargo nextest run --all-features &&
cargo nextest run --examples --all-features &&
cargo test --doc --all-features &&
cargo doc --all-features &&
cargo clippy --all-features -- --deny warnings
```