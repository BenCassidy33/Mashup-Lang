run-dev:
	cargo run --bin run ./examples/main.mash > ./examples/main.ll

build:
	cargo build

test:
	cargo test -- --nocapture
