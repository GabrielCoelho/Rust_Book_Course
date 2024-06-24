enum Message {
    Quit,                       // Doesn't have  any associated data at all
    Move { x: i32, y: i32 },    // has named fields, like a struct
    Write(String),              // tuple
    ChangeColor(i32, i32, i32), // tuple
}

fn main() {
    let m = Message::Write(String::from("This is my message"));
}
