.PHONY: build-wasm
build-wasm:
	cargo build --target=wasm32-unknown-unknown --release
	# wasm-opt -O3 -o target/wasm32-unknown-unknown/release/chipinho.wasm target/wasm32-unknown-unknown/release/chipinho.wasm
show-wasm: build-wasm
	wasm2wat target/wasm32-unknown-unknown/release/chipinho.wasm | less
