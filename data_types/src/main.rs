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
    let _float1: f32 = 2.0;
    let _float2: f64 = 2.01;
}
