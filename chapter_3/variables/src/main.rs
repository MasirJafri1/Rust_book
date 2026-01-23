fn main() {
    // Invalid: 'fn' is a reserved keyword in Rust
    // let fn = 1;

    // Variables are immutable by default.
    // To make them mutable, add 'mut' before the variable name.
    // This indicates to future readers that the value will change.

    let a: i32 = 1;
    // Invalid: 'a' is immutable
    // a = 100;

    let mut b: i32 = 2;
    b = b + 198; // Valid: 'b' is mutable

    println!("The value of a and b is {} and {}", a, b);

    // Constants
    // Constants are always immutable and must have a type annotation.
    // They can be declared in any scope, including global.
    // They must be set to a constant expression, not runtime values.

    const PI: f64 = 3.14;

    // Invalid: 'mut' is not allowed with constants
    // const mut MAX_POINTS: u32 = 100_000;

    // Invalid: Cannot include dynamic variables in constants
    // const MAX_POINTS: u32 = 100 + 1000 + b;

    println!("The value of PI is {}", PI);

    // Shadowing
    // Shadowing allows declaring a new variable with the same name.
    // The new variable shadows the old one until the scope ends.
    // This is different from mutating the original variable.

    let x = 5;
    let x = x + 1; // Creates a new 'x', shadows the previous one

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);

    // Example of shadowing with type change
    let apple: i32 = 10;
    println!("The value of apple is: {}", apple);

    // Invalid: Type mismatch even for mutable variables
    // apple = true;

    let apple = apple + 5; // Shadowing: new 'apple' with updated value
    println!("The value of apple after shadowing is: {}", apple);

    // Valid: Shadowing allows changing the type
    let apple: bool = true; // Shadowing: new 'apple' with different type
    println!("The value of apple now is: {}", apple);
}
