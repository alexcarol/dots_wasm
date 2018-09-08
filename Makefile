build:
	cargo +nightly build --target wasm32-unknown-unknown --release
	wasm-gc target/wasm32-unknown-unknown/release/rocket.wasm docs/program.wasm

run-server: build
	http-server docs .

