fn main() {
    println!("Getting started with variables in Rust\n");

    // There are two types of variables in Rust
    // 1. Immutable variables
    // 2. Mutable variables

    // All variables are immutable by default
    // It means onces it is assigned a value, it cannot be changed

    // Variables can be declared using 'let' or 'const' keyword

    // Using 'let' keyword
    // Rust is a strongly typed language

    // NUMBER TYPES
    number_types();
}

fn number_types() {
    // By default if we assign a number to a variable
    // It automatically assigns the 'i32' type as default type
    // There can be either signed(negatives) or unsigned(positives) number types
    let _my_variable: i32 = 10;

    // As _my_variable is immutable, we cannot change its value
    // Uncomment the code and you will see error
    // cannot mutate immutable variable `_my_variable`
    // _my_variable = 20;

    // We can make _my_variable mutable by using 'mut' keyword
    // Or we can reassign the variable with a new type called shadowing
    let mut _my_variable: u64 = 10;
    _my_variable = 30 + _my_variable;

    // Using 'const' keyword
    // consts are always immutable
    // They need to be declared with a type
    const MY_CONSTANT: i64 = 100;

    println!("_my_variable: {}", _my_variable);
    println!("MY_CONSTANT: {}", MY_CONSTANT);
}
