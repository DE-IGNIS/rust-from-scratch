# Learning Rust Repository

Welcome to the Learning Rust Repository! This project serves as a structured guide and reference for learning Rust, progressing from basic syntax to intermediate concepts.

Each file in this repository is designed to be a simple, educational, and runnable Rust example.

## Table of Contents

- [Getting Started](#getting-started)
- [Project Structure](#project-structure)
  - [Basics (`basics/`)](#basics-basics)
  - [Ownership & Borrowing (`ownership/`)](#ownership--borrowing-ownership)
  - [Intermediate Concepts (`intermediate/`)](#intermediate-concepts-intermediate)
- [How to Run the Examples](#how-to-run-the-examples)

## Getting Started

To get started, ensure you have Rust installed on your machine. You can install Rust using `rustup`:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Project Structure

The project is organized into three main categories:

### Basics (`basics/`)
These examples cover fundamental Rust syntax and concepts.
- **`hello.rs`**: A simple "Hello World" program demonstrating basic syntax and comments.
- **`ops.rs`**: Covers basic mathematical, boolean, and bitwise operations.
- **`variables.rs`**: Explains primitive (integers, floats, booleans, chars) and compound (tuples, arrays) data types.
- **`tuples.rs`**: Detailed examples on creating, using, and destructuring tuples.
- **`arrays.rs`**: Demonstrates fixed-size arrays and slices.
- **`control_flow.rs`**: Covers basic control flow structures like `if`/`else`, `loop`, `while`, and `for`.
- **`functions.rs`**: Explains how to define functions, return values, and use expressions (like blocks).

### Ownership & Borrowing (`ownership/`)
These examples delve into Rust's unique memory management system.
- **`ownership.rs`**: Introduces the concept of ownership, moving data, and memory management without garbage collection.
- **`move_copy.rs`**: Explains the difference between types that `Copy` (like primitives) and types that move (like `String`), as well as explicit copying using `Clone`.
- **`borrowing.rs`**: Covers immutable and mutable borrowing, and the rules around references.
- **`references_and_dereferencing.rs`**: Shows how to use references (`&`, `&mut`), dereferencing (`*`), and briefly introduces raw pointers and struct borrowing.
- **`slices.rs`**: Demonstrates string slices (`&str`), array/vector slices (`&[T]`), and how they interact with memory.
- **`stack_vs_heap.rs`**: Explains the difference between stack and heap allocation, showing what goes where and why.

### Intermediate Concepts (`intermediate/`)
These examples introduce more complex Rust features for structuring larger applications.
- **`enums.rs`**: Demonstrates defining enums and using `match` for control flow and pattern matching.

## How to Run the Examples

Every file is a standalone Rust program. You can compile and run them directly using `rustc`:

1. Compile the file (e.g., `hello.rs`):
   ```bash
   rustc basics/hello.rs
   ```
2. Run the resulting executable:
   ```bash
   ./hello
   ```

Alternatively, you can use `rust-script` or copy the contents into a `cargo` project if you prefer a more robust environment, but `rustc` is the simplest way to run these examples as intended.
