fn main() {
    // -----------------------------------------------------------
    // 1. Every value has a single owner.
    // -----------------------------------------------------------
    let owner = String::from("I own this heap allocation");
    println!("owner = {}", owner);          // OK – `owner` is still valid

    // -----------------------------------------------------------
    // 2. Moving ownership
    // -----------------------------------------------------------
    // When we assign `owner` to `new_owner`, the *ownership* of the
    // heap data is transferred.  `owner` becomes unusable.
    let new_owner = owner;                   // <-- move
    // println!("{}", owner);               // <- compile error! value moved
    println!("new_owner (after move) = {}", new_owner);

    // -----------------------------------------------------------
    // 3. Borrowing (read‑only reference)
    // -----------------------------------------------------------
    let borrowed = &new_owner;               // a shared reference
    println!("borrowed = {}", borrowed);    // we can still read it
    // `new_owner` is still the real owner and can be used later
    println!("new_owner still usable = {}", new_owner);

    // -----------------------------------------------------------
    // 4. Mutable borrowing (write access)
    // -----------------------------------------------------------
    let mut mutable_owner = String::from("mutable");
    {
        // One mutable reference at a time
        let mref = &mut mutable_owner;
        mref.push_str(" string");
        println!("inside mutable block: {}", mref);
    } // `mref` goes out of scope here, borrow ends
    println!("after mutable block: {}", mutable_owner);

    // -----------------------------------------------------------
    // 5. Ownership across function boundaries
    // -----------------------------------------------------------
    // a) Function **takes** ownership
    let taken = String::from("will be moved");
    takes_ownership(taken);
    // println!("{}", taken); // error – `taken` was moved

    // b) Function only **borrows**
    let still_here = String::from("still owned");
    borrows(&still_here);
    println!("still_here after borrowing = {}", still_here);

    // c) Function **returns** ownership
    let revived = returns_owned_string();
    println!("revived = {}", revived);
}

// ---------------------------------------------------------------
// Helper functions used above
// ---------------------------------------------------------------

/// Consumes the `String`. When the function ends, the value is dropped.
fn takes_ownership(s: String) {
    println!("inside takes_ownership(): {}", s);
    // `s` goes out of scope → memory freed automatically.
}

/// Only reads the string; ownership stays with the caller.
fn borrows(s: &String) {
    println!("inside borrows(): {}", s);
}

/// Creates a new `String` and gives its ownership back to the caller.
fn returns_owned_string() -> String {
    String::from("I was created inside a function and now I belong to you")
}