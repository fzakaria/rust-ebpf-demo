# Rust eBPF Hello World

This is a **minimal** example of how to write an eBPF filter in rust **natively**.
This "hello-world" draws on a lot of inspiration from (this blog post)[http://unhandledexpression.com/general/rust/2018/02/02/poc-compiling-to-ebpf-from-rust.html] however I found the need to use gobpf
unecessary.

> The blog uses gobpf simply from what I can surmise to leverage gobpf ability to load ebpf programs
> for kprobes. gobpf uses the section names in a well understood format to attach the filter to the
> correct function.

## Building

The following will has `rustc` emit the LLVM IR instead of produce a full library.
The LLVM IR can then be fed into `llc` which can generate the correct eBPF object ELF file.

```bash
cargo rustc --release -- --emit=llvm-ir
cp target/release/deps/rust_ebpf_demo-*.ll rust_ebpf_demo.ll
cargo rustc --release -- --emit=llvm-bc
cp target/release/deps/rust_ebpf_demo-*.bc rust_ebpf_demo.bc
llc rust_ebpf_demo.bc -march=bpf -filetype=obj -o rust_ebpf_demo.o
```