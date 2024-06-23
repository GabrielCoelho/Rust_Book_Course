fn main() {
    // On Ownership chapter, we learned that passing a String to a function, would move and then be
    // droped. In other words, we passed the ownership of the string from the fn main to other
    // function.
    // Now we gonna learn to borrow.

    let s1 = String::from("hello");
    // Passing the address of the variable, we maintain it usable after passing it to a function.
    let len = calculate_length(&s1);

    println!("The length of '{s1}' is {len}.");
}

// But you need to specify that you're getting a address
fn calculate_length(s: &String) -> usize {
    s.len()
}
