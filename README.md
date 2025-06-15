# Sink

A CLI tool for deconstructing floats, and other whimsical activities.

Developed as part of a learning exercise while exploring interesting topics such as those presented by the excellent book: [Rust In Action](https://www.manning.com/books/rust-in-action).

## Ch 5: implementing a CPU in Software

> Operations and the data being operated on share the same encoding
Emulating instruction sets
- `operation`: (aka `op`) refers to procedures that are supported natively by the system.
- `Registers`: memory locations that the CPU can access directly
- `opcode`: number that maps operations and operands to CPU registers.
