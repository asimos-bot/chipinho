.PHONY: build-wasm
build-wasm:
	wasm-pack build --target web
show-wasm: build-wasm
	wasm2wat target/wasm32-unknown-unknown/release/chipinho.wasm | less
serve-wasm: build-wasm
	python3 -m http.server
