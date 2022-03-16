BIN=sudo ../target/debug/hanteker_cli

.PHONY: build
build: clear
	cargo build

.PHONY: fmt
fmt:
	cargo fmt

.PHONY: clippy
clippy: clear
	cargo clippy


.PHONY: run-raw
run-raw: clear
	cargo build
	$(BIN)


.PHONY: run-help
run-help: clear
	cargo build
	$(BIN) --help

.PHONY: run-version
run-version: clear
	cargo build
	$(BIN) -V


.PHONY: run-pretty-print
run-pretty-print: clear
	cargo build
	$(BIN) -p


.PHONY: run-start
run-start: clear
	cargo build
	$(BIN) --start

.PHONY: run-stop
run-stop: clear
	cargo build
	$(BIN) --stop

.PHONY: run-en-channel1
run-en-channel1: clear
	cargo build
	$(BIN) --enable-channel 1

.PHONY: run-en-channel2
run-en-channel2: clear
	cargo build
	$(BIN) --enable-channel 2

.PHONY: run-dis-channel1
run-dis-channel1: clear
	cargo build
	$(BIN) --disable-channel 1

.PHONY: run-dis-channel2
run-dis-channel2: clear
	cargo build
	$(BIN) --disable-channel 2

.PHONY: clear
clear:
	@for (( i=0; i<100; i++ )) ; do echo "" ; done
