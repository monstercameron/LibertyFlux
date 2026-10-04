// original: 0x00b05b50 Script_Shared_CELL_CAM_SET_CENTRE_POS_CELL_CAM_SET_COLOUR_BRIGHTNESS_Etc

/// Flag-table lookup returning the +8 field of row `idx`.
///
/// The object holds a row base (+0), a per-row flag table (+4), and a row
/// stride (+0xc). Returns 0 when the row is flagged (high bit set) or when
/// the computed row address is null.
export!(thiscall, rw_00b05b50(obj: *mut u8, idx: u32) -> u32 {
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
        *((row.wrapping_add(8)) as *const u32)
    }
});
