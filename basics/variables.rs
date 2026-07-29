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
}
