// original: 0x008FAFC0 NativeImpl_IS_THIS_PRINT_BEING_DISPLAYED
/// Compare two 16-bit strings up to a word budget.
///
/// The lengths come from the length routine: unequal lengths pass
/// only when both reach the budget. An empty first string matches at
/// once; otherwise words are compared until the budget, a mismatch
/// (returns 0) or a first-string NUL (returns 1). Cdecl, three stack
/// arguments (first, second, max words), boolean in al.
export!(cdecl, rw_008fafc0(s0: u32, s1: u32, max: u32) -> u32 {
    unsafe {
        let l0: u32 = callee_cdecl!(1, u32, s0);
        let l1: u32 = callee_cdecl!(2, u32, s1);
        let bp = (l0 & 0xFFFF) as u16;
        let ax = (l1 & 0xFFFF) as u16;
        let dx = (max & 0xFFFF) as u16;
        if bp != ax {
            if bp < dx {
                return 0;
            }
            if ax < dx {
                return 0;
            }
        }
        if ((s0 as *const u16).read_unaligned() == 0) {
            return 1;
        }
        let mut si: u16 = 0;
        loop {
            if si >= dx {
                return 1;
            }
            let a = ((s0 + (si as u32) * 2) as *const u16).read_unaligned();
            let b = ((s1 + (si as u32) * 2) as *const u16).read_unaligned();
            if a != b {
                return 0;
            }
            si = si.wrapping_add(1);
            if (((s0 + (si as u32) * 2) as *const u16).read_unaligned() == 0) {
                return 1;
            }
        }
    }
});
