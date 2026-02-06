build:
    cargo build

run:
    cargo run

lint:
    cargo clippy

fix:
    cargo clippy --fix -- -W clippy::all -W clippy::pedantic

dev:
    just build
    just run
