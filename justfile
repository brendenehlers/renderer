default:
    @just list

list:
    @just --list

build:
    cargo build --manifest-path ./open-gl-rs/Cargo.toml

run:
    cargo run --manifest-path ./open-gl-rs/Cargo.toml