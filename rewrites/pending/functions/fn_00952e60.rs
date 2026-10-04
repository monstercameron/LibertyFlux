// original: 0x00952e60 bump_counter_b
/// Increment the second 16-bit counter, wrapping back to zero when it
/// reaches 0x7fff. Only the low word of the result is meaningful.
export!(cdecl, rw_00952e60() -> u32 {
    unsafe {
        let slot = global::<u16>(0x011f7108);
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
