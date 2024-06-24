fn main() {
    // Learning Option<T>
    let _some_number: Option<i32> = Some(5);
    let _some_char = Some('e');
    let _absent_number: Option<i32> = None;

    let _six = plus_one(_some_number);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        _ => None,
    }
}
