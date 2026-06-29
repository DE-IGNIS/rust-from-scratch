fn main() {
    // -----------------------
    // 1. Primitive data types
    // -----------------------

    // Integer (i32 by default)
    let int_a: i32 = 42;
    println!("int_a = {}", int_a);

    // Float (f64 by default)
    let float_b: f64 = 3.1415;
    println!("float_b = {}", float_b);

    // Boolean
    let bool_c: bool = true;
    println!("bool_c = {}", bool_c);

    // Char – a single Unicode scalar value
    let char_d: char = '🦀';
    println!("char_d = {}", char_d);

    // -----------------------
    // 2. Compound data types
    // -----------------------

    // Tuple – a fixed‑size collection of different types
    let tup: (i32, f64, char) = (int_a, float_b, char_d);
    // Destructure the tuple into individual bindings
    let (x, y, z) = tup;
    println!("tuple = ({}, {}, '{}')", x, y, z);

    // Array – a fixed‑size collection of the same type
    let arr: [i32; 5] = [10, 20, 30, 40, 50];
    println!("first element of array = {}", arr[0]);

    // Iterate over the array with a `for` loop
    println!("array elements:");
    for element in arr.iter() {
        println!("  {}", element);
    }

    // -----------------------
    // 3. Control flow
    // -----------------------

    // ---- if / else -------------------------------------------------
    if bool_c && int_a > 0 {
        println!("Both bool_c is true and int_a is positive.");
    } else if int_a == 0 {
        println!("int_a is zero!");
    } else {
        println!("Something else.");
    }

    // ---- match -----------------------------------------------------
    // Using an enum just for the sake of a clean `match`
    enum Day {
        Monday,
        Tuesday,
        Wednesday,
        Thursday,
        Friday,
        Saturday,
        Sunday,
    }

    let today = Day::Wednesday;

    match today {
        Day::Monday => println!("Back to work!"),
        Day::Tuesday | Day::Wednesday | Day::Thursday => {
            println!("Mid‑week vibes.");
        }
        Day::Friday => println!("Weekend is near!"),
        Day::Saturday | Day::Sunday => println!("Enjoy the weekend!"),
    }

    // ---- loop (infinite) -------------------------------------------
    // We'll break after 3 iterations to avoid an actual endless loop.
    let mut counter = 0;
    loop {
        println!("loop counter = {}", counter);
        counter += 1;
        if counter >= 3 {
            break; // exit the loop
        }
    }

    // ---- while ------------------------------------------------------
    let mut n = 5;
    while n != 0 {
        println!("while counting down: {}", n);
        n -= 1;
    }

    // ---- for (range) ------------------------------------------------
    // Print the numbers 1 through 4 (exclusive upper bound)
    for i in 1..5 {
        println!("for-loop i = {}", i);
    }

    // -----------------------
    // 4. Functions & expressions
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
    // 5. Expressions
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
