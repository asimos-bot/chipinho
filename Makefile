.PHONY: build-wasm
build-wasm:
	wasm-pack build --target web --out-dir ../examples/wasm/pkg chipinho/
show-wasm: build-wasm
	wasm2wat examples/wasm/pkg/chipinho_bg.wasm | less
serve-wasm: build-wasm
	python3 -m http.server -d examples/wasm
