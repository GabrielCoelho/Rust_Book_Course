fn main() {
    let number = 6;

    // If statement
    // // Note: it doesn'n need any parenthesis to check, only if you want to encapsulate more
    // logical statements
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // If with multiple conditions (Else If)
    if number % 4 == 0 {
        println!("number was divisible by 4");
    } else if number % 3 == 0 {
        println!("number was divisible by 3");
    } else if number % 2 == 0 {
        println!("number was divisible by 2");
    } else {
        println!("number was not divisible by 4, 3 or 2");
    }

    // If in a let statement
    let condition = true;
    let number2: u8 = if condition { 5 } else { 6 };

    println!("The value of the number is {number2}"); // gonna be 5, because condition is true
}
