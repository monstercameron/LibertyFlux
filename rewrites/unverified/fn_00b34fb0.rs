// original: 0x00b34fb0 adjust_heap_16b_key0 (proposed)

/// Sift the 16-byte `value` (passed by value on the stack) down into the
/// heap of 16-byte elements at `base`: while the hole index is above `top`,
/// compare the value's leading key float with the parent element's leading
/// float, moving the parent down when the value's key is ordered-above the
/// parent's, then store the value in the final hole. Indices are signed;
/// the parent of hole `h` is `(h - 1) / 2` (signed division). Only the key
/// lanes are compared (ordered `>`, so NaN on either side ends the sift).
/// The return value is the original's register residue: twice the last
/// tested parent index when the sift stopped on a comparison, otherwise the
/// decremented hole (`hole - 1`, or `hole` when the hole was not positive).
/// Original: 0x00b34fb0 (cdecl, seven stack words: base, hole, top, four
/// value words).
lf_checker_rt::export!(cdecl, rw_00b34fb0(
    base: u32,
    hole: u32,
    top: u32,
    v0: u32,
    v1: u32,
    v2: u32,
    v3: u32,
) -> u32 {
    unsafe {
        const ELEM: u32 = 16;
        const N_COPY: usize = 4;
        let value = [v0, v1, v2, v3];
        let value_key = f32::from_bits(v0);
        let top = top as i32;
        let hole_i = hole as i32;
        let mut residue = if hole_i >= 1 { hole_i - 1 } else { hole_i } as u32;
        let mut parent = (hole_i - 1) / 2;
        let mut hole = hole_i;
        while hole > top {
            residue = (parent as u32).wrapping_mul(2);
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
            residue = (if parent >= 1 { parent - 1 } else { parent }) as u32;
            parent = (parent - 1) / 2;
        }
        let slot = base.wrapping_add((hole as u32).wrapping_mul(ELEM));
        core::ptr::copy_nonoverlapping(value.as_ptr(), slot as *mut u32, N_COPY);
        residue
    }
});
