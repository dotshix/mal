all: target/debug/step1_read_print

target/debug/step1_read_print:
	@echo "Building step1_read_print..."
	cargo build --bin step1_read_print

clean:
	cargo clean
	rm -f target/debug/step1_read_print

test^rust-dotshix^step0: impls/rust-dotshix/target/debug/step0_repl
	@echo "Running tests for rust-dotshix step0"
	impls/rust-dotshix/run step0_repl ../tests/step0_repl.mal
