default:
    just --list

run:
    cargo run -p {{ project_name }}_bin

install:
    cargo install --path crates/{{ project_name }}_bin

build:
    cargo build

check:
    cargo check

fmt:
    cargo fmt

clippy:
    cargo clippy -- -D warnings
