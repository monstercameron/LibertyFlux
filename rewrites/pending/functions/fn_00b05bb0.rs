// original: 0x00b05bb0 Script_Shared_CELL_CAM_SET_CENTRE_POS_CELL_CAM_SET_COLOUR_BRIGHTNESS_Etc_2

/// Scans rows for the first live one and returns its index, or -1.
///
/// A row counts when its flag byte is clear, its computed address is non-null,
/// and the byte at row+0xc is nonzero. An empty or negative count yields -1.
export!(thiscall, rw_00b05bb0(obj: *mut u8) -> u32 {
    unsafe {
        let count = *(obj.add(8) as *const i32);
        if count <= 0 {
            return 0xFFFF_FFFF;
        }
        let flags = *(obj.add(4) as *const u32);
        let base = *(obj as *const u32);
        let stride = *(obj.add(0x0c) as *const u32);
        let mut i: i32 = 0;
        loop {
            if *((flags.wrapping_add(i as u32)) as *const u8) & 0x80 == 0 {
                let row = base.wrapping_add(stride.wrapping_mul(i as u32));
                if row != 0 && *((row.wrapping_add(0x0c)) as *const u8) != 0 {
                    return i as u32;
                }
            }
            i += 1;
            if i >= count {
                return 0xFFFF_FFFF;
            }
        }
    }
});
