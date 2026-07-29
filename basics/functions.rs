fn main() {
    // -----------------------
    // 1. Functions & expressions
    // -----------------------

    // Simple function that adds two numbers and returns the result
    let sum = add(7, 3);
    println!("add(7, 3) = {}", sum);

    // Function that returns the larger of two values (expression style)
    let bigger = max(10, 20);
    println!("max(10, 20) = {}", bigger);

    // Function that greets a user – demonstrates ownership/borrowing
    greet("Alice");

    // -----------------------
    // 2. Expressions
    // -----------------------

    // A block can be used as an expression; its last line is the value.
    let computed = {
        let a = 2;
        let b = 5;
        a * b + 1 // <-- this line is the value of the whole block
    };
    println!("result of the block expression = {}", computed);
}

// -----------------------------------------------------------------
// Functions – all are defined *outside* of `main`
// -----------------------------------------------------------------

/// Adds two 32‑bit integers and returns the sum.
fn add(x: i32, y: i32) -> i32 {
    x + y // expression – implicit `return`
}

/// Returns the larger of two numbers.
/// The `if` expression itself yields a value.
fn max(a: i32, b: i32) -> i32 {
    if a > b { a } else { b }
}

/// Greets the given person. Takes a string slice (`&str`)
/// which does not take ownership of the data.
fn greet(name: &str) {
    println!("Hello, {}! Welcome to the Rust tour.", name);
}
