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

    mutable_reff();
}

// But you need to specify that you're getting a address
fn calculate_length(s: &String) -> usize {
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, it is not dropped.

// First we change s to be mut. Then we create a mutable reference with &mut s
// where we call the change function, and update the function signature to
// accept a mutable reference with some_string: &mut String. This makes it very
// clear that the change function will mutate the value it borrows.
fn mutable_reff() {
    let mut s = String::from("hello");

    change(&mut s);
    println!("{s}");
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
