pub fn get_bit(num: &u32, bit: u8) -> bool {
    debug_assert!(bit < 32, "bit no must be lower than 32");
    let mask = 1u32 << bit;
    let val = num & mask;
    val != 0
}

pub fn set_bit(num: &mut u32, bit: u8, val: bool) {
    debug_assert!(bit < 32, "bit no must be lower than 32");
    let mask = 1u32 << bit;
    if val {
        *num |= mask;
    } else {
        *num &= !mask;
    }
}

pub fn copy_bit(src: &u32, src_pos: u8, dst: &mut u32, dst_pos: u8) {
    debug_assert!(src_pos < 32 && dst_pos < 32, "bit no must be lower than 32");
    set_bit(dst, dst_pos, get_bit(src, src_pos));
}

pub fn copy_bits(src: &u32, src_pos: u8, dst: &mut u32, dst_pos: u8, n: u8) {
    debug_assert!(src_pos < 32 && dst_pos < 32, "bit no must be lower than 32");
    for i in 0..n {
        set_bit(dst, dst_pos + i, get_bit(src, src_pos + i));
    }
}
