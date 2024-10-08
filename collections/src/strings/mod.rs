pub fn learning_strings() {
    let mut s = String::new();

    let data = "Some text";
    s = data.to_string();
    println!("{s}");
    s = String::from("Mutated Text");
    println!("{s}");

    {
        let mut s1 = String::from("foo");
        let s2 = String::from("bar");
        s1.push_str(&s2);
        // push_str took ownership, that's why I had to borrow with &
        println!("{s1} has a s2 string in it, as s2 = {s2}");
    }

    {
        let s1 = String::from("Hello, ");
        let s2 = String::from("world!");
        // let s3 = s1 + &s2; // s1 was moved
        let s5 = format!("{s1}{s2}!{s2}-- {s1}");

        println!("{s5}");
        let mut s4 = String::from("lo");
        s4.push_str("l");
    }

    {
        let hello = "hello";
        for c in hello.chars() {
            println!("{c}");
        }
        for chars in hello.bytes() {
            println!("{chars}");
        }
    }
}
