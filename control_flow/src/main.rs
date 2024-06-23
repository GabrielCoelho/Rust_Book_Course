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

    // let number3:u8 = if condition {5} else {"six"}; <- This will cause an error, because is
    // mismatching the types. number3 is a unsigned integer of 8 bits, which cannot take a string
    // such as "six" as its value.
    learning_loops();
}

// Learning Loops.
fn learning_loops() {
    // This is an example of a endless loop, because it doesn't have any condition to exit this
    // repetition block.
    // loop {
    //    println!("again!");
    // }
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            // We learned before that to return something we don't put the semicolon at the end of
            // the line which the returned value is. In case of the loop control flow, we may use
            // the break expression to exit the loop. After the expression, in case we need to
            // return something, we can put it there (as below). The semicolon is needed to end the
            // loop statement by the break expression.
            break counter * 2; // return 10 times 2
        }
        // Is possible to use `return` expression, but it will end the entire function. Break
        // expression only returns from the loop
    };

    println!("The result is {result}");
}
