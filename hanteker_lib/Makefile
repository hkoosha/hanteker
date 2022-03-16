.PHONY: build
build: clear
	cargo build

.PHONY: fmt
fmt:
	cargo fmt

.PHONY: clippy
clippy: clear
	cargo clippy

.PHONY: clear
clear:
	@for (( i=0; i<100; i++ )) ; do echo "" ; done
