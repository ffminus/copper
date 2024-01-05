# list all available commands
help:
    @just --list

# lint and type check code with static analyzers
check:
    cargo fmt --check
    cargo clippy -- --deny warnings
    cargo check
    RUSTDOCFLAGS='--deny warnings' cargo doc
