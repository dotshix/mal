all: target/debug/step0_repl

target/debug/step0_repl:
	@echo "Building step0_repl..."
	cargo build --bin step0_repl

clean:
	cargo clean
	rm -f target/debug/step0_repl

test^rust-dotshix^step0: impls/rust-dotshix/target/debug/step0_repl
	@echo "Running tests for rust-dotshix step0"
	impls/rust-dotshix/run step0_repl ../tests/step0_repl.mal
