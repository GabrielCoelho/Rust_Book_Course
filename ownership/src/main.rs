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
    }
}
