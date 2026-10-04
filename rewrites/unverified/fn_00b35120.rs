// original: 0x00b35120 linear_insert_28b_key_c (proposed)

/// Insert the 28-byte `value` (passed by value on the stack) into the run of
/// 28-byte elements ending at `end`: walk one element back at a time while
/// the value's key float at offset 12 is ordered-above the element's key at
/// the same offset, shifting each passed element one slot forward, then
/// store the value in the freed slot. Only the key lanes are compared
/// (ordered `>`, so NaN on either side ends the walk); every other byte is
/// moved untouched. Returns the word at offset 16 of the last tested
/// element, matching the original's exit register. Original: 0x00b35120
/// (cdecl, eight stack words: end, seven value words).
lf_checker_rt::export!(cdecl, rw_00b35120(
    end: u32,
    v0: u32,
    v1: u32,
    v2: u32,
    v3: u32,
    v4: u32,
    v5: u32,
    v6: u32,
) -> u32 {
    unsafe {
        const ELEM: u32 = 28;
        const KEY_OFF: u32 = 12;
        const RET_OFF: u32 = 16;
        const N_COPY: usize = 7;
        let value = [v0, v1, v2, v3, v4, v5, v6];
        let value_key = f32::from_bits(v3);
        let mut slot = end;
        let mut elem = end.wrapping_sub(ELEM);
        let mut last_tested = elem;
        loop {
            last_tested = elem;
            let elem_key = f32::from_bits(
                ((elem + KEY_OFF) as *const u32).read_unaligned(),
            );
            if !(value_key > elem_key) {
                break;
            }
            core::ptr::copy_nonoverlapping(
                elem as *const u32,
                slot as *mut u32,
                N_COPY,
            );
            slot = elem;
            elem = elem.wrapping_sub(ELEM);
        }
        core::ptr::copy_nonoverlapping(
            value.as_ptr(),
            slot as *mut u32,
            N_COPY,
        );
        ((last_tested + RET_OFF) as *const u32).read_unaligned()
    }
});
