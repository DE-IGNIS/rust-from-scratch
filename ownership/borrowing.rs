// ---------------------------------------------------------------
// borrowing.rs
// Explains immutable and mutable borrowing in Rust.
// ---------------------------------------------------------------

fn main() {
    // -----------------------------------------------------------
    // 1. Immutable borrow – read‑only access
    // -----------------------------------------------------------
    let message = String::from("Hello, borrowing!");
    // `&message` creates an immutable reference; we cannot modify
    // the original data through this reference.
    let r1: &String = &message;
    let r2: &String = &message; // many immutable refs are allowed

    println!("r1 says: {}", r1);
    println!("r2 says: {}", r2);
    // `message` is still usable because we only borrowed it.
    println!("original = {}", message);

    // -----------------------------------------------------------
    // 2. Mutable borrow – exclusive read/write access
    // -----------------------------------------------------------
    let mut counter = 0;
    // `&mut counter` creates a *single* mutable reference.
    // No other references (mutable or immutable) may exist at the same time.
    {
        let counter_mut: &mut i32 = &mut counter;
        *counter_mut += 1;          // dereference to modify the value
        *counter_mut += 2;
        println!("inside mutable block: counter = {}", counter_mut);
    } // `counter_mut` goes out of scope → the borrow ends

    // Now we can read `counter` again, or take another mutable borrow.
    println!("after mutable block: counter = {}", counter);

    // -----------------------------------------------------------
    // 3. Mixing immutable and mutable borrows – what is NOT allowed
    // You cannot have a mutable borrow if an immutable borrow exists.
    // -----------------------------------------------------------
    let mut data = vec![1, 2, 3];

    // ---------------------------------------------------------
    // let immut_ref = &data;      // ok: immutable borrow
    // let mut_ref = &mut data;    // ❌ error: cannot borrow `data` as mutable
    //                               // because it is also borrowed as immutable
    // ---------------------------------------------------------

    // The fix is to end the first borrow (by limiting its scope) before
    // taking a mutable one.
    {
        let immut_ref = &data; // immutable borrow lives only in this block
        println!("first element (immutable) = {}", immut_ref[0]);
    } // immut_ref dropped here

    // Now a mutable borrow is allowed.
    {
        let mut_ref = &mut data; // exclusive mutable reference
        mut_ref.push(4);
        println!("vector after push (mutable) = {:?}", mut_ref);
    } // mutable borrow ends

    // -----------------------------------------------------------
    // 4. Borrowing through function parameters
    // -----------------------------------------------------------
    let name = String::from("Alice");
    greet(&name);               // `&String` = immutable borrow
    // `name` is still usable here
    println!("still own name after greet = {}", name);

    let mut score = 0;
    increase_score(&mut score); // `&mut i32` = mutable borrow
    println!("score after increase_score = {}", score);

    // -----------------------------------------------------------
    // 5. Slices – a special kind of immutable reference
    // -----------------------------------------------------------
    let numbers = [10, 20, 30, 40, 50];
    // `&numbers[1..4]` is a slice (`&[i32]`) that points to part of the array.
    let slice: &[i32] = &numbers[1..4];
    println!("slice of numbers = {:?}", slice);
    // slices are read‑only; to modify you would need a `&mut [i32]`.

    // -----------------------------------------------------------
    // 6. Borrowing fields of a struct
    // -----------------------------------------------------------
    let mut person = Person {
        name: String::from("Bob"),
        age: 28,
    };

    // Immutable borrow of a single field
    let name_ref = &person.name;
    println!("person's name (borrowed) = {}", name_ref);

    // Mutable borrow of a different field – allowed because the
    // borrows are of *disjoint* parts of the struct.
    {
        let age_mut = &mut person.age;
        *age_mut += 1;
        println!("person's age after birthday (mutable) = {}", age_mut);
    }

    // Full mutable borrow of the whole struct
    {
        let person_mut = &mut person;
        person_mut.name.push_str(" Jr.");
        person_mut.age += 5;
        println!("person after big update = {:?}", person_mut);
    }

    // We can still read the fields after the mutable borrow ends.
    println!("final person = {:?}", person);
}

// -----------------------------------------------------------------
// Helper functions used in the demo
// -----------------------------------------------------------------

/// Takes an immutable reference and prints a greeting.
/// The function **does not** take ownership.
fn greet(name: &String) {
    println!("Hello, {}!", name);
}

/// Takes a mutable reference to an `i32` and increments it.
/// Because we have `&mut`, we are allowed to modify the caller's data.
fn increase_score(score: &mut i32) {
    *score += 10; // dereference to write through the reference
}

// -----------------------------------------------------------------
// A tiny struct to show borrowing of fields
// -----------------------------------------------------------------
#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}