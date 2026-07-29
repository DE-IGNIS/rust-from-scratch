fn main() {
    // -----------------------------------------------------------------
    // 1. A reference that would become dangling – compile‑time error
    // -----------------------------------------------------------------
    // The block below creates a `String` that lives only inside the
    // block.  If we tried to keep a reference to it outside the block,
    // the compiler would reject the code because the reference would
    // dangle after the `String` is dropped.
    //
    // Uncomment the lines marked “ERROR” to see the compiler message.
    //
    // -------------------------------------------------------------
    // {
    //     let short_lived = String::from("I disappear");
    //     // `ref_to_short` would point to `short_lived`
    //     let ref_to_short = &short_lived; // ← borrow starts here
    //     println!("inside block: {}", ref_to_short);
    // } // `short_lived` is dropped here
    // // ERROR: `ref_to_short` would be used after the value it points to
    // // was freed.
    // // println!("outside block: {}", ref_to_short);
    // -------------------------------------------------------------
    //
    // The code above does **not** compile, which is exactly what we
    // want – Rust guarantees that no dangling reference can ever be
    // created.

    // -----------------------------------------------------------------
    // 2. A *valid* reference whose lifetime is clearly bounded.
    // -----------------------------------------------------------------
    let outer = String::from("I live long enough");
    {
        // `inner_ref` lives only inside this inner block, so the borrow
        // ends when the block ends – no danger of dangling.
        let inner_ref = &outer;
        println!("inner_ref (still valid) = {}", inner_ref);
    } // `inner_ref` dropped, borrow ends
    // `outer` is still alive, so we can use it again.
    println!("outer after inner block = {}", outer);

    // -----------------------------------------------------------------
    // 3. Returning a slice that lives as long as its source string.
    // -----------------------------------------------------------------
    let sentence = String::from("Rust prevents dangling references");
    let first = first_word(&sentence);
    println!("first word of the sentence = \"{}\"", first);
    // `sentence` is still owned here, therefore `first` (a `&str` slice)
    // is perfectly safe.

    // -----------------------------------------------------------------
    // 4. Trying to return a reference to a *local* variable (illegal).
    // -----------------------------------------------------------------
    // This function would try to give the caller a reference that points
    // to data created inside the function.  The data is dropped when the
    // function returns, so the reference would dangle.  Rust refuses to
    // compile it – the error is shown in the commented‑out version.
    //
    // let bad = bad_reference();          // <-- compile error!
    // println!("bad = {}", bad);
    //
    // The correct way is to return the **owned** value instead of a
    // reference, as shown in `good_reference`.

    let good = good_reference();
    println!("good (owned String returned) = {}", good);
}

// -----------------------------------------------------------------
// 3. Helper that returns a slice (`&str`) of the first word.
//    The lifetime `'a` tells the compiler that the returned slice lives
//    exactly as long as the input slice, so no dangling can happen.
// -----------------------------------------------------------------
fn first_word<'a>(s: &'a str) -> &'a str {
    // Find the first space character; if none is found we return the whole string.
    match s.find(' ') {
        Some(idx) => &s[0..idx],
        None => s,
    }
}

// -----------------------------------------------------------------
// 4a. ❌ Illegal: returns a reference to a locally created String.
//      The function is commented out because it does NOT compile.
// -----------------------------------------------------------------
// fn bad_reference<'a>() -> &'a String {
//     let local = String::from("I will be dropped");
//     &local   // error: returns a reference to data owned by the function
// }
//
// -----------------------------------------------------------------
// 4b. ✅ Correct: returns the owned `String` itself.
//      The caller takes ownership, so there is no dangling reference.
// -----------------------------------------------------------------
fn good_reference() -> String {
    String::from("I am owned by the caller")
}