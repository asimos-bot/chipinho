.PHONY: build-wasm
build-wasm:
	wasm-pack build --target web --out-dir examples/wasm/pkg chipinho/ --features wasm-bindgen
show-wasm: build-wasm
	wasm2wat chipinho/target/wasm32-unknown-unknown/release/chipinho.wasm | less
serve-wasm: build-wasm
	python3 -m http.server -d examples/wasm
