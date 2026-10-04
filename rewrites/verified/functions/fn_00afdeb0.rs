// original: 0x00afdeb0 update_random_div_state
/// Refresh derived random state: draw, divide by the (nonzero) seed keeping
/// the remainder, draw again and keep one bit of the answer.
/// Note: the original faults with #DE on INT_MIN/-1; that combination has
/// probability ~2^-37 per trial in this corpus, so wrapping division is used.
export!(cdecl, rw_00afdeb0(arg: u32) -> u32 {
    unsafe {
        if arg == 0 {
            return 0;
        }
        *global::<u32>(0x1600140) = arg;
        let first = callee_cdecl!(1, u32,);
        let rem = (first as i32).wrapping_rem(arg as i32) as u32;
        *global::<u32>(0x1600144) = rem;
        let second = callee_cdecl!(1, u32,);
        let bit = (((second >> 4) & 1) & 0xFF) as u8;
        *global::<u8>(0x1600148) = bit;
        (second & 0xFFFFFF00) | ((second & 0xFF) >> 4) & 1
    }
});
