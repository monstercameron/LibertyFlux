// original: 0x00ae0e40 ui_reset_inline_pool
/// Release this object's inline pool entries and reset its cursor.
///
/// Counts the dwords between the inline start (0x34) and the cursor at
/// 0x74; a nonzero count is handed to the worker with (count, start, 1),
/// then the cursor is parked back at the start. Returns the start address.
export!(thiscall, rw_00ae0e40(this: *mut u32) -> u32 {
    unsafe {
        let base = this as u32;
        let cursor = *this.add(0x74 / 4);
        let count = (cursor.wrapping_sub(base).wrapping_sub(0x34) as i32 >> 2) as u32;
        let start = base.wrapping_add(0x34);
        if count != 0 {
            callee_cdecl!(1, u32, count, start, 1);
        }
        *this.add(0x74 / 4) = start;
        start
    }
});
