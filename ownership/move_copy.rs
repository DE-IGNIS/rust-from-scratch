fn main() {
    // -----------------------------------------------------------
    // 1. Types that are `Copy` – assignment copies the bits.
    // -----------------------------------------------------------
    let a: i32 = 7;
    let b = a;                // <-- copy, not a move
    println!("a = {}, b = {}", a, b); // both still usable

    let flag: bool = true;
    let flag_copy = flag;    // copy
    println!("flag = {}, flag_copy = {}", flag, flag_copy);

    // -----------------------------------------------------------
    // 2. Types that are **not** `Copy` – they are moved.
    // -----------------------------------------------------------
    let s1 = String::from("I am not Copy");
    // let s2 = s1;            // <-- this would be a move
    // println!("{}", s1);   // error: use after move

    // -----------------------------------------------------------
    // 3. Explicit deep copy with `Clone`
    // -----------------------------------------------------------
    let s2 = s1.clone();       // now we have two independent Strings
    println!("original (s1) = {}", s1);
    println!("clone (s2)    = {}", s2);

    // -----------------------------------------------------------
    // 4. Move semantics in function calls
    // -----------------------------------------------------------
    let moved = String::from("will be moved into a fn");
    takes_string(moved);       // `moved` is no longer valid here
    // println!("{}", moved); // compile error

    // -----------------------------------------------------------
    // 5. Borrowing keeps the original value alive
    // -----------------------------------------------------------
    let still_here = String::from("I stay alive");
    borrows_string(&still_here);
    println!("still_here after borrowing = {}", still_here);

    // -----------------------------------------------------------
    // 6. Returning ownership from a function
    // -----------------------------------------------------------
    let returned = give_me_a_string();
    println!("got back ownership: {}", returned);
}

// ---------------------------------------------------------------
// Functions used for the demo
// ---------------------------------------------------------------

/// Takes ownership of a `String`. The value is dropped when the function ends.
fn takes_string(s: String) {
    println!("inside takes_string(): {}", s);
    // `s` is dropped here.
}

/// Only reads the string; ownership stays with the caller.
fn borrows_string(s: &String) {
    println!("inside borrows_string(): {}", s);
}

/// Creates a new `String` and transfers ownership to the caller.
fn give_me_a_string() -> String {
    String::from("I was created inside give_me_a_string")
}