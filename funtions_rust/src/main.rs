fn main() {
    println!("Hello, world!");
    another_fn();

    fn_args(5);
    print_labeled_measurement(5, 'h');
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
