fn main() {
    let age = 19;
    // age = 100; // By default all the variables in rust are immutable
    println!("My age is {age}");

    // for mutable variable
    let mut num = 100;
    num = num + 19;

    // This line prints the string that now contains the user’s input. The {} set of curly brackets is a placeholder: Think of {} as little crab pincers that hold a value in place. When printing the value of a variable, the variable name can go inside the curly brackets. When printing the result of evaluating an expression, place empty curly brackets in the format string, then follow the format string with a comma-separated list of expressions to print in each empty curly bracket placeholder in the same order. Printing a variable and the result of an expression in one call to println!()

    println!("The num is {num} ");

    println!("The num is {} and the age is {} ", num, age + 1);

    let a = true;
    match a {
        //match is exhaustive so it expects all the cases to be handled
        true => println!("It is true"),
        false => println!("It is false"),
    }
}
