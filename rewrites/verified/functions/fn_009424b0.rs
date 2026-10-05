// original: 0x009424b0 streaming_lane_zero (proposed)

/// Clear the three header words of the indexed lane's base record.
///
/// The object points into lane `n` (read at `this + 0x2048`) of an array
/// with stride 0x100c; stores zero to the words at offsets 0x2030, 0x2034
/// and 0x2038 of the lane base (`this - n * 0x100c`). Returns the lane's
/// byte offset from its base (`n * 0x100c`) in `eax`.
///
/// Original: 0x009424b0 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_009424b0(this: u32) -> u32 {
    unsafe {
        const LANE: u32 = 0x2048;
        const STRIDE: u32 = 0x100C;
        const W0: u32 = 0x2030;
        const W1: u32 = 0x2034;
        const W2: u32 = 0x2038;
        let n = ((this + LANE) as *const u32).read_unaligned();
        let offset = n.wrapping_mul(STRIDE);
        let base = this.wrapping_sub(offset);
        ((base + W0) as *mut u32).write_unaligned(0);
        ((base + W1) as *mut u32).write_unaligned(0);
        ((base + W2) as *mut u32).write_unaligned(0);
        offset
    }
});
