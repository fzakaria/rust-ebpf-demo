

.PHONY: all

.DEFAULT_GOAL := all

all:
	cargo rustc --release -- --emit=llvm-bc
	cargo rustc --release -- --emit=llvm-ir
	llc target/release/deps/rust_ebpf_demo-*.bc -march=bpf -filetype=obj -o rust_ebpf_demo.o

