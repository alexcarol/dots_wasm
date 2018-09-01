run-server:
	cargo +nightly build --target wasm32-unknown-unknown --release
	wasm-gc target/wasm32-unknown-unknown/release/rocket.wasm html/program.wasm
	http-server html .

