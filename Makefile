doc:
	cargo rustdoc --open --all-features -- --cfg docsrs

test:
	cargo test --all-features

miri:
	RUST_BACKTRACE=1 MIRIFLAGS="-Zmiri-backtrace=full -Zmiri-symbolic-alignment-check" cargo +nightly miri test --all-features