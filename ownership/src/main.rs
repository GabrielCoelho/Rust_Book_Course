fn main() {
    let _s = "hello";
    {
        // the previous s is not valid inside this scope
        let _s = "hello"; // now that an inside "S" is declared, is now valid this point forward
                          // do stuff with s
    } // this scope is over, and s is no longer valid

    // Learning about the String Type
    {
        let mut s = String::from("hello");
        s.push_str(", world"); // append to the string
        println!("{s}"); // print "Hello, world"

        let _s2 = s;
        // println!("{s}"); <- causes an error, because we already assigned the pointer value of s
        // to a new variable. This is called "Borrowing" in rust. Once something is borrowed, rust
        // consider s as no longer valid.
        println!("{_s2}");
    }
}
