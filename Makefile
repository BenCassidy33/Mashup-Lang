all:
	cargo run ./examples/main.mash > ./examples/main.ll
test:
	cargo test -- --nocapture
