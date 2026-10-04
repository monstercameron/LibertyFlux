// original: 0x00b34ed0 adjust_heap_28b_key0 (proposed)

/// Sift the 28-byte `value` (passed by value on the stack) down into the
/// heap of 28-byte elements at `base`: while the hole index is above `top`,
/// compare the value's leading key float with the parent element's leading
/// float, moving the parent down when the value's key is ordered-above the
/// parent's, then store the value in the final hole. Indices are signed;
/// the parent of hole `h` is `(h - 1) / 2` (signed division). Only the key
/// lanes are compared (ordered `>`, so NaN on either side ends the sift);
/// every other byte is moved untouched. Returns the last value word,
/// matching the original's exit register. Original: 0x00b34ed0 (cdecl,
/// ten stack words: base, hole, top, seven value words).
lf_checker_rt::export!(cdecl, rw_00b34ed0(
    base: u32,
    hole: u32,
    top: u32,
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
        const N_COPY: usize = 7;
        let value = [v0, v1, v2, v3, v4, v5, v6];
        let value_key = f32::from_bits(v0);
        let top = top as i32;
        let mut hole = hole as i32;
        let mut parent = (hole - 1) / 2;
        while hole > top {
            let elem = base.wrapping_add((parent as u32).wrapping_mul(ELEM));
            let elem_key =
                f32::from_bits((elem as *const u32).read_unaligned());
            if !(value_key > elem_key) {
                break;
            }
            let slot = base.wrapping_add((hole as u32).wrapping_mul(ELEM));
            core::ptr::copy_nonoverlapping(
                elem as *const u32,
                slot as *mut u32,
                N_COPY,
            );
            hole = parent;
            parent = (parent - 1) / 2;
        }
        let slot = base.wrapping_add((hole as u32).wrapping_mul(ELEM));
        core::ptr::copy_nonoverlapping(value.as_ptr(), slot as *mut u32, N_COPY);
        v6
    }
});
