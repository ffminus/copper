# list all available commands
help:
    @just --list

# lint and type check code with static analyzers
check:
    cargo fmt --check
    cargo clippy --tests --examples -- --deny warnings
    cargo check --tests --examples
    RUSTDOCFLAGS='--deny warnings' cargo doc

# run code quality and logic checks
ci: check
    cargo test
