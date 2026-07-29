// ---------------------------------------------------------------
// slices_basics.rs
// A compact, beginner‑level demo of:
//
//   • `&str` – a view into a UTF‑8 string
//   • Slices of arrays/s vectors (`&[T]`)
//   • How to create, index, iterate, and mutate slices
//
// Compile with `rustc slices_basics.rs` and run the executable.
// ---------------------------------------------------------------

fn main() {
    // ===========================================================
    // 1️⃣  String slices – `&str`
    // ===========================================================
    // A `String` owns heap‑allocated UTF‑8 data.
    // A `&str` is a *view* into that data; it does **not** own anything.
    let full: String = String::from("Hello, Rust slices!");
    println!("full string = \"{}\"", full);

    // Borrow a slice that covers the whole string.
    // The `as_str()` method returns a `&str` that points at the same data.
    let slice_all: &str = full.as_str();
    println!("slice_all (the whole string) = \"{}\"", slice_all);

    // Slice a portion of the string – the syntax is the same as for arrays.
    // IMPORTANT: the indices are **byte** offsets, NOT character indices!
    //   The string above is pure ASCII, so each character = 1 byte.
    let slice_hello: &str = &full[0..5]; // "Hello"
    let slice_rust: &str = &full[7..11]; // "Rust"
    println!("slice_hello = \"{}\"", slice_hello);
    println!("slice_rust  = \"{}\"", slice_rust);

    // You can also take a slice using a range that omits the start or end.
    let slice_from_comma: &str = &full[5..]; // from ',' to the end
    let slice_up_to_comma: &str = &full[..5]; // up to but not including index 5
    println!("slice_from_comma = \"{}\"", slice_from_comma);
    println!("slice_up_to_comma = \"{}\"", slice_up_to_comma);

    // ------------------------------------------------------------
    // 2️⃣  Array and vector slices – `&[T]`
    // ------------------------------------------------------------
    // Fixed‑size array (type `[i32; 5]`) – the data lives on the stack.
    let numbers: [i32; 5] = [10, 20, 30, 40, 50];
    println!("numbers = {:?}", numbers);

    // A slice is a reference to a *contiguous* part of the array.
    // `&numbers[1..4]` has type `&[i32]`.
    let slice_mid: &[i32] = &numbers[1..4]; // elements 20,30,40
    println!("slice_mid (numbers[1..4]) = {:?}", slice_mid);

    // Slices are read‑only by default.  Trying to modify them will not compile.
    // Uncommenting the line below produces a compile error:
    // slice_mid[0] = 999; // error: cannot assign to `slice_mid[_]` which is behind a `&` reference

    // ------------------------------------------------------------
    // 2️⃣b Mutable slice of a mutable vector
    // ------------------------------------------------------------
    // A `Vec<T>` owns its elements on the heap.  By borrowing it mutably we can
    // get a *mutable slice* (`&mut [T]`) that lets us change the elements.
    let mut vec_numbers = vec![1, 2, 3, 4, 5];
    println!("vec_numbers before = {:?}", vec_numbers);

    {
        // `&mut vec_numbers[1..4]` creates a mutable slice that covers 2,3,4.
        let slice_mut: &mut [i32] = &mut vec_numbers[1..4];
        // We can modify the slice in place.
        for v in slice_mut.iter_mut() {
            *v *= 10; // multiply each element by 10
        }
        println!("inside mutable slice block: slice_mut = {:?}", slice_mut);
    } // mutable borrow ends here

    // The original vector now reflects the changes we made through the slice.
    println!("vec_numbers after = {:?}", vec_numbers);

    // ------------------------------------------------------------
    // 3️⃣  Functions that accept slices
    // ------------------------------------------------------------
    let words = ["apple", "banana", "cherry", "date"];
    print_slice(&words);            // pass a whole array as a slice
    print_slice(&words[1..3]);       // pass only a part of it

    // A function that works on a string slice (`&str`).
    let greeting = "Good morning, world!";
    count_vowels(greeting);
}

// -----------------------------------------------------------------
// Helper: print any slice of items that implement `Debug`
// -----------------------------------------------------------------
fn print_slice<T: std::fmt::Debug>(s: &[T]) {
    println!("slice contains {} elements: {:?}", s.len(), s);
}

// -----------------------------------------------------------------
// Helper: count vowels in a string slice (`&str`)
// -----------------------------------------------------------------
fn count_vowels(s: &str) {
    // `chars()` iterates over Unicode scalar values (not bytes).
    let mut count = 0;
    for ch in s.chars() {
        if "aeiouAEIOU".contains(ch) {
            count += 1;
        }
    }
    println!("'{}' contains {} vowel(s)", s, count);
}