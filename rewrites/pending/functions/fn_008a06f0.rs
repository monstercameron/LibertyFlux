// original: 0x008a06f0 audSound_compute_slot_and_valid_flag
/// Audio sound slot lookup with validity flag.
///
/// Computes a slot index from the sound's count and stride fields
/// (`(count - 1) * stride`, or zero when count is all-ones), then, unless the
/// selector byte is `0xFF`, resolves a scaled table entry through two game
/// globals and asks a helper (by `thiscall`) for an adjustment that is added
/// to the index unless it is all-ones. When `out` is non-null the low byte
/// there records whether the count was all-ones. Returns the slot index.
export!(thiscall, rw_008a06f0(this: *const u8, out: *mut u8) -> u32 {
    const COUNT_OFF: usize = 0xd0;
    const STRIDE_OFF: usize = 0xcc;
    const SELECTOR_OFF: usize = 0x48;
    const ROW_OFF: usize = 0x40;
    const ROW_STRIDE: u32 = 0x6f40;
    const ROW_BASE: u32 = 0x6f10;
    const TABLE_BASE_GLOB: u32 = 0x115d988;
    const SCALE_GLOB: u32 = 0x115d964;
    const NONE: u32 = 0xffff_ffff;
    unsafe {
        let count = *(this.add(COUNT_OFF) as *const u32);
        let stride = *(this.add(STRIDE_OFF) as *const u32);
        let mut index = if count == NONE {
            0
        } else {
            count.wrapping_sub(1).wrapping_mul(stride)
        };
        let selector = *this.add(SELECTOR_OFF);
        if selector != 0xff {
            let row = *this.add(ROW_OFF) as u32;
            let table = *global::<u32>(TABLE_BASE_GLOB);
            let entry = *(table
                .wrapping_add(row.wrapping_mul(ROW_STRIDE))
                .wrapping_add(ROW_BASE) as *const u32);
            let scale = *global::<u32>(SCALE_GLOB);
            let target = scale.wrapping_mul(selector as u32).wrapping_add(entry);
            if target != 0 {
                let answer = callee_thiscall!(1, u32, target, 0);
                if answer != NONE {
                    index = index.wrapping_add(answer);
                }
            }
        }
        if !out.is_null() {
            *out = (count == NONE) as u8;
        }
        index
    }
});
