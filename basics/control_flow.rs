fn main() {
    let bool_c: bool = true;
    let int_a: i32 = 42;

    // -----------------------
    // 1. Control flow
    // -----------------------

    // ---- if / else -------------------------------------------------
    if bool_c && int_a > 0 {
        println!("Both bool_c is true and int_a is positive.");
    } else if int_a == 0 {
        println!("int_a is zero!");
    } else {
        println!("Something else.");
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
}
