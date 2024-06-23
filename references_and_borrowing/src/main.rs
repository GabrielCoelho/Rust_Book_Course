fn main() {
    // On Ownership chapter, we learned that passing a String to a function, would move and then be
    // droped. In other words, we passed the ownership of the string from the fn main to other
    // function.
    // Now we gonna learn to borrow.

    let s1 = String::from("hello");
    // Passing the address of the variable, we maintain it usable after passing it to a function.
    let len = calculate_length(&s1);

    // The & syntax create a reference that refers to the value of s1 but does not own it.
    // Because it does not own it, the value it points to will not be dropped when the reference
    // stops being used.

    println!("The length of '{s1}' is {len}.");
}

// But you need to specify that you're getting a address
fn calculate_length(s: &String) -> usize {
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, it is not dropped.
