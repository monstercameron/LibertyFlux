// original: 0x00b05b80 lookup_row_field0

/// Flag-table lookup returning the first field of row `idx`.
///
/// Same shape as its sibling lookup, but reads the row start instead of +8.
export!(thiscall, rw_00b05b80(obj: *mut u8, idx: u32) -> u32 {
    unsafe {
        let flags = *(obj.add(4) as *const u32);
        if *((flags.wrapping_add(idx)) as *const u8) & 0x80 != 0 {
            return 0;
        }
        let base = *(obj as *const u32);
        let stride = *(obj.add(0x0c) as *const u32);
        let row = base.wrapping_add(stride.wrapping_mul(idx));
        if row == 0 {
            return 0;
        }
        *(row as *const u32)
    }
});
