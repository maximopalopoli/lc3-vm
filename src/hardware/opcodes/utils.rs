/// If the first number from left to right is a 1, extends the 1. Otherwise, returns the original value
pub fn sign_extend(mut x: u16, bit_count: i32) -> u16 {
    if (x >> (bit_count - 1)) & 1 != 0 {
        x |= 0xFFFF << bit_count;
    }
    x
}
