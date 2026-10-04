// original: 0x00952db0 bump_counter_a
/// Increment the first 16-bit counter, wrapping back to zero when it reaches
/// 0x7fff. Only the low word of the result is meaningful.
export!(cdecl, rw_00952db0() -> u32 {
    unsafe {
        let slot = global::<u16>(0x011f7100);
        let v = (*slot).wrapping_add(1);
        if v == 0x7fff {
            *slot = 0;
            0
        } else {
            *slot = v;
            v as u32
        }
    }
});
