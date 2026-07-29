// ---------------------------------------------------------------
// references_and_dereferencing.rs
// A compact, beginner‑friendly tour of Rust references (`&`, `&mut`)
// and dereferencing (`*`).  The program compiles and runs as‑is.
// ---------------------------------------------------------------

fn main() {
    // -----------------------------------------------------------
    // 1. Simple immutable reference
    // -----------------------------------------------------------
    let x: i32 = 10;
    let r: &i32 = &x;                 // `r` points to `x` but cannot change it
    println!("x = {}, r (ref to x) = {}", x, r);
    // Dereferencing the immutable reference gives us the original value:
    println!("*r = {}", *r);         // `*r` = 10

    // -----------------------------------------------------------
    // 2. Mutable reference – exclusive read/write access
    // -----------------------------------------------------------
    let mut y: i32 = 20;
    {
        let r_mut: &mut i32 = &mut y;   // `r_mut` can modify `y`
        *r_mut += 5;                    // deref + assign
        println!("inside mutable block: *r_mut = {}", *r_mut);
    } // `r_mut` goes out of scope, borrow ends
    println!("after mutable block: y = {}", y);

    // -----------------------------------------------------------
    // 3. References in function parameters
    // -----------------------------------------------------------
    // a) Immutable borrow – the function only reads the value.
    let val = 42;
    print_immutable(&val);
    // `val` is still usable after the call.
    println!("val after print_immutable = {}", val);

    // b) Mutable borrow – the function can change the caller's data.
    let mut count = 0;
    increment(&mut count);
    println!("count after increment = {}", count);

    // -----------------------------------------------------------
    // 4. Returning a reference (lifetime annotation in brief)
    // -----------------------------------------------------------
    // The `first` function returns a reference to the first element of a slice.
    let numbers = [3, 1, 4, 1, 5];
    let first_ref = first(&numbers);
    println!("first element of numbers = {}", *first_ref);

    // -----------------------------------------------------------
    // 5. Dereferencing a raw pointer (unsafe block)
    // -----------------------------------------------------------
    // Raw pointers (`*const T` / `*mut T`) are not checked by the borrow
    // checker.  They can be created from references and used only inside
    // an `unsafe` block.
    let z: i32 = 99;
    let raw_ptr: *const i32 = &z as *const i32;
    unsafe {
        // Dereferencing a raw pointer is unsafe because the compiler
        // cannot guarantee the pointer is valid.
        println!("raw pointer points to {}", *raw_ptr);
    }

    // -----------------------------------------------------------
    // 6. References to struct fields
    // -----------------------------------------------------------
    let mut person = Person {
        name: String::from("Ada"),
        age: 28,
    };

    // Immutable borrow of a field
    let name_ref: &String = &person.name;
    println!("person's name (borrowed) = {}", name_ref);

    // Mutable borrow of a different field – allowed because the borrows
    // are of *disjoint* parts of the struct.
    {
        let age_mut: &mut u32 = &mut person.age;
        *age_mut += 1;
        println!("person's age after birthday (mutable) = {}", *age_mut);
    }

    // Full mutable borrow of the whole struct
    {
        let person_mut: &mut Person = &mut person;
        person_mut.name.push_str(" Lovelace");
        person_mut.age += 5;
        println!("person after big update = {:?}", person_mut);
    }

    // We can still read the struct after the mutable borrow ends.
    println!("final person = {:?}", person);
}

// -----------------------------------------------------------------
// Helper functions used in the demo
// -----------------------------------------------------------------

/// Takes an immutable reference and prints it.  No ownership change.
fn print_immutable(v: &i32) {
    // `*v` dereferences the `&i32` to get the underlying `i32`.
    println!("print_immutable sees: {}", *v);
}

/// Takes a mutable reference and increments the value.
fn increment(counter: &mut i32) {
    *counter += 1; // dereference then modify
}

/// Returns a reference to the first element of a slice.
/// The lifetime `'a` tells the compiler that the returned reference lives
/// as long as the input slice lives.
fn first<'a>(slice: &'a [i32]) -> &'a i32 {
    &slice[0] // `&` creates a reference; the compiler knows it points into `slice`
}

// -----------------------------------------------------------------
// A tiny struct to illustrate borrowing of fields
// -----------------------------------------------------------------
#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}