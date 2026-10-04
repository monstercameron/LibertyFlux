// original: 0x00b05750 bump_sequence_counter

/// Increments the sequence word at +0x7e8 and returns its previous value.
export!(thiscall, rw_00b05750(obj: *mut u8) -> u32 {
    unsafe {
        let slot = obj.add(0x7e8) as *mut u32;
        let old = *slot;
        *slot = old.wrapping_add(1);
        old
    }
});
