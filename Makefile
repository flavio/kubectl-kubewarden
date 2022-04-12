build:
	cargo build --target wasm32-wasi --release
	cp target/wasm32-wasi/release/kubectl-kubewarden.wasm kubewarden.wasm
