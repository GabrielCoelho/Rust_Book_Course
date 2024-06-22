fn main() {
    println!("Hello, world!");
    another_fn();

    fn_args(5);
    print_labeled_measurement(5, 'h');
    learning_rust_expressions();
    println!(
        "This line will have a number 5 at the end: {}",
        return_five()
    );
}

fn another_fn() {
    println!("This is from another function");
}

// Passing an argument or a parameter, you alwais MUST declare the type of each one of your
// parameters. In this case, the function below is recieving an integer of 8 bits, and printing it
// after.
fn fn_args(x: i8) {
    println!("The value of x is {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

// Learning about Statement and Expression.
// // In Rust, statement are instructions that do something, or some action but don'r return any
// values. But Expressions does return something.
fn learning_rust_expressions() {
    let _x = 9; // this is a statement.

    // Check this code below
    let y = {
        let x = 4;
        x + 1
    };

    // The "let y {...};" block is a statement, but what is inside the curly bracets is an
    // expression. If you pay close attention, there isn't a semicolon at the end of this
    // expression, because the result of x + 1 is RETURNING to the value of y, in other words, the
    // expression is returning a value to the statement.

    println!("The value of y is {y}"); // will return 5
}

// Returning Values via function
// // A function that return some value must be declared with a arrow (->) followed by the data
// type this function will return
fn return_five() -> i32 {
    5 // If a line doesn't have a semicolon at the end, is because this expression is returning
      // some value.
}
