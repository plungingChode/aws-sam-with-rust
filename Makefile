CARGO_LAMBDA_FLAGS = 

.PHONY: build
build:
	cargo lambda build --release $(CARGO_LAMBDA_FLAGS)