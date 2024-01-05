# list all available commands
help:
    @just --list

# lint and type check code with static analyzers
check:
    cargo fmt --check
    cargo clippy --tests -- --deny warnings
    cargo check --tests
    RUSTDOCFLAGS='--deny warnings' cargo doc
