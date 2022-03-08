BIN=sudo ../target/debug/hanteker_cli

.PHONY: build
build:
	cargo build

.PHONY: fmt
fmt:
	cargo fmt

.PHONY: clippy
clippy:
	cargo clippy


.PHONY: run-raw
run-raw:
	cargo build
	$(BIN)


.PHONY: run-help
run-help:
	cargo build
	$(BIN) --help

.PHONY: run-version
run-version:
	cargo build
	$(BIN) -V


.PHONY: run-pretty-print
run-pretty-print:
	cargo build
	$(BIN) -p


.PHONY: run-start
run-start:
	cargo build
	$(BIN) --start

.PHONY: run-stop
run-stop:
	cargo build
	$(BIN) --stop

.PHONY: run-en-channel1
run-en-channel1:
	cargo build
	$(BIN) --enable-channel 1

.PHONY: run-en-channel2
run-en-channel2:
	cargo build
	$(BIN) --enable-channel 2

.PHONY: run-dis-channel1
run-dis-channel1:
	cargo build
	$(BIN) --disable-channel 1

.PHONY: run-dis-channel2
run-dis-channel2:
	cargo build
	$(BIN) --disable-channel 2
