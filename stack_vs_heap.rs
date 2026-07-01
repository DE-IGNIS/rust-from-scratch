// ================================================================
// stack_vs_heap.rs
// A short, beginner‑friendly demo that shows the difference
// between values stored on the **stack** and values stored on the **heap**.
//
// * Stack  – fixed size, fast, automatically freed when the variable
//            goes out of scope.
// * Heap   – dynamic size, may grow or shrink, freed when the owning
//            value (e.g. `String`, `Vec`, `Box`) is dropped.
//
// Compile & run:
//
//     rustc stack_vs_heap.rs
//     ./stack_vs_heap
// ================================================================

fn main() {
    // -----------------------------------------------------------------
    // 1️⃣  Stack‑only data
    // -----------------------------------------------------------------
    // Primitive integers, booleans, chars, fixed‑size arrays and tuples
    // live completely on the stack. Their size is known at compile time.
    let stack_int: i32 = 123;          // 4 bytes on the stack
    let stack_bool: bool = true;      // 1 byte on the stack
    let stack_char: char = '🦀';      // 4 bytes (Unicode scalar) on the stack

    // Fixed‑size array: the whole array is stored inline on the stack.
    let stack_array: [u8; 3] = [10, 20, 30];

    // Tuple containing values of different types – also stack‑only.
    let stack_tuple = (stack_int, stack_bool, stack_char);

    println!("--- Stack‑only values ---");
    println!("int   = {}", stack_int);
    println!("bool  = {}", stack_bool);
    println!("char  = {}", stack_char);
    println!("array = {:?}", stack_array);
    println!("tuple = {:?}", stack_tuple);

    // -----------------------------------------------------------------
    // 2️⃣  Heap‑allocated data
    // -----------------------------------------------------------------
    // Types that may need a size that is not known at compile time
    // (e.g. `String`, `Vec<T>`, `Box<T>`) store their *payload* on the heap.
    // The variable itself (a pointer, length, capacity) lives on the stack.
    //
    // ┌───────────────┐   ┌───────────────────────────────┐
    // │ stack slot    │ → │ heap allocation (actual data)   │
    // └───────────────┘   └───────────────────────────────┘
    //
    // When the stack variable is dropped, its `Drop` implementation
    // automatically frees the heap memory.

    // `String` owns a heap‑allocated UTF‑8 buffer.
    let mut heap_string = String::from("I live on the heap");
    heap_string.push_str(" – and I can grow!");
    println!("\n--- Heap‑allocated `String` ---");
    println!("heap_string = \"{}\"", heap_string);
    println!("(the pointer, length and capacity are on the stack, the bytes are on the heap)");

    // `Vec<T>` (dynamic array) works the same way as `String`.
    let mut heap_vec = vec![1, 2, 3];
    heap_vec.push(4);
    heap_vec.push(5);
    println!("\n--- Heap‑allocated `Vec<i32>` ---");
    println!("heap_vec = {:?}", heap_vec);
    println!("(the `Vec` struct is on the stack, its elements are on the heap)");

    // `Box<T>` explicitly puts a value on the heap and gives you a pointer.
    let boxed_number = Box::new(99);
    println!("\n--- `Box<T>` (explicit heap allocation) ---");
    println!("boxed_number = {}", boxed_number);
    // `*boxed_number` dereferences the pointer to get the inner value.
    println!("*boxed_number (dereferenced) = {}", *boxed_number);

    // -----------------------------------------------------------------
    // 3️⃣  Demonstrating lifetimes: stack value **cannot** outlive its owner,
    //     heap value can be moved around freely because the heap allocation
    //     lives as long as there is an owning pointer to it.
    // -----------------------------------------------------------------
    // Moving a stack value copies it (for `Copy` types) or moves ownership
    // (for non‑`Copy` types) – the original becomes unusable.
    let moved_string = heap_string; // move ownership to `moved_string`
    // println!("{}", heap_string); // ❌ compile error: value moved
    println!("\n--- Moving heap data ---");
    println!("moved_string now owns the heap data: \"{}\"", moved_string);

    // The same thing works for a stack‑only value that implements `Copy`.
    let a = 5;          // i32 implements `Copy`
    let b = a;          // copy, `a` is still usable
    println!("a = {}, b = {} (both still usable because i32 is Copy)", a, b);

    // -----------------------------------------------------------------
    // 4️⃣  When does the heap memory get freed?
    // -----------------------------------------------------------------
    // As soon as the last owning variable that points to that heap block
    // goes out of scope, Rust automatically calls `drop` and frees the memory.
    {
        let temp = String::from("I live only inside this block");
        println!("\nInside inner block: {}", temp);
        // When the block ends, `temp` is dropped → its heap buffer is freed.
    } // <-- `temp` dropped here

    println!("\n--- End of `main` ---");
    // At this point every stack variable (`stack_*`, `stack_tuple`,
    // `boxed_number`, `moved_string`, etc.) is dropped, which in turn
    // releases all heap allocations they owned.
}

// ---------------------------------------------------------------------
// A tiny helper that forces a heap allocation using `Box<T>`.
// This function shows that even though the *pointer* (`Box<T>`) lives on
// the stack, the actual `i32` value it points to lives on the heap.
// ---------------------------------------------------------------------
fn allocate_on_heap(value: i32) -> Box<i32> {
    Box::new(value) // allocation happens on the heap
}