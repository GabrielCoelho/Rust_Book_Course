use std::io;

fn main() {
    // Integer types. (Usage of underscore to let the LSP know that this variables won't be used)
    //          Signed types can be negative
    let _signed1: i8 = -12; // signed integer of 8 bits
    let _signed2: i16 = -120; // signed integer of 16 bits
    let _signed3: i32 = -1200; // signed integer of 32 bits
    let _signed4: i64 = -12000; // signed integer of 64 bits
    let _signed5: i128 = -12000; // signed integer of 128 bits
    let _signed6: isize = -120000; // signed integer of arch bits
                                   //          Unsigned is always greater than 0
    let _unsigned1: u8 = 12; // unsigned integer of 8 bits
    let _unsigned1: u16 = 120; // unsigned integer of 16 bits
    let _unsigned1: u32 = 1200; // unsigned integer of 32 bits
    let _unsigned1: u64 = 12000; // unsigned integer of 64 bits
    let _unsigned1: u128 = 120000; // unsigned integer of 128 bits
    let _unsigned1: usize = 1200000; // unsigned integer of arch bits

    // The arch size is based on the architecture of your CPU -> 64 or 32 bits.

    // Floating-Point Types
    let _float1: f32 = 2.0; // Single precision Floating-Point
    let _float2: f64 = 2.01; // Double precision Floating-Point

    // Numeric Operations

    let _sum = 5 + 10; // sum of two numbers or more numbers
    let _diff = 95.5 - 4.3; // difference of two or more numbers
    let _product = 4 * 30; // product of two or more numbers
    let _quotient = 56.7 / 32.0; // gives the exact quotient of a division (must be Floating-Point
                                 // type)
    let _truncated = -5 / 3; // when dividing integers, the result will also be a integer (in this
                             // case = -1)
    let _mod = 43 % 2; // same as in C, the percentile sign is used to get the remainder of a
                       // division.

    // Boolean type
    let _this_true = true; // non explicit type annotation
    let _this_false: bool = false; // with explicit type annotation

    // Char types
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻'; // rust does support emojis
                               // Rust's char type represents a Unicode Scalar Value -> Can represent more than just ASCII,
                               // such as accented letters, emojis, Chinese, Japanese and Korean. Going from U+0000 to U+D7FF
                               // and U+E000 to U+10FFFF

    // Tuple Type
    let _a_tuple: (i32, f64, u8) = (500, 5.3, 1);
    // once declared, a tuple cannot change its size. In the above example, the tuple has 3
    // different data types in it. I cannot shrink it to two, or grow it to five. It's only three
    // for all the execution of the program.

    // Accessing a tupple value
    // We can assign the value to other variables like below:
    let (_ax, _ay, _az) = _a_tuple; // where _ax = 500; _ay = 5.3; and _az = 1.

    // Or accessing by the tuple index (starting always in 0)
    let _five_hundred = _a_tuple.0; // accessing the first value of _a_tuple (500)
    let _five_point_three = _a_tuple.1; // acessing the second value of _a_tuple (5.3)
    let _one = _a_tuple.2; // accessing the third value of _a_tuple (1)

    // Arrays
    let _array1 = [1, 2, 3, 4, 5]; // you can either declare a array without the explicit annotation
    let _array2: [u16; 5] = [5, 4, 3, 2, 1]; // or with the explicit annotation, where first it
                                             // comes the data type of your array, and second the
                                             // number of elements it will have.
    let _array3 = [2; 3]; // Or even have an array of same value for each element -> this _array3 is
                          // the same as writing `let _array3 = [2,2,2];`

    // Accessing array elements.
    let _first_element = _array1[0]; // as simple as in c

    // Be careful to not reach an invalid element.
    println!("Please enter an array index.");
    let mut index = String::new();

    // standard reading from terminal
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = _array1[index];

    println!("The value of the element at index {index} is: {element}");
    // the code above will panic if you put an index greater than the max_length of the _array1,
    // which is 4. This code will compile because Rust check at runtime, as the compiler can't
    // possibly know what value a user will enter when they run the code later.
}
