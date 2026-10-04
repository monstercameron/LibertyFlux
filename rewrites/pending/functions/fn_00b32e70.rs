// original: 0x00b32e70 ordered_pair_check (proposed)

/// Decide whether two records form an accepted ordered pair.
///
/// `first` and `second` point to records, `d` and `c` are bounds. A compare
/// callee first orders the records' head words: a negative answer returns the
/// answer with its low byte set to 1. Otherwise the tag words at `+0x44` are
/// compared: differing tags return the first tag with its low byte set to 1
/// when `c` is strictly greater than `d` (signed), else the first tag with
/// its low byte cleared. Equal tags consult the first head word minus 12: an
/// out-of-range value returns itself with its low byte cleared; heads 12,
/// 22, 23 and 26 return 1 when `c` is strictly greater than `d`, else 0; any
/// other head returns 0.
///
/// Original: 0x00b32e70 (cdecl, four stack words; one two-word callee; a
/// range-checked jump table).
lf_checker_rt::export!(cdecl, rw_00b32e70(first: u32, d: u32, second: u32, c: u32) -> u32 {
    unsafe {
        const TAG: u32 = 0x44;
        const COMPARE: u32 = 1;
        #[inline(always)]
        unsafe fn r32(o: u32, off: u32) -> u32 {
            unsafe { (o as *const u32).byte_add(off as usize).read_unaligned() }
        }
        let ordering: u32 =
            lf_checker_rt::callee_cdecl!(COMPARE, u32, r32(first, 0), r32(second, 0));
        if (ordering as i32) < 0 {
            return ordering & !0xFF | 1;
        }
        let x = r32(first, TAG);
        let y = r32(second, TAG);
        if x != y {
            if (c as i32) > (d as i32) {
                return x & !0xFF | 1;
            }
            return x & !0xFF;
        }
        let s = r32(first, 0).wrapping_sub(12);
        if s > 14 {
            return s & !0xFF;
        }
        match r32(first, 0) {
            12 | 22 | 23 | 26 => {
                if (c as i32) > (d as i32) {
                    1
                } else {
                    0
                }
            }
            _ => 0,
        }
    }
});
