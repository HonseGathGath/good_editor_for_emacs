build:
    cargo build

run file:
    cargo run {{file}}

lint:
    cargo clippy

fix:
    cargo clippy --fix -- -W clippy::all -W clippy::pedantic

dev file:
    just build
    just run {{file}}
