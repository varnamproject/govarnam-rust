test:
	cargo t -- --nocapture

test-inside-container:
	docker build -t govarnam-rust-test .
	docker run --rm -v $(PWD):/workspace govarnam-rust-test cargo test -- --nocapture


.PHONY: test test-inside-container

