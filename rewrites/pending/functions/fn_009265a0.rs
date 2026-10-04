// original: 0x009265A0 input_cell_reset
/// Reset the input cell for index `a`: seven words at a strided address.
///
/// Index -1 does nothing. Otherwise writes 0x47800000 at the cell base,
/// -1 at three flag words, and 0 at three counter words, all at fixed
/// offsets from `0x011A0BE0 + (a << 8)`. Returns `a << 8`.
export!(cdecl, rw_009265A0(a: u32) -> u32 {
    unsafe {
        if a == 0xFFFF_FFFF {
            return 0xFFFF_FFFF;
        }
        let b = relocated(0x11A0BE0).wrapping_add(a.wrapping_shl(8));
        let w = |off: u32| b.wrapping_add(off) as *mut u32;
        *w(0x00) = 0x47800000;
        *w(0x0C) = 0;
        *w(0x10) = 0;
        *w(0x14) = 0xFFFF_FFFF;
        *w(0x18) = 0xFFFF_FFFF;
        *w(0x1C) = 0xFFFF_FFFF;
        *w(0x20) = 0;
        a.wrapping_shl(8)
    }
});
