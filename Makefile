.PHONY: build-wasm
build-wasm:
	cargo build --target wasm32-unknown-unknown --release
show-wasm: build-wasm
	wasm2wat target/wasm32-unknown-unknown/release/chipinho.wasm | less
serve-wasm: build-wasm
	python3 -m http.server
